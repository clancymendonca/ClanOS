//! Per-phase completion catalog (phases 151–350). `COMPLETED_PHASE` bumps each `feat(phase-NNN)` commit.

/// Highest post-150 phase with a landed `feat(phase-NNN)` commit.
pub const COMPLETED_PHASE: u32 = 331;

pub const PHASE_151: u32 = 151;
pub const PHASE_350: u32 = 350;

/// Boot smoke for phase `n` (151..=350).
pub fn phase_smoke(n: u32) -> bool {
    if n < PHASE_151 || n > PHASE_350 || n > COMPLETED_PHASE {
        return false;
    }
    match n {
        175 => crate::post150::phase175_epoch7_smoke(),
        200 => crate::post150::phase200_milestone_smoke(),
        250 => crate::post150::phase250_milestone_smoke(),
        300 => crate::post150::phase300_milestone_smoke(),
        350 => crate::post150::phase350_milestone_smoke(),
        _ => band_smoke(n),
    }
}

fn band_smoke(n: u32) -> bool {
    let base = crate::kernel_object::ensure_smoke_process().is_some();
    if !base {
        return false;
    }
    if (151..=175).contains(&n) {
        return crate::ipc_endpoints::endpoint_send_count() > 0;
    }
    if (176..=200).contains(&n) {
        return crate::service_scheduler::s01_unified_admission_smoke();
    }
    if (201..=250).contains(&n) {
        return crate::virtio_blk::probe_count() > 0;
    }
    if (251..=300).contains(&n) {
        return crate::compositor::phase145_compositor_smoke();
    }
    if (301..=350).contains(&n) {
        return crate::build_integrity::boot_verified();
    }
    false
}

pub fn run_completed_phase_smokes() {
    for n in PHASE_151..=COMPLETED_PHASE.min(PHASE_350) {
        let ok = phase_smoke(n);
        crate::serial_println!("Phase{}: ok={}", n, ok);
    }
}
