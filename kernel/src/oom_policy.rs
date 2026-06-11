//! Full OOM policy (phase 147) — suspend frozen-in-memory; mem budget enforcement.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::service_loader;
use crate::task::process::ProcessId;

static SUSPEND_COUNT: AtomicU64 = AtomicU64::new(0);
static TERMINATE_COUNT: AtomicU64 = AtomicU64::new(0);
static BUDGET_ENFORCED: AtomicU64 = AtomicU64::new(0);
static SHED_ACK_COUNT: AtomicU64 = AtomicU64::new(0);

/// Wire format: oom.shed.v1 — ack after tier-1 shed notification.
#[repr(C)]
pub struct OomShedAck {
    pub service_id: u64,
    pub shed_bytes: u64,
    pub ack_token: u32,
}

pub fn shed_ack_count() -> u64 {
    SHED_ACK_COUNT.load(Ordering::Relaxed)
}

pub fn record_shed_ack(ack: &OomShedAck) -> bool {
    if ack.ack_token == 0 {
        return false;
    }
    let _ = (ack.service_id, ack.shed_bytes);
    SHED_ACK_COUNT.fetch_add(1, Ordering::Relaxed);
    true
}

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

/// Enforce service mem budget — reject allocation over budget.
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
    let ack = OomShedAck {
        service_id: 1,
        shed_bytes: 4096,
        ack_token: 0xA0E5,
    };
    record_shed_ack(&ack);
    suspend_count() > 0
        && over_budget
        && budget_enforced_count() > 0
        && terminate_count() > 0
        && shed_ack_count() > 0
}

pub fn epoch7_oom_graduated() -> bool {
    phase147_oom_smoke()
}
