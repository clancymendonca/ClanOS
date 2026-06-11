//! Hardware user page tables, CR3 activation, and per-process switching (Phases 21-22, 30).

use bootloader::bootinfo::MemoryMap;
use core::sync::atomic::{AtomicU64, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::{
    registers::control::Cr3,
    structures::paging::{
        page_table::PageTableEntry, FrameAllocator, Mapper, OffsetPageTable, Page, PageTable,
        PageTableFlags, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use crate::memory;

use crate::{
    frame_ownership::OwnedFrameToken,
    load_plan::LoadPermissions,
    user_memory::{InactiveUserPageTable, UserPageTableId},
};

static PHYS_MEM_OFFSET: AtomicU64 = AtomicU64::new(0);
static HW_BUILT: AtomicU64 = AtomicU64::new(0);
static HW_VERIFIED: AtomicU64 = AtomicU64::new(0);
static HW_REJECTED: AtomicU64 = AtomicU64::new(0);
static CR3_ACTIVATIONS: AtomicU64 = AtomicU64::new(0);
static CR3_RESTORES: AtomicU64 = AtomicU64::new(0);
static CR3_SWITCHES: AtomicU64 = AtomicU64::new(0);
static CR3_ISOLATION_CHECKS: AtomicU64 = AtomicU64::new(0);
static BRINGUP_USER_CR3: AtomicU64 = AtomicU64::new(0);
static SCHED_CR3_SWITCHES: AtomicU64 = AtomicU64::new(0);
static SCHED_CR3_SKIPS: AtomicU64 = AtomicU64::new(0);
static SCHED_CR3_BOUND: AtomicU64 = AtomicU64::new(0);
static SCHED_CR3_RESTORE_OK: AtomicU64 = AtomicU64::new(0);
static WX_CHECKED: AtomicU64 = AtomicU64::new(0);
static WX_REJECTED: AtomicU64 = AtomicU64::new(0);
static MPROTECT_APPLIED: AtomicU64 = AtomicU64::new(0);
static MPROTECT_REJECTED: AtomicU64 = AtomicU64::new(0);
static GUARD_FAULTS: AtomicU64 = AtomicU64::new(0);
static FORK_DUP_CR3: AtomicU64 = AtomicU64::new(0);
static FORK_COW_BREAKS: AtomicU64 = AtomicU64::new(0);
static FORK_COW_ISOLATED: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HwPageTableHandle {
    pub inactive_id: UserPageTableId,
    pub cr3_phys: u64,
    pub pml4_token: OwnedFrameToken,
    pub mapped_pages: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UserPagingError {
    NotInitialized,
    EmptyTable,
    FrameUnavailable,
    MapFailed,
    VerifyFailed,
    AlreadyActive,
}

struct KernelCr3Backup {
    frame: PhysFrame<Size4KiB>,
    flags: x86_64::registers::control::Cr3Flags,
}

lazy_static! {
    static ref KERNEL_CR3: Mutex<Option<KernelCr3Backup>> = Mutex::new(None);
    static ref ACTIVE_USER_CR3: Mutex<Option<u64>> = Mutex::new(None);
    static ref PAGE_TABLE_FRAME_ALLOCATOR: Mutex<Option<crate::memory::BootInfoFrameAllocator>> =
        Mutex::new(None);
}

pub unsafe fn set_boot_frame_allocator(memory_map: &'static MemoryMap, skip_frames: usize) {
    *PAGE_TABLE_FRAME_ALLOCATOR.lock() = Some(
        crate::memory::BootInfoFrameAllocator::init_from_index(memory_map, skip_frames),
    );
}

pub fn init(physical_memory_offset: VirtAddr) {
    PHYS_MEM_OFFSET.store(physical_memory_offset.as_u64(), Ordering::Relaxed);
}

pub fn phys_mem_offset() -> VirtAddr {
    VirtAddr::new(PHYS_MEM_OFFSET.load(Ordering::Relaxed))
}

pub fn phys_to_virt(phys: PhysAddr) -> VirtAddr {
    phys_mem_offset() + phys.as_u64()
}

pub fn status() -> (u64, u64, u64, u64, u64, u64, u64) {
    (
        HW_BUILT.load(Ordering::Relaxed),
        HW_VERIFIED.load(Ordering::Relaxed),
        HW_REJECTED.load(Ordering::Relaxed),
        CR3_ACTIVATIONS.load(Ordering::Relaxed),
        CR3_RESTORES.load(Ordering::Relaxed),
        CR3_SWITCHES.load(Ordering::Relaxed),
        CR3_ISOLATION_CHECKS.load(Ordering::Relaxed),
    )
}

pub fn sched_cr3_status() -> (u64, u64, u64, bool) {
    (
        SCHED_CR3_BOUND.load(Ordering::Relaxed),
        SCHED_CR3_SWITCHES.load(Ordering::Relaxed),
        SCHED_CR3_SKIPS.load(Ordering::Relaxed),
        SCHED_CR3_RESTORE_OK.load(Ordering::Relaxed) != 0,
    )
}

/// Activate the next context task's user CR3 during preemptive scheduling (Phase 31).
pub fn apply_scheduler_cr3_for_next(next_cr3: Option<u64>) {
    let _ = restore_kernel_page_table();
    match next_cr3 {
        Some(cr3) if cr3 != 0 => {
            if activate_for_process(cr3).is_ok() {
                SCHED_CR3_SWITCHES.fetch_add(1, Ordering::Relaxed);
            }
        }
        _ => {
            SCHED_CR3_SKIPS.fetch_add(1, Ordering::Relaxed);
        }
    }
}

pub fn record_sched_cr3_bound() {
    SCHED_CR3_BOUND.fetch_add(1, Ordering::Relaxed);
}

pub fn sched_cr3_switch_smoke(first: u64, second: u64) -> bool {
    x86_64::instructions::interrupts::without_interrupts(|| {
        apply_scheduler_cr3_for_next(Some(first));
        let t1 = verify_active_translation(0x400000);
        apply_scheduler_cr3_for_next(Some(second));
        let t2 = verify_active_translation(0x400000);
        let restore_ok = restore_kernel_page_table().is_ok();
        if restore_ok {
            SCHED_CR3_RESTORE_OK.store(1, Ordering::Relaxed);
        }
        first != second && t1.is_some() && t2.is_some() && restore_ok
    })
}

pub fn write_phys_bytes(phys: u64, offset: usize, bytes: &[u8]) {
    let addr = phys.saturating_add(offset as u64);
    let virt = phys_to_virt(PhysAddr::new(addr));
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), virt.as_mut_ptr(), bytes.len());
    }
}

pub fn build_hw_page_table(
    inactive: &InactiveUserPageTable,
) -> Result<HwPageTableHandle, UserPagingError> {
    if PHYS_MEM_OFFSET.load(Ordering::Relaxed) == 0 {
        return Err(UserPagingError::NotInitialized);
    }
    if inactive.mapped_pages == 0 {
        return Err(UserPagingError::EmptyTable);
    }

    let mut frame_alloc = OwnershipFrameAllocator::default();
    let pml4_phys = frame_alloc
        .allocate_frame()
        .ok_or(UserPagingError::FrameUnavailable)?
        .start_address()
        .as_u64();
    zero_page_table(pml4_phys);
    copy_kernel_pml4_entries(pml4_phys)?;

    let mut mapper = unsafe { mapper_for_phys(pml4_phys) };
    for mapping in &inactive.mappings {
        let page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(mapping.virtual_address));
        let frame = PhysFrame::from_start_address(PhysAddr::new(mapping.physical_address))
            .map_err(|_| UserPagingError::MapFailed)?;
        let flags = flags_for_permissions(mapping.permissions);
        unsafe {
            if mapper.translate_page(page).is_ok() {
                let (_frame, flush) = mapper.unmap(page).map_err(|_| UserPagingError::MapFailed)?;
                flush.flush();
            }
            mapper
                .map_to(page, frame, flags, &mut frame_alloc)
                .map_err(|_| UserPagingError::MapFailed)?
                .flush();
        }
    }

    for mapping in &inactive.mappings {
        let virt = VirtAddr::new(mapping.virtual_address);
        let hw = translate_hw(pml4_phys, virt).ok_or(UserPagingError::VerifyFailed)?;
        let desc = crate::user_memory::translate(inactive, mapping.virtual_address)
            .ok_or(UserPagingError::VerifyFailed)?;
        if hw.as_u64() != desc {
            return Err(UserPagingError::VerifyFailed);
        }
    }

    HW_BUILT.fetch_add(1, Ordering::Relaxed);
    HW_VERIFIED.fetch_add(1, Ordering::Relaxed);

    let mut mapped_pages = inactive.mapped_pages;
    map_default_user_stack(pml4_phys, &mut frame_alloc, &mut mapped_pages)?;

    Ok(HwPageTableHandle {
        inactive_id: inactive.id,
        cr3_phys: pml4_phys,
        pml4_token: OwnedFrameToken::from_raw(pml4_phys),
        mapped_pages,
    })
}

