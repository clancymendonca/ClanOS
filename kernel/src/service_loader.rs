//! Phase 121 — service loader contract, resource stubs, and E-00 admission control.
//!
//! See docs/phase-121-checklist.md, docs/KERNEL_OBJECT_MODEL.md (bootstrap ceremony),
//! docs/ERROR_TAXONOMY.md.

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

use crate::kernel_object::{self, CapError, ObjectKind, Rights};
use crate::task::process::{self, ProcessId};

/// E-00 transient: admission queue saturated (caller may retry).
pub const E00_SATURATED: u32 = 0xE000;

/// Remediable structural: cap quota exceeded (release caps and retry).
pub const ERR_CAP_QUOTA: u32 = 0xE101;

/// System: memory budget exceeded (MEM_BUDGET_STUB).
pub const ERR_MEM_BUDGET: u32 = 0xE200;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    Transient,
    StructuralRemediable,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeError {
    pub code: u32,
    pub class: ErrorClass,
}

impl NativeError {
    pub const fn e00_saturated() -> Self {
        NativeError {
            code: E00_SATURATED,
            class: ErrorClass::Transient,
        }
    }

    pub const fn cap_quota() -> Self {
        NativeError {
            code: ERR_CAP_QUOTA,
            class: ErrorClass::StructuralRemediable,
        }
    }

    pub const fn mem_budget() -> Self {
        NativeError {
            code: ERR_MEM_BUDGET,
            class: ErrorClass::System,
        }
    }
}

/// MEM_BUDGET_STUB — contract before full enforcement (phase 147).
#[derive(Debug, Clone, Copy)]
pub struct MemBudgetStub {
    pub total_bytes: u64,
    pub fault_handler_reserved_bytes: u64,
}

impl MemBudgetStub {
    pub const fn qemu_default() -> Self {
        MemBudgetStub {
            total_bytes: 256 * 1024,
            fault_handler_reserved_bytes: 32 * 1024,
        }
    }

    pub fn allocatable(&self, used: u64) -> u64 {
        self.total_bytes
            .saturating_sub(self.fault_handler_reserved_bytes)
            .saturating_sub(used)
    }
}

/// CAP_QUOTA_STUB — per-service cap slot limit (includes endpoint creation).
pub const DEFAULT_SERVICE_CAP_QUOTA: usize = 8;

static MEM_BUDGET: MemBudgetStub = MemBudgetStub::qemu_default();
static MEM_USED: AtomicU64 = AtomicU64::new(0);
static E00_INFLIGHT: AtomicU32 = AtomicU32::new(0);
static E00_MAX_INFLIGHT: AtomicU32 = AtomicU32::new(4);
static BOOTSTRAP_ACTIVE: AtomicBool = AtomicBool::new(false);
static BOOTSTRAP_MINTS: AtomicU32 = AtomicU32::new(0);
static QUOTA_REJECTS: AtomicU64 = AtomicU64::new(0);
static E00_REJECTS: AtomicU64 = AtomicU64::new(0);
static BUDGET_REJECTS: AtomicU64 = AtomicU64::new(0);

pub fn mem_budget_status() -> (u64, u64, u64) {
    let used = MEM_USED.load(Ordering::Relaxed);
    (
        MEM_BUDGET.total_bytes,
        used,
        MEM_BUDGET.allocatable(used),
    )
}

pub fn stub_status() -> (u64, u64, u64, u32) {
    (
        QUOTA_REJECTS.load(Ordering::Relaxed),
        E00_REJECTS.load(Ordering::Relaxed),
        BUDGET_REJECTS.load(Ordering::Relaxed),
        BOOTSTRAP_MINTS.load(Ordering::Relaxed),
    )
}

pub fn cap_quota_for_process(pid: ProcessId) -> usize {
    if process::process_mode(pid) == process::ProcessMode::Native {
        DEFAULT_SERVICE_CAP_QUOTA
    } else {
        crate::kernel_object::MAX_CAPS
    }
}

pub fn check_cap_quota(pid: ProcessId) -> Result<(), NativeError> {
    let held = kernel_object::cap_count_for_process(pid);
    if held >= cap_quota_for_process(pid) {
        QUOTA_REJECTS.fetch_add(1, Ordering::Relaxed);
        return Err(NativeError::cap_quota());
    }
    Ok(())
}

pub fn check_mem_budget(need_bytes: u64) -> Result<(), NativeError> {
    let used = MEM_USED.load(Ordering::Relaxed);
    if MEM_BUDGET.allocatable(used) < need_bytes {
        BUDGET_REJECTS.fetch_add(1, Ordering::Relaxed);
        return Err(NativeError::mem_budget());
    }
    Ok(())
}

pub fn charge_mem_budget(bytes: u64) -> Result<(), NativeError> {
    check_mem_budget(bytes)?;
    MEM_USED.fetch_add(bytes, Ordering::Relaxed);
    Ok(())
}

pub fn release_mem_budget(bytes: u64) {
    MEM_USED.fetch_sub(bytes.min(MEM_USED.load(Ordering::Relaxed)), Ordering::Relaxed);
}

