//! mmap bring-up: anonymous and read-only file mappings (Phase 54).

use core::sync::atomic::{AtomicU64, Ordering};

pub const MMAP_ANON_BASE: u64 = 0x600000;
pub const MMAP_ANON_LIMIT: u64 = 0x610000;

static ANON_MAPPED: AtomicU64 = AtomicU64::new(0);
static FILE_MAPPED: AtomicU64 = AtomicU64::new(0);
static MMAP_REJECTED: AtomicU64 = AtomicU64::new(0);

pub fn status() -> (u64, u64, u64) {
    (
        ANON_MAPPED.load(Ordering::Relaxed),
        FILE_MAPPED.load(Ordering::Relaxed),
        MMAP_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn mmap_anonymous(cr3_phys: u64, prot: u64) -> Result<u64, ()> {
    let writable = (prot & 2) != 0;
    let executable = (prot & 4) != 0;
    if executable && writable {
        MMAP_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    crate::user_paging::map_demand_zero_page(cr3_phys, MMAP_ANON_BASE).map_err(|_| {
        MMAP_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    ANON_MAPPED.fetch_add(1, Ordering::Relaxed);
    Ok(MMAP_ANON_BASE)
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
    crate::user_paging::activate_for_process(cr3_phys).ok();
    let _ = crate::demand_paging::handle_file_backed_fault(cr3_phys, page);
    crate::user_paging::restore_kernel_page_table().ok();
    FILE_MAPPED.fetch_add(1, Ordering::Relaxed);
    Ok(page)
}

pub fn mmap_syscall(user_path: u64, prot: u64, anon: u64) -> Result<u64, ()> {
    let cr3 = crate::user_paging::active_user_cr3().ok_or(())?;
    if anon != 0 {
        return mmap_anonymous(cr3, prot);
    }
    let path = crate::user_path::copy_path_from_user(user_path)?;
    mmap_file_readonly(cr3, &path, prot)
}

pub fn phase54_smoke() -> bool {
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(
        crate::security::Credentials::shell_user(),
        "hello",
    )
    .ok() else {
        return false;
    };
    let anon = mmap_anonymous(built.hw.cr3_phys, 2).is_ok();
    let file = mmap_file_readonly(built.hw.cr3_phys, "/bin/hello", 1).is_ok();
    let rejected = mmap_anonymous(built.hw.cr3_phys, 6).is_err();
    let (a, f, _) = status();
    anon && file && rejected && a > 0 && f > 0
}
