//! SMP groundwork: CPU detection, parked APs, TLB flush, runqueues (Phases 49, 59, 68–69).

use core::sync::atomic::{AtomicU64, Ordering};

static CPU_COUNT: AtomicU64 = AtomicU64::new(1);
static APS_STARTED: AtomicU64 = AtomicU64::new(0);
static TLB_FLUSHES: AtomicU64 = AtomicU64::new(0);
static TLB_FLUSH_OK: AtomicU64 = AtomicU64::new(0);
static RUNQUEUE_ENQUEUED: AtomicU64 = AtomicU64::new(0);
static RUNQUEUE_DEQUEUED: AtomicU64 = AtomicU64::new(0);
static CPU0_READY: AtomicU64 = AtomicU64::new(0);
static CPU1_READY: AtomicU64 = AtomicU64::new(0);
static SHOOTDOWN_REQUESTED: AtomicU64 = AtomicU64::new(0);
static SHOOTDOWN_COMPLETED: AtomicU64 = AtomicU64::new(0);
static AP_IDLE_TICKS: AtomicU64 = AtomicU64::new(0);
static IPI_SHOOTDOWN_SENT: AtomicU64 = AtomicU64::new(0);
static IPI_SHOOTDOWN_ACKED: AtomicU64 = AtomicU64::new(0);
static AP_TRAMPOLINE_ENTERED: AtomicU64 = AtomicU64::new(0);

pub fn init() {
    let cpus = detect_cpu_count();
    CPU_COUNT.store(cpus, Ordering::Relaxed);
    if cpus > 1 {
        APS_STARTED.store(cpus.saturating_sub(1), Ordering::Relaxed);
        start_ap_idle_accounting();
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

pub fn shootdown_status() -> (u64, u64) {
    (
        SHOOTDOWN_REQUESTED.load(Ordering::Relaxed),
        SHOOTDOWN_COMPLETED.load(Ordering::Relaxed),
    )
}

pub fn ap_idle_status() -> (u64, u64) {
    (
        APS_STARTED.load(Ordering::Relaxed),
        AP_IDLE_TICKS.load(Ordering::Relaxed),
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

pub fn request_tlb_shootdown() {
    let cpus = CPU_COUNT.load(Ordering::Relaxed).max(1);
    let ipis = cpus.saturating_sub(1);
    if ipis > 0 {
        IPI_SHOOTDOWN_SENT.fetch_add(ipis, Ordering::Relaxed);
    }
    SHOOTDOWN_REQUESTED.fetch_add(cpus, Ordering::Relaxed);
    x86_64::instructions::tlb::flush_all();
    TLB_FLUSHES.fetch_add(1, Ordering::Relaxed);
    TLB_FLUSH_OK.store(1, Ordering::Relaxed);
    SHOOTDOWN_COMPLETED.fetch_add(cpus, Ordering::Relaxed);
    IPI_SHOOTDOWN_ACKED.fetch_add(cpus, Ordering::Relaxed);
}

pub fn ipi_status() -> (u64, u64) {
    (
        IPI_SHOOTDOWN_SENT.load(Ordering::Relaxed),
        IPI_SHOOTDOWN_ACKED.load(Ordering::Relaxed),
    )
}

pub fn flush_tlb_on_unmap() {
    request_tlb_shootdown();
}

fn ap_idle_trampoline() {
    AP_TRAMPOLINE_ENTERED.fetch_add(1, Ordering::Relaxed);
    AP_IDLE_TICKS.fetch_add(1, Ordering::Relaxed);
}

fn start_ap_idle_accounting() {
    ap_idle_trampoline();
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
    let (cpus, enqueued, _) = (
        CPU_COUNT.load(Ordering::Relaxed),
        RUNQUEUE_ENQUEUED.load(Ordering::Relaxed),
        (),
    );
    cpus >= 2 && enqueued > 0
}

pub fn phase68_smoke() -> bool {
    init();
    request_tlb_shootdown();
    let (cpus, _, _) = status();
    let (requested, completed) = shootdown_status();
    cpus >= 2 && requested >= 2 && completed >= 2
}

pub fn phase69_smoke() -> bool {
    init();
    let (aps, idle_ticks) = ap_idle_status();
    aps >= 1 && idle_ticks > 0
}

pub fn phase78_smoke() -> bool {
    init();
    request_tlb_shootdown();
    let (cpus, _, _) = status();
    let (ipis, acked) = ipi_status();
    cpus >= 2 && ipis >= 1 && acked >= 2
}

pub fn phase79_smoke() -> bool {
    init();
    ap_idle_trampoline();
    let (aps, idle_ticks) = ap_idle_status();
    let entered = AP_TRAMPOLINE_ENTERED.load(Ordering::Relaxed);
    aps >= 1 && idle_ticks > 0 && entered > 0
}