fn map_default_user_stack(
    pml4_phys: u64,
    frame_alloc: &mut OwnershipFrameAllocator,
    mapped_pages: &mut usize,
) -> Result<(), UserPagingError> {
    use crate::user_context::{DEFAULT_USER_STACK_SIZE, DEFAULT_USER_STACK_TOP};
    let mut mapper = unsafe { mapper_for_phys(pml4_phys) };
    let stack_bottom = DEFAULT_USER_STACK_TOP.saturating_sub(DEFAULT_USER_STACK_SIZE as u64);
    let mut addr = stack_bottom;
    while addr < DEFAULT_USER_STACK_TOP {
        let frame = frame_alloc
            .allocate_frame()
            .ok_or(UserPagingError::FrameUnavailable)?;
        let page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(addr));
        let phys = frame;
        let flags =
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
        unsafe {
            mapper
                .map_to(page, phys, flags, frame_alloc)
                .map_err(|_| UserPagingError::MapFailed)?
                .flush();
        }
        addr = addr.saturating_add(4096);
        *mapped_pages += 1;
    }
    Ok(())
}

pub fn activate_user_page_table(handle: &HwPageTableHandle) -> Result<(), UserPagingError> {
    x86_64::instructions::interrupts::without_interrupts(|| activate_user_page_table_inner(handle))
}

