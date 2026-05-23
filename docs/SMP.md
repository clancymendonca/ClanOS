# SMP Groundwork

Phase 49 adds multiprocessor detection and hooks for future TLB management without scheduling work on application processors yet.

## Behavior

`smp::init()` runs during `kernel::init()`:

- Reads CPU count from ACPI or CPUID bring-up probes
- Records application processors as parked (not started)
- Exposes TLB flush helpers for future cross-CPU invalidation

Boot smoke:

```text
Phase49-Smp: cpus=..., aps=..., parked=..., tlb_hooks=true
```

Phase 31 binds CR3 on preemptive context switch on the bootstrap processor only. Phase 49 does not migrate runnable tasks to APs.

## Validation

```bash
python scripts/phase49_smp_check.py --timeout 180
```

## Runqueue Skeleton (Phase 59)

Phase 59 increments per-CPU enqueue counters when the BSP scheduler preempts. Application processors remain parked; this is accounting only, not work migration.

Boot smoke:

```text
Phase59-Runqueues: cpus=..., enqueued=..., ok=true
```

## TLB Shootdown Accounting (Phase 68)

`smp::request_tlb_shootdown()` records shootdown requests and completions per online CPU, then flushes the local TLB on the BSP. Munmap and demand-map paths call this helper instead of flushing silently.

Boot smoke:

```text
Phase68-TlbShootdown: cpus=..., shootdowns=..., ok=true
```

## AP Idle Accounting (Phase 69)

When more than one CPU is reported, `smp::init()` increments `ap_idle_ticks` to represent parked application processors in an idle state. This is accounting only; APs do not run the scheduler yet.

Boot smoke:

```text
Phase69-ApIdle: aps=..., idle_ticks=..., ok=true
```

## Deferred

- LAPIC timer per CPU, IPI reschedule, and runnable work on APs
- Real AP trampolines and ACPI MADT-driven AP startup
- Work-stealing runqueues
