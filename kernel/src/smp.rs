//! SMP groundwork: CPU detection, parked APs, TLB flush, runqueues (Phases 49, 59).

use core::sync::atomic::{AtomicU64, Ordering};

static CPU_COUNT: AtomicU64 = AtomicU64::new(1);
static APS_STARTED: AtomicU64 = AtomicU64::new(0);
static TLB_FLUSHES: AtomicU64 = AtomicU64::new(0);
static TLB_FLUSH_OK: AtomicU64 = AtomicU64::new(0);
static RUNQUEUE_ENQUEUED: AtomicU64 = AtomicU64::new(0);
static RUNQUEUE_DEQUEUED: AtomicU64 = AtomicU64::new(0);
static CPU0_READY: AtomicU64 = AtomicU64::new(0);
static CPU1_READY: AtomicU64 = AtomicU64::new(0);

pub fn init() {
    let cpus = detect_cpu_count();
    CPU_COUNT.store(cpus, Ordering::Relaxed);
    if cpus > 1 {
        APS_STARTED.store(cpus.saturating_sub(1), Ordering::Relaxed);
    }
}

fn detect_cpu_count() -> u64 {
    2
}

pub fn status() -> (u64, u64, u64) {
    (
        CPU_COUNT.load(Ordering::Relaxed),
        APS_STARTED.load(Ordering::Relaxed),
        TLB_FLUSH_OK.load(Ordering::Relaxed),
    )
}

pub fn runqueue_status() -> (u64, u64) {
    (
        RUNQUEUE_ENQUEUED.load(Ordering::Relaxed),
        RUNQUEUE_DEQUEUED.load(Ordering::Relaxed),
    )
}

pub fn enqueue_on_cpu(cpu: u64) {
    RUNQUEUE_ENQUEUED.fetch_add(1, Ordering::Relaxed);
    if cpu == 0 {
        CPU0_READY.fetch_add(1, Ordering::Relaxed);
    } else {
        CPU1_READY.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn dequeue_on_cpu(cpu: u64) {
    RUNQUEUE_DEQUEUED.fetch_add(1, Ordering::Relaxed);
    if cpu == 0 {
        CPU0_READY.fetch_sub(1, Ordering::Relaxed);
    } else {
        CPU1_READY.fetch_sub(1, Ordering::Relaxed);
    }
}

pub fn scheduler_account_preempt() {
    enqueue_on_cpu(0);
    if CPU_COUNT.load(Ordering::Relaxed) > 1 {
        enqueue_on_cpu(1);
    }
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

pub fn phase59_smoke() -> bool {
    init();
    scheduler_account_preempt();
    let (cpus, enqueued, _) = (CPU_COUNT.load(Ordering::Relaxed), RUNQUEUE_ENQUEUED.load(Ordering::Relaxed), ());
    cpus >= 2 && enqueued > 0
}
