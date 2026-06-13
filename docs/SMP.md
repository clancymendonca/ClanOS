# SMP Groundwork

Phase 49 adds multiprocessor detection and hooks for future TLB management without scheduling work on application processors yet.

## Behavior

`smp::init()` runs during `kernel::init()`:

- Reads CPU count from ACPI or CPUID bring-up probes
- Records application processors as parked (not started)
- Exposes TLB flush helpers for future cross-CPU invalidation

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

Phase 31 binds CR3 on preemptive context switch on the bootstrap processor only. Phase 49 does not migrate runnable tasks to APs.

## Validation

```bash
python scripts/gate/legacy.py --phase 49 --timeout 180
```

## Runqueue Skeleton (Phase 59)

Phase 59 increments per-CPU enqueue counters when the BSP scheduler preempts. Application processors remain parked; this is accounting only, not work migration.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## TLB Shootdown Accounting (Phase 68)

`smp::request_tlb_shootdown()` records shootdown requests and completions per online CPU, then flushes the local TLB on the BSP. Munmap and demand-map paths call this helper instead of flushing silently.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## AP Idle Accounting (Phase 69)

When more than one CPU is reported, `smp::init()` increments `ap_idle_ticks` to represent parked application processors in an idle state. This is accounting only; APs do not run the scheduler yet.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## IPI TLB Shootdown Stub (Phase 78)

`request_tlb_shootdown()` records logical IPI send/ack counters (`ipi_shootdown_sent`, `ipi_shootdown_acked`) for all online CPUs, then flushes the BSP TLB. Munmap (phase 73) calls this helper after PTE changes.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## AP Idle Trampoline (Phase 79)

`ap_idle_trampoline()` is the minimal AP entry stub: it accounts `AP_TRAMPOLINE_ENTERED` and bumps `ap_idle_ticks`. A real `hlt` loop is deferred until ACPI AP startup exists (halting on the BSP during `smp::init()` would freeze QEMU bring-up).

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## LAPIC IPI Send Stub (Phase 89)

`request_tlb_shootdown()` increments `LAPIC_IPI_SEND` for each logical IPI target (`cpus - 1`) in addition to the existing shootdown and ack counters. This is accounting only; no LAPIC ICR MMIO is programmed yet.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## Work-Stealing Stub (Phase 97)

When the BSP runqueue is empty (`CPU0_READY == 0`) but CPU1 has work, `try_work_steal()` increments `WORK_STEAL_ATTEMPTS` and logically dequeues from CPU1 (`WORK_STEALS`). This is accounting only; tasks are not migrated.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## AP Runnable Enqueue Stub (Phase 98)

`enqueue_ap_runnable()` enqueues on CPU1 when `CPU_COUNT > 1` and increments `AP_RUNNABLE_ENQUEUED`. Application processors remain parked; no AP scheduler loop or BSP `hlt` in `smp::init()`.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## LAPIC ICR Write Stub (Phase 99)

`lapic_icr_send_stub()` records an ICR-low write in `LAPIC_ICR_WRITES` using a discard slot instead of programming real LAPIC MMIO at `0xfee0_0300` (which can hang QEMU bring-up tests). `request_tlb_shootdown()` still only bumps `LAPIC_IPI_SEND` counters.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## Deferred

- LAPIC timer per CPU, IPI reschedule, and real runnable work on APs
- Real AP trampolines and ACPI MADT-driven AP startup
- Real LAPIC ICR low/high MMIO for inter-processor interrupts
- Full work-stealing runqueues with task migration
