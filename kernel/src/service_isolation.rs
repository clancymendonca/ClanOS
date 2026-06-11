//! Service crash isolation (phase 127) — FAULT_ESCALATION tier 2 restart path.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::kernel_object::{self, Generation, ObjectKind};
use crate::task::process::ProcessId;

static TIER2_RESTARTS: AtomicU64 = AtomicU64::new(0);
static GENERATION_BUMPS: AtomicU64 = AtomicU64::new(0);

pub fn tier2_restart_count() -> u64 {
    TIER2_RESTARTS.load(Ordering::Relaxed)
}

pub fn generation_bump_count() -> u64 {
    GENERATION_BUMPS.load(Ordering::Relaxed)
}

/// Tier-2 service restart: bump generation on service object, invalidate derived caps at checkpoint.
pub fn restart_service_tier2(pid: ProcessId, service_cap_slot: u32) -> bool {
    let Some(cap) = kernel_object::get_cap(pid, service_cap_slot) else {
        return false;
    };
    if cap.kind != ObjectKind::Service {
        return false;
    }
    if !kernel_object::bump_object_generation(cap.object_id) {
        return false;
    }
    GENERATION_BUMPS.fetch_add(1, Ordering::Relaxed);
    TIER2_RESTARTS.fetch_add(1, Ordering::Relaxed);
    true
}

pub fn phase127_service_isolation_smoke() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    crate::task::process::set_process_mode(pid, crate::task::process::ProcessMode::Native);
    let slot = crate::service_loader::bootstrap_root_caps(pid).ok();
    let oid = slot.and_then(|s| kernel_object::get_cap(pid, s).map(|c| c.object_id));
    let gen_before = oid.and_then(kernel_object::object_generation);
    let restarted = slot.map(|s| restart_service_tier2(pid, s)).unwrap_or(false);
    let gen_after = oid.and_then(kernel_object::object_generation);
    let invalidated = matches!(
        (gen_before, gen_after),
        (Some(Generation::INITIAL), Some(g)) if g.0 > Generation::INITIAL.0
    );
    crate::task::process::set_process_mode(pid, crate::task::process::ProcessMode::Compat);
    restarted && invalidated
}
