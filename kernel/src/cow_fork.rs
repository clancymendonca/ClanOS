//! Copy-on-write fork: shared read-only mappings broken on write `#PF`.

use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicU64, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;

static COW_PF_BREAKS: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    static ref COW_FRAME_REFS: Mutex<BTreeMap<u64, u32>> = Mutex::new(BTreeMap::new());
}

pub fn pf_break_status() -> u64 {
    COW_PF_BREAKS.load(Ordering::Relaxed)
}

pub fn is_page_cow_shared(cr3: u64, virtual_address: u64) -> bool {
    let page_base = virtual_address & !0xfff;
    let Some(phys) = crate::user_paging::translate_hw_page(cr3, page_base) else {
        return false;
    };
    COW_FRAME_REFS
        .lock()
        .get(&(phys & !0xfff))
        .copied()
        .unwrap_or(0)
        > 0
}

/// After `fork_duplicate_cr3`, mark duplicated writable user pages shared + read-only.
pub fn share_after_fork(parent_cr3: u64, child_cr3: u64, start: u64, end: u64) {
    let mut addr = start;
    while addr < end {
        if crate::user_paging::page_is_writable(parent_cr3, addr) {
            if let Some(phys) = crate::user_paging::translate_hw_page(parent_cr3, addr) {
                let phys_base = phys & !0xfff;
                let mut refs = COW_FRAME_REFS.lock();
                *refs.entry(phys_base).or_insert(0) += 2;
                drop(refs);
                let _ = crate::user_paging::mprotect_page(parent_cr3, addr, false);
                let _ = crate::user_paging::mprotect_page(child_cr3, addr, false);
            }
        }
        addr = addr.saturating_add(0x1000);
    }
}

fn note_privatized(old_phys: u64) {
    let mut refs = COW_FRAME_REFS.lock();
    if let Some(count) = refs.get_mut(&(old_phys & !0xfff)) {
        *count = count.saturating_sub(1);
        if *count == 0 {
            refs.remove(&(old_phys & !0xfff));
        }
    }
}

/// Handle user write fault on a CoW-shared page (returns true when resolved).
pub fn try_break_on_write(cr3: u64, fault_addr: u64) -> bool {
    let page_base = fault_addr & !0xfff;
    let Some(phys) = crate::user_paging::translate_hw_page(cr3, page_base) else {
        return false;
    };
    let phys_base = phys & !0xfff;
    if COW_FRAME_REFS.lock().get(&phys_base).copied().unwrap_or(0) == 0 {
        return false;
    }
    if crate::user_paging::privatize_cow_page(cr3, page_base).is_err() {
        return false;
    }
    note_privatized(phys_base);
    COW_PF_BREAKS.fetch_add(1, Ordering::Relaxed);
    crate::user_paging::record_fork_cow_break();
    true
}

/// Fork-lite smoke: PF-driven CoW break isolates parent/child anonymous pages.
pub fn smoke_cow_fork() -> bool {
    let _ = crate::task::process::reap_terminated_processes();
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(parent) =
        crate::task::process::create_kernel_process_as("cow-fork-parent", tick, creds)
    else {
        return false;
    };
    let Some(built) =
        crate::task::program_loader::build_hw_page_table_program(creds, "hello").ok()
    else {
        return false;
    };
    let parent_cr3 = built.hw.cr3_phys;
    if !crate::task::process::set_process_cr3(parent, parent_cr3) {
        return false;
    }
    let Some(child) = crate::task::process::fork_lite(parent, tick.saturating_add(1)) else {
        return false;
    };
    let Some(child_cr3) =
        crate::task::process::with_process_mut(child, |p| p.cr3_phys()).flatten()
    else {
        return false;
    };
    let anon_va = crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(0x1000);
    if !is_page_cow_shared(parent_cr3, anon_va) {
        return false;
    }
    if !try_break_on_write(child_cr3, anon_va) {
        return false;
    }
    let _ = crate::user_paging::write_user_byte(child_cr3, anon_va, 0xBB);
    if !try_break_on_write(parent_cr3, anon_va) {
        return false;
    }
    let _ = crate::user_paging::write_user_byte(parent_cr3, anon_va, 0xAA);
    let parent_byte = crate::user_paging::read_user_byte(parent_cr3, anon_va).ok();
    let child_byte = crate::user_paging::read_user_byte(child_cr3, anon_va).ok();
    let isolated = parent_byte == Some(0xAA) && child_byte == Some(0xBB);
    if isolated {
        crate::user_paging::record_fork_cow_isolated();
    }
    let (breaks, isolated_n) = crate::user_paging::fork_cow_status();
    isolated && breaks > 0 && isolated_n > 0 && pf_break_status() >= 2
}
