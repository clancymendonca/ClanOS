//! Demand-zero page faults for user stack growth (Phase 38).

use core::sync::atomic::{AtomicU64, Ordering};
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

static DEMAND_FAULTS: AtomicU64 = AtomicU64::new(0);
static DEMAND_MAPPED: AtomicU64 = AtomicU64::new(0);
static DEMAND_REJECTED: AtomicU64 = AtomicU64::new(0);

const USER_GROW_BASE: u64 = 0x401000;
const USER_GROW_LIMIT: u64 = 0x500000;

pub fn status() -> (u64, u64, u64) {
    (
        DEMAND_FAULTS.load(Ordering::Relaxed),
        DEMAND_MAPPED.load(Ordering::Relaxed),
        DEMAND_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn handle_page_fault(
    _stack_frame: &InterruptStackFrame,
    error_code: PageFaultErrorCode,
) -> bool {
    let fault_addr = x86_64::registers::control::Cr2::read().as_u64();
    let user_mode = error_code.contains(PageFaultErrorCode::USER_MODE);
    let not_present = !error_code.contains(PageFaultErrorCode::PROTECTION_VIOLATION);
    if !user_mode || !not_present {
        DEMAND_REJECTED.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    if fault_addr < USER_GROW_BASE || fault_addr >= USER_GROW_LIMIT {
        DEMAND_REJECTED.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    DEMAND_FAULTS.fetch_add(1, Ordering::Relaxed);
    let page_base = fault_addr & !0xfff;
    let cr3 = crate::user_paging::active_user_cr3().unwrap_or(0);
    if cr3 == 0 {
        DEMAND_REJECTED.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    match crate::user_paging::map_demand_zero_page(cr3, page_base) {
        Ok(()) => {
            DEMAND_MAPPED.fetch_add(1, Ordering::Relaxed);
            true
        }
        Err(_) => {
            DEMAND_REJECTED.fetch_add(1, Ordering::Relaxed);
            false
        }
    }
}

pub fn phase38_smoke(cr3_phys: u64) -> bool {
    let before = DEMAND_MAPPED.load(Ordering::Relaxed);
    if crate::user_paging::map_demand_zero_page(cr3_phys, USER_GROW_BASE).is_ok() {
        DEMAND_MAPPED.fetch_add(1, Ordering::Relaxed);
    }
    let mapped = DEMAND_MAPPED.load(Ordering::Relaxed) > before;
    mapped && crate::user_paging::translate_hw_page(cr3_phys, USER_GROW_BASE).is_some()
}