/// E-00 admission: bounded in-flight service load operations.
pub fn e00_try_admit() -> Result<u32, NativeError> {
    loop {
        let cur = E00_INFLIGHT.load(Ordering::Acquire);
        let max = E00_MAX_INFLIGHT.load(Ordering::Relaxed);
        if cur >= max {
            E00_REJECTS.fetch_add(1, Ordering::Relaxed);
            return Err(NativeError::e00_saturated());
        }
        if E00_INFLIGHT
            .compare_exchange(cur, cur + 1, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            return Ok(cur + 1);
        }
    }
}

pub fn e00_release_admission() {
    E00_INFLIGHT.fetch_sub(1, Ordering::AcqRel);
}

pub fn bootstrap_mint_allowed(pid: ProcessId) -> bool {
    BOOTSTRAP_ACTIVE.load(Ordering::Acquire)
        && process::process_mode(pid) == process::ProcessMode::Native
        && kernel_object::cap_count_for_process(pid) == 0
}

/// Bootstrap cap ceremony — only caps minted without prior cap authorization (PID-1 equivalent).
pub fn bootstrap_root_caps(pid: ProcessId) -> Result<u32, CapError> {
    process::set_process_mode(pid, process::ProcessMode::Native);
    BOOTSTRAP_ACTIVE.store(true, Ordering::Release);
    let _ticket = e00_try_admit().map_err(|_| CapError::InvalidArgument)?;

    check_mem_budget(4096).map_err(|_| CapError::InvalidArgument)?;

    let service_oid = kernel_object::register_object(ObjectKind::Service, Rights::all_for_smoke());
    let broker_oid =
        kernel_object::register_object(ObjectKind::BrokerSession, Rights::all_for_smoke());

    let cap0 = kernel_object::mint_cap_for_process(pid, service_oid, Rights::read_write())?;
    let _cap1 = kernel_object::mint_cap_for_process(pid, broker_oid, Rights(Rights::READ | Rights::DELEGATE))?;

    charge_mem_budget(4096).ok();
    BOOTSTRAP_MINTS.fetch_add(1, Ordering::Relaxed);
    BOOTSTRAP_ACTIVE.store(false, Ordering::Release);
    e00_release_admission();

    Ok(cap0)
}

pub fn load_service_with_stubs(pid: ProcessId, mem_need: u64) -> Result<u32, NativeError> {
    let _ticket = e00_try_admit()?;
    check_mem_budget(mem_need)?;
    check_cap_quota(pid)?;

    let oid = kernel_object::register_object(ObjectKind::Service, Rights::read_write());
    let slot = kernel_object::mint_cap_for_process(pid, oid, Rights::read_write())
        .map_err(|_| NativeError::cap_quota())?;

    charge_mem_budget(mem_need).ok();
    e00_release_admission();
    Ok(slot)
}

pub fn phase121_service_loader_smoke() -> bool {
    let Some(pid) = process::create_process_for_smoke("phase121-svc") else {
        return false;
    };

    // Bootstrap ceremony: zero-cap native process receives root mint caps.
    let bootstrap_slot = bootstrap_root_caps(pid).ok();
    let post_bootstrap_count = kernel_object::cap_count_for_process(pid);
    let bootstrap_ok = bootstrap_slot.is_some() && post_bootstrap_count >= 2;

    // E-00 saturation: fill admission slots then reject.
    E00_MAX_INFLIGHT.store(1, Ordering::Relaxed);
    let t1 = e00_try_admit().is_ok();
    let e00_fail = e00_try_admit().is_err();
    e00_release_admission();
    E00_MAX_INFLIGHT.store(4, Ordering::Relaxed);

    // Cap quota: fill to quota then remediable reject; release and retry.
    process::set_process_mode(pid, process::ProcessMode::Native);
    let quota = cap_quota_for_process(pid);
    let mut filled = kernel_object::cap_count_for_process(pid);
    while filled < quota {
        let oid = kernel_object::register_object(ObjectKind::Endpoint, Rights(Rights::READ));
        if kernel_object::mint_cap_for_process(pid, oid, Rights(Rights::READ)).is_err() {
            break;
        }
        filled = kernel_object::cap_count_for_process(pid);
    }
    let at_quota = kernel_object::cap_count_for_process(pid) >= quota;
    let quota_reject = check_cap_quota(pid).is_err();
    if let Some(slot) = (0..kernel_object::MAX_CAPS as u32).find(|s| {
        kernel_object::get_cap(pid, *s).is_some()
    }) {
        let _ = kernel_object::close_cap_for_process(pid, slot);
    }
    let retry_ok = check_cap_quota(pid).is_ok();

    // Mem budget system error.
    MEM_USED.store(MEM_BUDGET.total_bytes, Ordering::Relaxed);
    let budget_fail = check_mem_budget(1).is_err();
    MEM_USED.store(0, Ordering::Relaxed);

    process::set_process_mode(pid, process::ProcessMode::Compat);

    bootstrap_ok && t1 && e00_fail && at_quota && quota_reject && retry_ok && budget_fail
}
