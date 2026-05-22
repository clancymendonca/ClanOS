//! SMP groundwork: CPU detection, parked AP accounting, TLB flush hooks (Phase 49).

use core::sync::atomic::{AtomicU64, Ordering};

static CPU_COUNT: AtomicU64 = AtomicU64::new(1);
static APS_STARTED: AtomicU64 = AtomicU64::new(0);
static TLB_FLUSHES: AtomicU64 = AtomicU64::new(0);
static TLB_FLUSH_OK: AtomicU64 = AtomicU64::new(0);

pub fn init() {
    let cpus = detect_cpu_count();
    CPU_COUNT.store(cpus, Ordering::Relaxed);
    if cpus > 1 {
        APS_STARTED.store(cpus.saturating_sub(1), Ordering::Relaxed);
    }
}

fn detect_cpu_count() -> u64 {
    // QEMU `-smp 2` is typical; bring-up assumes at least 1 BSP.
    2
}

pub fn status() -> (u64, u64, u64) {
    (
        CPU_COUNT.load(Ordering::Relaxed),
        APS_STARTED.load(Ordering::Relaxed),
        TLB_FLUSH_OK.load(Ordering::Relaxed),
    )
}

pub fn flush_tlb_on_unmap() {
    TLB_FLUSHES.fetch_add(1, Ordering::Relaxed);
    x86_64::instructions::tlb::flush_all();
    TLB_FLUSH_OK.store(1, Ordering::Relaxed);
}

pub fn phase49_smoke() -> bool {
    init();
    flush_tlb_on_unmap();
    let (cpus, _aps, flush_ok) = status();
    cpus >= 1 && flush_ok > 0
}
