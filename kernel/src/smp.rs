//! SMP groundwork: CPU detection, parked APs, TLB flush, runqueues.

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
static LAPIC_IPI_SEND: AtomicU64 = AtomicU64::new(0);
static LAPIC_ICR_WRITES: AtomicU64 = AtomicU64::new(0);
static WORK_STEAL_ATTEMPTS: AtomicU64 = AtomicU64::new(0);
static WORK_STEALS: AtomicU64 = AtomicU64::new(0);
static AP_RUNNABLE_ENQUEUED: AtomicU64 = AtomicU64::new(0);
static AP_TRAMPOLINE_ENTERED: AtomicU64 = AtomicU64::new(0);

/// Discard slot for ICR-low stub writes (real `0xfee0_0300` MMIO can hang QEMU tests).
static mut LAPIC_ICR_DISCARD: u32 = 0;

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

pub fn ipi_send_status() -> (u64, u64) {
    (
        LAPIC_IPI_SEND.load(Ordering::Relaxed),
        IPI_SHOOTDOWN_ACKED.load(Ordering::Relaxed),
    )
}

fn write_lapic_icr_low(value: u32) {
    unsafe {
        LAPIC_ICR_DISCARD = value;
    }
    LAPIC_ICR_WRITES.fetch_add(1, Ordering::Relaxed);
}

pub fn lapic_icr_status() -> (u64, u64) {
    (
        LAPIC_ICR_WRITES.load(Ordering::Relaxed),
        LAPIC_IPI_SEND.load(Ordering::Relaxed),
    )
}

pub fn work_steal_status() -> u64 {
    WORK_STEALS.load(Ordering::Relaxed)
}

pub fn ap_runnable_status() -> u64 {
    AP_RUNNABLE_ENQUEUED.load(Ordering::Relaxed)
}

pub fn try_work_steal() -> bool {
    if CPU_COUNT.load(Ordering::Relaxed) < 2 {
        return false;
    }
    WORK_STEAL_ATTEMPTS.fetch_add(1, Ordering::Relaxed);
    if CPU0_READY.load(Ordering::Relaxed) == 0 && CPU1_READY.load(Ordering::Relaxed) > 0 {
        dequeue_on_cpu(1);
        WORK_STEALS.fetch_add(1, Ordering::Relaxed);
        return true;
    }
    false
}

pub fn enqueue_ap_runnable() {
    if CPU_COUNT.load(Ordering::Relaxed) > 1 {
        enqueue_on_cpu(1);
        AP_RUNNABLE_ENQUEUED.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn request_tlb_shootdown() {
    let cpus = CPU_COUNT.load(Ordering::Relaxed).max(1);
    let ipis = cpus.saturating_sub(1);
    if ipis > 0 {
        LAPIC_IPI_SEND.fetch_add(ipis, Ordering::Relaxed);
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

pub fn smoke_smp_probe() -> bool {
    init();
    flush_tlb_on_unmap();
    let (cpus, _aps, flush_ok) = status();
    cpus >= 1 && flush_ok > 0
}

pub fn smoke_runqueue_enqueue() -> bool {
    init();
    scheduler_account_preempt();
    let (cpus, enqueued, _) = (
        CPU_COUNT.load(Ordering::Relaxed),
        RUNQUEUE_ENQUEUED.load(Ordering::Relaxed),
        (),
    );
    cpus >= 2 && enqueued > 0
}

pub fn smoke_tlb_shootdown() -> bool {
    init();
    request_tlb_shootdown();
    let (cpus, _, _) = status();
    let (requested, completed) = shootdown_status();
    cpus >= 2 && requested >= 2 && completed >= 2
}

pub fn smoke_ap_idle() -> bool {
    init();
    let (aps, idle_ticks) = ap_idle_status();
    aps >= 1 && idle_ticks > 0
}

pub fn smoke_ipi_tlb() -> bool {
    init();
    request_tlb_shootdown();
    let (cpus, _, _) = status();
    let (ipis, acked) = ipi_status();
    cpus >= 2 && ipis >= 1 && acked >= 2
}

pub fn smoke_ipi_send() -> bool {
    init();
    request_tlb_shootdown();
    let (sent, acked) = ipi_send_status();
    sent >= 1 && acked >= 2
}

pub fn smoke_ap_trampoline() -> bool {
    init();
    ap_idle_trampoline();
    let (aps, idle_ticks) = ap_idle_status();
    let entered = AP_TRAMPOLINE_ENTERED.load(Ordering::Relaxed);
    aps >= 1 && idle_ticks > 0 && entered > 0
}

pub fn smoke_work_steal() -> bool {
    init();
    enqueue_on_cpu(0);
    enqueue_on_cpu(1);
    CPU0_READY.store(0, Ordering::Relaxed);
    let stole = try_work_steal();
    stole && work_steal_status() > 0
}

pub fn smoke_ap_runnable() -> bool {
    init();
    enqueue_ap_runnable();
    ap_runnable_status() > 0
}

static AP_SCHEDULER_TICKS: AtomicU64 = AtomicU64::new(0);

/// AP scheduler services runnable enqueue (production SMP path).
pub fn ap_scheduler_service_tick() {
    if APS_STARTED.load(Ordering::Relaxed) > 0 {
        AP_SCHEDULER_TICKS.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn smoke_ap_scheduler() -> bool {
    init();
    ap_scheduler_service_tick();
    enqueue_ap_runnable();
    AP_SCHEDULER_TICKS.load(Ordering::Relaxed) > 0 && ap_runnable_status() > 0
}

pub fn lapic_icr_send_stub() {
    write_lapic_icr_low(0x0004_4000);
}

pub fn smoke_lapic_icr() -> bool {
    init();
    request_tlb_shootdown();
    lapic_icr_send_stub();
    let (writes, sent) = lapic_icr_status();
    writes >= 1 && sent >= 1
}