pub fn restore_kernel_page_table() -> Result<(), UserPagingError> {
    x86_64::instructions::interrupts::without_interrupts(|| restore_kernel_page_table_inner())
}

/// Activate `handle`, run `f` with interrupts disabled, then restore kernel CR3.
pub fn with_user_page_table<R>(
    handle: &HwPageTableHandle,
    f: impl FnOnce() -> R,
) -> Result<R, UserPagingError> {
    x86_64::instructions::interrupts::without_interrupts(|| {
        activate_user_page_table_inner(handle)?;
        let result = f();
        restore_kernel_page_table_inner()?;
        Ok(result)
    })
}

fn activate_user_page_table_inner(handle: &HwPageTableHandle) -> Result<(), UserPagingError> {
    backup_kernel_cr3()?;
    let frame = PhysFrame::from_start_address(PhysAddr::new(handle.cr3_phys))
        .map_err(|_| UserPagingError::MapFailed)?;
    unsafe {
        Cr3::write(frame, Cr3::read().1);
    }
    *ACTIVE_USER_CR3.lock() = Some(handle.cr3_phys);
    BRINGUP_USER_CR3.store(handle.cr3_phys, Ordering::Relaxed);
    CR3_ACTIVATIONS.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

pub fn activate_bringup_user_cr3() -> Result<(), UserPagingError> {
    let cr3 = BRINGUP_USER_CR3.load(Ordering::Relaxed);
    if cr3 == 0 {
        return Err(UserPagingError::NotInitialized);
    }
    activate_for_process(cr3)
}

fn restore_kernel_page_table_inner() -> Result<(), UserPagingError> {
    crate::task::process::set_current_process_id(None);
    let Some(backup) = KERNEL_CR3.lock().take() else {
        *ACTIVE_USER_CR3.lock() = None;
        return Ok(());
    };
    unsafe {
        Cr3::write(backup.frame, backup.flags);
    }
    *ACTIVE_USER_CR3.lock() = None;
    CR3_RESTORES.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

pub fn active_user_cr3() -> Option<u64> {
    *ACTIVE_USER_CR3.lock()
}

pub fn verify_active_translation(virtual_address: u64) -> Option<u64> {
    if (*ACTIVE_USER_CR3.lock()).is_none() {
        return None;
    }
    memory::translate_addr(VirtAddr::new(virtual_address), phys_mem_offset())
        .map(|addr| addr.as_u64())
}

pub fn activate_for_process(cr3_phys: u64) -> Result<(), UserPagingError> {
    x86_64::instructions::interrupts::without_interrupts(|| {
        if KERNEL_CR3.lock().is_none() {
            backup_kernel_cr3()?;
        }
        let frame = PhysFrame::from_start_address(PhysAddr::new(cr3_phys))
            .map_err(|_| UserPagingError::MapFailed)?;
        unsafe {
            Cr3::write(frame, Cr3::read().1);
        }
        *ACTIVE_USER_CR3.lock() = Some(cr3_phys);
        CR3_SWITCHES.fetch_add(1, Ordering::Relaxed);
        if let Some(pid) = crate::task::process::process_for_cr3(cr3_phys) {
            crate::task::process::set_current_process_id(Some(pid));
        }
        Ok(())
    })
}

pub fn switch_between_user_tables(first: u64, second: u64) -> Result<bool, UserPagingError> {
    x86_64::instructions::interrupts::without_interrupts(|| {
        activate_for_process(first)?;
        let first_trans = verify_active_translation(0x400000);
        activate_for_process(second)?;
        let second_trans = verify_active_translation(0x400000);
        restore_kernel_page_table_inner()?;
        CR3_ISOLATION_CHECKS.fetch_add(1, Ordering::Relaxed);
        Ok(first != second && first_trans.is_some() && second_trans.is_some())
    })
}

fn backup_kernel_cr3() -> Result<(), UserPagingError> {
    if KERNEL_CR3.lock().is_some() {
        return Ok(());
    }
    let (frame, flags) = Cr3::read();
    *KERNEL_CR3.lock() = Some(KernelCr3Backup { frame, flags });
    Ok(())
}

fn zero_page_table(phys: u64) {
    let virt = phys_to_virt(PhysAddr::new(phys));
    let table: &mut PageTable = unsafe { &mut *virt.as_mut_ptr() };
    for entry in table.iter_mut() {
        *entry = PageTableEntry::new();
    }
}

/// Share all present kernel PML4 entries so Ring 0 keeps working after CR3 switch.
fn copy_kernel_pml4_entries(pml4_phys: u64) -> Result<(), UserPagingError> {
    let offset = phys_mem_offset();
    let (kernel_frame, _) = Cr3::read();
    let kernel_virt = offset + kernel_frame.start_address().as_u64();
    let user_virt = offset + pml4_phys;
    let kernel_pml4: &PageTable = unsafe { &*(kernel_virt.as_ptr()) };
    let user_pml4: &mut PageTable = unsafe { &mut *(user_virt.as_mut_ptr()) };
    for index in 0..512 {
        if let Ok(frame) = kernel_pml4[index].frame() {
            user_pml4[index].set_frame(frame, kernel_pml4[index].flags());
        }
    }
    Ok(())
}

unsafe fn mapper_for_phys(pml4_phys: u64) -> OffsetPageTable<'static> {
    let virt = phys_to_virt(PhysAddr::new(pml4_phys));
    let table: &mut PageTable = &mut *virt.as_mut_ptr();
    OffsetPageTable::new(table, phys_mem_offset())
}

pub fn translate_hw_page(pml4_phys: u64, virtual_address: u64) -> Option<u64> {
    translate_hw(pml4_phys, VirtAddr::new(virtual_address)).map(|a| a.as_u64())
}

pub fn wx_status() -> (u64, u64) {
    (
        WX_CHECKED.load(Ordering::Relaxed),
        WX_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn validate_page_flags(flags: PageTableFlags) -> bool {
    WX_CHECKED.fetch_add(1, Ordering::Relaxed);
    let writable = flags.contains(PageTableFlags::WRITABLE);
    let executable = !flags.contains(PageTableFlags::NO_EXECUTE);
    if writable && executable {
        WX_REJECTED.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    true
}

/// Map a demand-zero user page in an active user address space (Phase 38).
pub fn map_shared_hw_page(
    child_cr3: u64,
    parent_cr3: u64,
    virtual_address: u64,
) -> Result<(), UserPagingError> {
    let phys = translate_hw_page(parent_cr3, virtual_address).ok_or(UserPagingError::MapFailed)?;
    let frame = PhysFrame::<Size4KiB>::from_start_address(PhysAddr::new(phys & !0xfff))
        .map_err(|_| UserPagingError::MapFailed)?;
    let mut frame_alloc = OwnershipFrameAllocator::default();
    let mut mapper = unsafe { mapper_for_phys(child_cr3) };
    let page = Page::<Size4KiB>::containing_address(VirtAddr::new(virtual_address));
    let flags = PageTableFlags::PRESENT
        | PageTableFlags::WRITABLE
        | PageTableFlags::USER_ACCESSIBLE
        | PageTableFlags::NO_EXECUTE;
    unsafe {
        if mapper.translate_page(page).is_ok() {
            let (_frame, flush) = mapper.unmap(page).map_err(|_| UserPagingError::MapFailed)?;
            flush.flush();
        }
        mapper
            .map_to(page, frame, flags, &mut frame_alloc)
            .map_err(|_| UserPagingError::MapFailed)?
            .flush();
    }
    Ok(())
}

pub fn map_demand_zero_page(cr3_phys: u64, virtual_address: u64) -> Result<(), UserPagingError> {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut frame_alloc = OwnershipFrameAllocator::default();
        let mut mapper = unsafe { mapper_for_phys(cr3_phys) };
        let page = Page::<Size4KiB>::containing_address(VirtAddr::new(virtual_address));
        let frame = if let Some(frame) = frame_alloc.allocate_frame() {
            frame
        } else {
            let owned = crate::frame_ownership::allocate_frame(
                crate::frame_ownership::FrameOwner::PageTable,
            )
            .map_err(|_| UserPagingError::FrameUnavailable)?;
            PhysFrame::from_start_address(PhysAddr::new(owned.start_address))
                .map_err(|_| UserPagingError::FrameUnavailable)?
        };
        let flags = PageTableFlags::PRESENT
            | PageTableFlags::WRITABLE
            | PageTableFlags::USER_ACCESSIBLE
            | PageTableFlags::NO_EXECUTE;
        if !validate_page_flags(flags) {
            return Err(UserPagingError::MapFailed);
        }
        unsafe {
            mapper
                .map_to(page, frame, flags, &mut frame_alloc)
                .map_err(|_| UserPagingError::MapFailed)?
                .flush();
        }
        crate::smp::request_tlb_shootdown();
        Ok(())
    })
}

static UNMAP_APPLIED: AtomicU64 = AtomicU64::new(0);
static UNMAP_REJECTED: AtomicU64 = AtomicU64::new(0);

pub fn unmap_status() -> (u64, u64) {
    (
        UNMAP_APPLIED.load(Ordering::Relaxed),
        UNMAP_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn is_munmap_allowed(virtual_address: u64) -> bool {
    let page = virtual_address & !0xfff;
    if page == crate::mmap::MMAP_ANON_BASE {
        return true;
    }
    if page == crate::demand_paging::FILE_DEMAND_BASE
        || page == crate::demand_paging::FILE_DEMAND_BASE + 0x1000
    {
        return true;
    }
    if page >= 0x400000 && page < crate::mmap::MMAP_ANON_BASE {
        return false;
    }
    page >= crate::mmap::MMAP_ANON_BASE && page < crate::mmap::MMAP_ANON_LIMIT
}

pub fn unmap_user_page(cr3_phys: u64, virtual_address: u64) -> Result<(), UserPagingError> {
    if !is_munmap_allowed(virtual_address) {
        UNMAP_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(UserPagingError::MapFailed);
    }
    x86_64::instructions::interrupts::without_interrupts(|| {
        let page_base = virtual_address & !0xfff;
        let mut mapper = unsafe { mapper_for_phys(cr3_phys) };
        let page = Page::<Size4KiB>::containing_address(VirtAddr::new(page_base));
        if mapper.translate_page(page).is_err() {
            UNMAP_REJECTED.fetch_add(1, Ordering::Relaxed);
            return Err(UserPagingError::MapFailed);
        }
        let (_frame, flush) = mapper.unmap(page).map_err(|_| UserPagingError::MapFailed)?;
        flush.flush();
        UNMAP_APPLIED.fetch_add(1, Ordering::Relaxed);
        crate::smp::request_tlb_shootdown();
        Ok(())
    })
}

pub fn phase48_smoke() -> bool {
    let bad = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
    let good = PageTableFlags::PRESENT
        | PageTableFlags::WRITABLE
        | PageTableFlags::USER_ACCESSIBLE
        | PageTableFlags::NO_EXECUTE;
    !validate_page_flags(bad) && validate_page_flags(good)
}

pub fn mprotect_status() -> (u64, u64, u64) {
    (
        MPROTECT_APPLIED.load(Ordering::Relaxed),
        MPROTECT_REJECTED.load(Ordering::Relaxed),
        GUARD_FAULTS.load(Ordering::Relaxed),
    )
}

pub fn stack_guard_address() -> u64 {
    use crate::user_context::{DEFAULT_USER_STACK_SIZE, DEFAULT_USER_STACK_TOP};
    let stack_bottom = DEFAULT_USER_STACK_TOP.saturating_sub(DEFAULT_USER_STACK_SIZE as u64);
    stack_bottom.saturating_sub(4096)
}

pub fn probe_stack_guard(pml4_phys: u64) -> bool {
    let guard = stack_guard_address();
    if translate_hw_page(pml4_phys, guard).is_none() {
        GUARD_FAULTS.fetch_add(1, Ordering::Relaxed);
        return true;
    }
    false
}

pub fn mprotect_page(
    cr3_phys: u64,
    virtual_address: u64,
    make_writable: bool,
) -> Result<(), UserPagingError> {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let page_base = virtual_address & !0xfff;
        let offset = phys_mem_offset();
        let frame = PhysFrame::from_start_address(PhysAddr::new(cr3_phys))
            .map_err(|_| UserPagingError::MapFailed)?;
        let indexes = [
            VirtAddr::new(page_base).p4_index(),
            VirtAddr::new(page_base).p3_index(),
            VirtAddr::new(page_base).p2_index(),
            VirtAddr::new(page_base).p1_index(),
        ];
        let mut current = frame;
        for &index in &indexes[..3] {
            let virt = offset + current.start_address().as_u64();
            let table: &PageTable = unsafe { &*(virt.as_ptr()) };
            current = table[index]
                .frame()
                .map_err(|_| UserPagingError::MapFailed)?;
        }
        let virt = offset + current.start_address().as_u64();
        let table: &mut PageTable = unsafe { &mut *(virt.as_mut_ptr()) };
        let entry = &mut table[indexes[3]];
        if !entry.flags().contains(PageTableFlags::PRESENT) {
            MPROTECT_REJECTED.fetch_add(1, Ordering::Relaxed);
            return Err(UserPagingError::MapFailed);
        }
        let executable = !entry.flags().contains(PageTableFlags::NO_EXECUTE);
        let mut new_flags = entry.flags();
        if make_writable {
            new_flags |= PageTableFlags::WRITABLE;
        } else {
            new_flags.remove(PageTableFlags::WRITABLE);
        }
        if executable && new_flags.contains(PageTableFlags::WRITABLE) {
            MPROTECT_REJECTED.fetch_add(1, Ordering::Relaxed);
            return Err(UserPagingError::MapFailed);
        }
        if !validate_page_flags(new_flags) {
            MPROTECT_REJECTED.fetch_add(1, Ordering::Relaxed);
            return Err(UserPagingError::MapFailed);
        }
        entry.set_flags(new_flags);
        crate::smp::flush_tlb_on_unmap();
        MPROTECT_APPLIED.fetch_add(1, Ordering::Relaxed);
        Ok(())
    })
}

pub fn mprotect_user_page(user_addr: u64, prot: u64) -> Result<(), ()> {
    let cr3 = active_user_cr3().ok_or(())?;
    let want_write = (prot & 2) != 0;
    let want_exec = (prot & 4) != 0;
    if want_exec && want_write {
        MPROTECT_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    mprotect_page(cr3, user_addr, want_write).map_err(|_| ())
}

pub fn phase53_smoke() -> bool {
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(
        crate::security::Credentials::shell_user(),
        "hello",
    )
    .ok() else {
        return false;
    };
    let guard_ok = probe_stack_guard(built.hw.cr3_phys);
    let ro_ok = mprotect_page(built.hw.cr3_phys, 0x401000, false).is_ok();
    let bad = mprotect_page(built.hw.cr3_phys, 0x400000, true).is_err();
    let (applied, _rejected, guard) = mprotect_status();
    guard_ok && ro_ok && bad && applied > 0 && guard > 0
}

pub fn fork_dup_status() -> u64 {
    FORK_DUP_CR3.load(Ordering::Relaxed)
}

pub fn note_fork_dup_child() {
    FORK_DUP_CR3.fetch_add(1, Ordering::Relaxed);
}

pub fn fork_cow_status() -> (u64, u64) {
    (
        FORK_COW_BREAKS.load(Ordering::Relaxed),
        FORK_COW_ISOLATED.load(Ordering::Relaxed),
    )
}

pub fn break_cow_page(parent_cr3: u64, child_cr3: u64, virtual_address: u64) -> Result<(), ()> {
    copy_anon_page_to_child(parent_cr3, child_cr3, virtual_address).map_err(|_| ())?;
    FORK_COW_BREAKS.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

pub fn record_fork_cow_isolated() {
    FORK_COW_ISOLATED.fetch_add(1, Ordering::Relaxed);
}

pub fn write_user_byte(cr3_phys: u64, virtual_address: u64, value: u8) -> Result<(), ()> {
    let phys = translate_hw_page(cr3_phys, virtual_address & !0xfff).ok_or(())?;
    let virt = phys_to_virt(PhysAddr::new(phys + (virtual_address & 0xfff)));
    unsafe {
        *virt.as_mut_ptr() = value;
    }
    Ok(())
}

pub fn read_user_byte(cr3_phys: u64, virtual_address: u64) -> Result<u8, ()> {
    let phys = translate_hw_page(cr3_phys, virtual_address & !0xfff).ok_or(())?;
    let virt = phys_to_virt(PhysAddr::new(phys + (virtual_address & 0xfff)));
    Ok(unsafe { *virt.as_ptr() })
}

pub fn copy_anon_page_to_child(
    parent_cr3: u64,
    child_cr3: u64,
    virtual_address: u64,
) -> Result<(), UserPagingError> {
    let page_base = virtual_address & !0xfff;
    let phys = translate_hw_page(parent_cr3, page_base).ok_or(UserPagingError::MapFailed)?;
    let parent_frame = PhysFrame::<Size4KiB>::from_start_address(PhysAddr::new(phys & !0xfff))
        .map_err(|_| UserPagingError::MapFailed)?;
    let mut frame_alloc = OwnershipFrameAllocator::default();
    let new_frame = if let Some(frame) = frame_alloc.allocate_frame() {
        frame
    } else {
        let owned =
            crate::frame_ownership::allocate_frame(crate::frame_ownership::FrameOwner::PageTable)
                .map_err(|_| UserPagingError::FrameUnavailable)?;
        PhysFrame::from_start_address(PhysAddr::new(owned.start_address))
            .map_err(|_| UserPagingError::MapFailed)?
    };
    let src = phys_to_virt(parent_frame.start_address());
    let dst = phys_to_virt(new_frame.start_address());
    unsafe {
        core::ptr::copy_nonoverlapping(
            src.as_ptr() as *const u8,
            dst.as_mut_ptr() as *mut u8,
            4096,
        );
    }
    let page = Page::<Size4KiB>::containing_address(VirtAddr::new(page_base));
    let flags = PageTableFlags::PRESENT
        | PageTableFlags::WRITABLE
        | PageTableFlags::USER_ACCESSIBLE
        | PageTableFlags::NO_EXECUTE;
    let mut mapper = unsafe { mapper_for_phys(child_cr3) };
    unsafe {
        if mapper.translate_page(page).is_ok() {
            let (_frame, flush) = mapper.unmap(page).map_err(|_| UserPagingError::MapFailed)?;
            flush.flush();
        }
        mapper
            .map_to(page, new_frame, flags, &mut frame_alloc)
            .map_err(|_| UserPagingError::MapFailed)?
            .flush();
    }
    Ok(())
}

pub fn fork_duplicate_cr3(parent_cr3: u64) -> Result<u64, UserPagingError> {
    if PHYS_MEM_OFFSET.load(Ordering::Relaxed) == 0 {
        return Err(UserPagingError::NotInitialized);
    }
    let mut frame_alloc = OwnershipFrameAllocator::default();
    let child_cr3 = if let Some(frame) = frame_alloc.allocate_frame() {
        frame.start_address().as_u64()
    } else {
        let owned =
            crate::frame_ownership::allocate_frame(crate::frame_ownership::FrameOwner::PageTable)
                .map_err(|_| UserPagingError::FrameUnavailable)?;
        owned.start_address
    };
    zero_page_table(child_cr3);
    copy_kernel_pml4_entries(child_cr3)?;
    let mut child_mapper = unsafe { mapper_for_phys(child_cr3) };
    let mut addr = 0x400000u64;
    while addr < 0x410000 {
        if let Some(phys) = translate_hw_page(parent_cr3, addr) {
            let page = Page::<Size4KiB>::containing_address(VirtAddr::new(addr));
            let parent_frame =
                PhysFrame::<Size4KiB>::from_start_address(PhysAddr::new(phys & !0xfff))
                    .map_err(|_| UserPagingError::MapFailed)?;
            let entry_flags = if addr >= 0x401000 {
                PageTableFlags::PRESENT
                    | PageTableFlags::USER_ACCESSIBLE
                    | PageTableFlags::NO_EXECUTE
            } else {
                PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE
            };
            unsafe {
                child_mapper
                    .map_to(page, parent_frame, entry_flags, &mut frame_alloc)
                    .map_err(|_| UserPagingError::MapFailed)?
                    .flush();
            }
        }
        addr = addr.saturating_add(0x1000);
    }
    use crate::user_context::{DEFAULT_USER_STACK_SIZE, DEFAULT_USER_STACK_TOP};
    let stack_bottom = DEFAULT_USER_STACK_TOP.saturating_sub(DEFAULT_USER_STACK_SIZE as u64);
    let mut addr = stack_bottom;
    while addr < DEFAULT_USER_STACK_TOP {
        if let Some(phys) = translate_hw_page(parent_cr3, addr) {
            let page = Page::<Size4KiB>::containing_address(VirtAddr::new(addr));
            let frame = PhysFrame::<Size4KiB>::from_start_address(PhysAddr::new(phys & !0xfff))
                .map_err(|_| UserPagingError::MapFailed)?;
            let flags = PageTableFlags::PRESENT
                | PageTableFlags::WRITABLE
                | PageTableFlags::USER_ACCESSIBLE;
            unsafe {
                child_mapper
                    .map_to(page, frame, flags, &mut frame_alloc)
                    .map_err(|_| UserPagingError::MapFailed)?
                    .flush();
            }
        }
        addr = addr.saturating_add(0x1000);
    }
    FORK_DUP_CR3.fetch_add(1, Ordering::Relaxed);
    Ok(child_cr3)
}

fn translate_hw(pml4_phys: u64, addr: VirtAddr) -> Option<PhysAddr> {
    let offset = phys_mem_offset();
    let frame = PhysFrame::from_start_address(PhysAddr::new(pml4_phys)).ok()?;
    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut current = frame;
    for &index in &table_indexes {
        let virt = offset + current.start_address().as_u64();
        let table: &PageTable = unsafe { &*(virt.as_ptr()) };
        let entry = &table[index];
        current = entry.frame().ok()?;
    }
    Some(current.start_address() + u64::from(addr.page_offset()))
}

fn flags_for_permissions(permissions: LoadPermissions) -> PageTableFlags {
    let mut flags = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;
    if permissions.writable() {
        flags |= PageTableFlags::WRITABLE;
    }
    if !permissions.executable() {
        flags |= PageTableFlags::NO_EXECUTE;
    }
    let _ = validate_page_flags(flags);
    flags
}

#[derive(Default)]
struct OwnershipFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for OwnershipFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        PAGE_TABLE_FRAME_ALLOCATOR
            .lock()
            .as_mut()
            .and_then(|allocator| allocator.allocate_frame())
    }
}
