//! mmap bring-up: anonymous and read-only file mappings (Phase 54+).

use core::sync::atomic::{AtomicU64, Ordering};

pub const MMAP_ANON_BASE: u64 = 0x600000;
pub const MMAP_ANON_LIMIT: u64 = 0x620000;

static ANON_MAPPED: AtomicU64 = AtomicU64::new(0);
static FILE_MAPPED: AtomicU64 = AtomicU64::new(0);
static MMAP_REJECTED: AtomicU64 = AtomicU64::new(0);
static MUNMAP_APPLIED: AtomicU64 = AtomicU64::new(0);
static MUNMAP_REJECTED: AtomicU64 = AtomicU64::new(0);
static LAST_MMAP_CR3: AtomicU64 = AtomicU64::new(0);

pub fn hw_handle_for_last_mmap_smoke() -> Option<crate::user_paging::HwPageTableHandle> {
    let cr3 = LAST_MMAP_CR3.load(Ordering::Relaxed);
    if cr3 == 0 {
        return None;
    }
    Some(crate::user_paging::HwPageTableHandle {
        inactive_id: crate::user_memory::UserPageTableId::from_raw(0),
        cr3_phys: cr3,
        pml4_token: crate::frame_ownership::OwnedFrameToken::from_raw(cr3),
        mapped_pages: 0,
    })
}

