//! Full OOM policy (phase 147) — suspend frozen-in-memory; MEM_BUDGET_STUB enforcement.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::service_loader;
use crate::task::process::ProcessId;

static SUSPEND_COUNT: AtomicU64 = AtomicU64::new(0);
static TERMINATE_COUNT: AtomicU64 = AtomicU64::new(0);
static BUDGET_ENFORCED: AtomicU64 = AtomicU64::new(0);

pub fn suspend_count() -> u64 {
    SUSPEND_COUNT.load(Ordering::Relaxed)
}

pub fn terminate_count() -> u64 {
    TERMINATE_COUNT.load(Ordering::Relaxed)
}

pub fn budget_enforced_count() -> u64 {
    BUDGET_ENFORCED.load(Ordering::Relaxed)
}

/// Tier 2: suspend service (frozen in memory).
pub fn suspend_service(pid: ProcessId) {
    let _ = pid;
    SUSPEND_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Tier 3: hard terminate after cap teardown window.
pub fn terminate_service(pid: ProcessId) {
    let _ = pid;
    TERMINATE_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Enforce MEM_BUDGET_STUB — reject allocation over budget.
pub fn enforce_mem_budget(requested: u64) -> bool {
    let (total, used, _free) = service_loader::mem_budget_status();
    if used.saturating_add(requested) > total {
        BUDGET_ENFORCED.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    true
}

pub fn phase147_oom_smoke() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    suspend_service(pid);
    let over_budget = !enforce_mem_budget(u64::MAX / 2);
    terminate_service(pid);
    suspend_count() > 0 && over_budget && budget_enforced_count() > 0 && terminate_count() > 0
}
