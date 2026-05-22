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

## Deferred

- LAPIC timer per CPU, IPI reschedule, and runnable work on APs
- Full TLB shootdown on CR3 and page-table edits from any CPU
- ACPI MADT-driven AP startup