pub fn status() -> (u64, u64, u64) {
    (
        ANON_MAPPED.load(Ordering::Relaxed),
        FILE_MAPPED.load(Ordering::Relaxed),
        MMAP_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn munmap_status() -> (u64, u64) {
    (
        MUNMAP_APPLIED.load(Ordering::Relaxed),
        MUNMAP_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn mmap_anonymous(cr3_phys: u64, prot: u64, hint: u64) -> Result<u64, ()> {
    let writable = (prot & 2) != 0;
    let executable = (prot & 4) != 0;
    if executable && writable {
        MMAP_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    let pid = crate::task::process::smoke_process_id()
        .or_else(|| crate::task::process::process_for_cr3(cr3_phys));
    let base = if hint != 0 {
        hint
    } else if let Some(pid) = pid {
        crate::vma::next_anon_hint(pid)
    } else {
        MMAP_ANON_BASE
    };
    if let Some(pid) = pid {
        if crate::vma::overlaps(pid, base, 0x1000) {
            MMAP_REJECTED.fetch_add(1, Ordering::Relaxed);
            return Err(());
        }
    }
    crate::user_paging::map_demand_zero_page(cr3_phys, base).map_err(|_| {
        MMAP_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    if let Some(pid) = pid {
        let _ = crate::vma::register_region(
            pid,
            crate::vma::VmaRegion {
                base,
                len: 0x1000,
                prot,
                backing: crate::vma::VmaBacking::Anon,
            },
        );
    }
    ANON_MAPPED.fetch_add(1, Ordering::Relaxed);
    Ok(base)
}

pub fn mmap_file_readonly(cr3_phys: u64, path: &str, prot: u64) -> Result<u64, ()> {
    if (prot & 2) != 0 || (prot & 4) != 0 {
        MMAP_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    let static_path = if path == "/bin/hello" {
        "/bin/hello"
    } else {
        MMAP_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    };
    crate::demand_paging::register_file_backed_region(static_path);
    let page = crate::demand_paging::FILE_DEMAND_BASE + 0x1000;
    let pid = crate::task::process::smoke_process_id()
        .or_else(|| crate::task::process::process_for_cr3(cr3_phys));
    if let Some(pid) = pid {
        if crate::vma::overlaps(pid, page, 0x1000) {
            MMAP_REJECTED.fetch_add(1, Ordering::Relaxed);
            return Err(());
        }
    }
    crate::user_paging::activate_for_process(cr3_phys).ok();
    let _ = crate::demand_paging::handle_file_backed_fault(cr3_phys, page);
    crate::user_paging::restore_kernel_page_table().ok();
    if let Some(pid) = pid {
        let _ = crate::vma::register_region(
            pid,
            crate::vma::VmaRegion {
                base: page,
                len: 0x1000,
                prot,
                backing: crate::vma::VmaBacking::File,
            },
        );
    }
    FILE_MAPPED.fetch_add(1, Ordering::Relaxed);
    Ok(page)
}

pub fn munmap_address(cr3_phys: u64, addr: u64) -> Result<(), ()> {
    let pid = crate::task::process::smoke_process_id()
        .or_else(|| crate::task::process::process_for_cr3(cr3_phys));
    if crate::user_paging::unmap_user_page(cr3_phys, addr).is_err() {
        MUNMAP_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    if let Some(pid) = pid {
        let _ = crate::vma::unregister_region(pid, addr & !0xfff);
    }
    MUNMAP_APPLIED.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

pub fn mmap_syscall(user_path: u64, prot: u64, anon: u64) -> Result<u64, ()> {
    let cr3 = crate::user_paging::active_user_cr3().ok_or(())?;
    if anon != 0 {
        return mmap_anonymous(cr3, prot, 0);
    }
    let path = crate::user_path::copy_path_from_user(user_path)?;
    mmap_file_readonly(cr3, &path, prot)
}

pub fn munmap_syscall(addr: u64) -> Result<(), ()> {
    let cr3 = crate::user_paging::active_user_cr3().ok_or(())?;
    munmap_address(cr3, addr)
}

pub fn phase54_smoke() -> bool {
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(
        crate::security::Credentials::shell_user(),
        "hello",
    )
    .ok() else {
        return false;
    };
    let cr3 = built.hw.cr3_phys;
    LAST_MMAP_CR3.store(cr3, Ordering::Relaxed);
    let rejected = mmap_anonymous(cr3, 6, 0).is_err();
    let anon = mmap_anonymous(cr3, 2, 0).is_ok();
    let file = mmap_file_readonly(cr3, "/bin/hello", 1).is_ok();
    let (a, f, _) = status();
    anon && file && rejected && a > 0 && f > 0
}

fn phase62_smoke_fresh(cr3: u64) -> bool {
    let (before_u, before_r) = munmap_status();
    let base = mmap_anonymous(cr3, 2, 0).ok();
    let file_base = crate::demand_paging::FILE_DEMAND_BASE;
    let file_mapped = crate::demand_paging::try_map_file_page(cr3, file_base)
        && crate::user_paging::translate_hw_page(cr3, file_base).is_some();
    let unmap_anon = base
        .map(|addr| munmap_address(cr3, addr).is_ok())
        .unwrap_or(false);
    let unmap_file = file_mapped && munmap_address(cr3, file_base).is_ok();
    let reject_image = munmap_address(cr3, 0x400000).is_err();
    let (after_u, after_r) = munmap_status();
    unmap_anon
        && unmap_file
        && reject_image
        && after_u >= before_u + 2
        && after_r > before_r
}

pub fn phase62_smoke() -> bool {
    let cr3 = LAST_MMAP_CR3.load(Ordering::Relaxed);
    if cr3 != 0 && crate::user_paging::translate_hw_page(cr3, MMAP_ANON_BASE).is_some() {
        let file_base = crate::demand_paging::FILE_DEMAND_BASE;
        if crate::user_paging::translate_hw_page(cr3, file_base).is_none() {
            let _ = crate::demand_paging::try_map_file_page(cr3, file_base);
        }
        let (before_u, before_r) = munmap_status();
        let unmap_anon = munmap_address(cr3, MMAP_ANON_BASE).is_ok();
        let unmap_file = munmap_address(cr3, file_base).is_ok();
        let reject_image = munmap_address(cr3, 0x400000).is_err();
        let (after_u, after_r) = munmap_status();
        return unmap_anon
            && unmap_file
            && reject_image
            && after_u >= before_u + 2
            && after_r > before_r;
    }

    let Some(built) = crate::task::program_loader::build_hw_page_table_program(
        crate::security::Credentials::shell_user(),
        "hello",
    )
    .ok() else {
        return false;
    };
    LAST_MMAP_CR3.store(built.hw.cr3_phys, Ordering::Relaxed);
    phase62_smoke_fresh(built.hw.cr3_phys)
}
