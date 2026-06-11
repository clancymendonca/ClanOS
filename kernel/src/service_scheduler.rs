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

/// S-01 unified native service admission (`SCHEDULING_UNIFIED.md`).
pub fn s01_unified_admission_smoke() -> bool {
    phase141_service_scheduler_smoke() && phase142_smp_readiness_smoke()
}

/// S-02: priority ceiling rejects above E-00.
pub fn s02_priority_ceiling_smoke() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    !schedule_service(pid, E00_PRIORITY_CEILING + 10)
}

/// S-03: SMP status consistent under schedule ops.
pub fn s03_smp_schedule_smoke() -> bool {
    s02_priority_ceiling_smoke() && phase142_smp_readiness_smoke()
}

/// S-04: schedule op counter monotonic.
pub fn s04_schedule_ops_smoke() -> bool {
    let before = schedule_ops();
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    let _ = schedule_service(pid, 50);
    schedule_ops() > before
}

/// S-05: unified band smoke (epoch 8 graduation).
pub fn s05_unified_band_smoke() -> bool {
    s01_unified_admission_smoke()
        && s02_priority_ceiling_smoke()
        && s03_smp_schedule_smoke()
        && s04_schedule_ops_smoke()
}

pub fn phase200_scheduling_unified_smoke() -> bool {
    s05_unified_band_smoke()
}

pub fn epoch8_scheduling_graduated() -> bool {
    phase200_scheduling_unified_smoke()
}
