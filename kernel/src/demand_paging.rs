//! Demand-zero page faults for user stack growth (Phase 38).

use core::sync::atomic::{AtomicU64, Ordering};
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

static DEMAND_FAULTS: AtomicU64 = AtomicU64::new(0);
static DEMAND_MAPPED: AtomicU64 = AtomicU64::new(0);
static DEMAND_REJECTED: AtomicU64 = AtomicU64::new(0);
static FILE_FAULTS: AtomicU64 = AtomicU64::new(0);
static FILE_PAGES_LOADED: AtomicU64 = AtomicU64::new(0);
static FILE_REJECTED: AtomicU64 = AtomicU64::new(0);

const USER_GROW_BASE: u64 = 0x401000;
const USER_GROW_LIMIT: u64 = 0x500000;
const FILE_DEMAND_BASE: u64 = 0x500000;
const FILE_DEMAND_LIMIT: u64 = 0x510000;

static FILE_BACKED_PATH: spin::Mutex<Option<&'static str>> = spin::Mutex::new(None);

pub fn status() -> (u64, u64, u64) {
    (
        DEMAND_FAULTS.load(Ordering::Relaxed),
        DEMAND_MAPPED.load(Ordering::Relaxed),
        DEMAND_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn file_status() -> (u64, u64, u64) {
    (
        FILE_FAULTS.load(Ordering::Relaxed),
        FILE_PAGES_LOADED.load(Ordering::Relaxed),
        FILE_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn register_file_backed_region(path: &'static str) {
    *FILE_BACKED_PATH.lock() = Some(path);
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
    if fault_addr >= FILE_DEMAND_BASE && fault_addr < FILE_DEMAND_LIMIT {
        let cr3 = crate::user_paging::active_user_cr3().unwrap_or(0);
        return handle_file_backed_fault(cr3, fault_addr);
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

fn handle_file_backed_fault(cr3: u64, fault_addr: u64) -> bool {
    FILE_FAULTS.fetch_add(1, Ordering::Relaxed);
    let path = match *FILE_BACKED_PATH.lock() {
        Some(path) => path,
        None => {
            FILE_REJECTED.fetch_add(1, Ordering::Relaxed);
            return false;
        }
    };
    let page_base = fault_addr & !0xfff;
    let page_index = ((page_base - FILE_DEMAND_BASE) / 0x1000) as usize;
    if cr3 == 0 {
        FILE_REJECTED.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    let contents = crate::storage::read_file(path).ok().flatten();
    let Some(contents) = contents else {
        FILE_REJECTED.fetch_add(1, Ordering::Relaxed);
        return false;
    };
    let bytes = contents.as_bytes();
    let offset = page_index.saturating_mul(4096);
    if offset >= bytes.len() {
        FILE_REJECTED.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    match crate::user_paging::map_demand_zero_page(cr3, page_base) {
        Ok(()) => {
            let chunk = &bytes[offset..core::cmp::min(offset + 4096, bytes.len())];
            if let Some(phys) = crate::user_paging::translate_hw_page(cr3, page_base) {
                crate::user_paging::write_phys_bytes(phys, 0, chunk);
                FILE_PAGES_LOADED.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        Err(_) => {}
    }
    FILE_REJECTED.fetch_add(1, Ordering::Relaxed);
    false
}

pub fn try_map_file_page(cr3_phys: u64, fault_addr: u64) -> bool {
    register_file_backed_region("/bin/hello");
    handle_file_backed_fault(cr3_phys, fault_addr)
}

pub fn phase47_smoke(cr3_phys: u64) -> bool {
    try_map_file_page(cr3_phys, FILE_DEMAND_BASE)
}

pub fn phase38_smoke(cr3_phys: u64) -> bool {
    let before = DEMAND_MAPPED.load(Ordering::Relaxed);
    if crate::user_paging::map_demand_zero_page(cr3_phys, USER_GROW_BASE).is_ok() {
        DEMAND_MAPPED.fetch_add(1, Ordering::Relaxed);
    }
    let mapped = DEMAND_MAPPED.load(Ordering::Relaxed) > before;
    mapped && crate::user_paging::translate_hw_page(cr3_phys, USER_GROW_BASE).is_some()
}
