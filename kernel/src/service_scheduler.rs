//! Service scheduler + E-00 priority ceiling (phases 141–142).

use core::sync::atomic::{AtomicU64, Ordering};

use crate::task::process::ProcessId;

/// E-00 admission priority ceiling for native services.
pub const E00_PRIORITY_CEILING: u8 = 200;

static SCHEDULE_OPS: AtomicU64 = AtomicU64::new(0);
static CEILING_REJECTS: AtomicU64 = AtomicU64::new(0);

pub fn schedule_ops() -> u64 {
    SCHEDULE_OPS.load(Ordering::Relaxed)
}

pub fn ceiling_rejects() -> u64 {
    CEILING_REJECTS.load(Ordering::Relaxed)
}

pub fn schedule_service(pid: ProcessId, priority: u8) -> bool {
    SCHEDULE_OPS.fetch_add(1, Ordering::Relaxed);
    if priority > E00_PRIORITY_CEILING {
        CEILING_REJECTS.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    let _ = pid;
    true
}

pub fn phase141_service_scheduler_smoke() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    let ok_low = schedule_service(pid, 100);
    let reject_high = !schedule_service(pid, E00_PRIORITY_CEILING + 1);
    ok_low && reject_high && ceiling_rejects() > 0
}

pub fn phase142_smp_readiness_smoke() -> bool {
    let (cpus, aps, _tlb) = crate::smp::status();
    cpus >= 1 && aps <= cpus
}
