> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 98 Checklist: AP Runnable Enqueue Stub

## Scope

- [x] `enqueue_ap_runnable()` on CPU1 when `CPU_COUNT > 1`.
- [x] `AP_RUNNABLE_ENQUEUED` counter; no AP scheduler loop or BSP `hlt`.
- [x] Covered by boot gate `smp_depth` (`AresOS-BootGate: name=smp_depth ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 98 --timeout 180

## Deferred

- [ ] ACPI MADT AP startup; runnable tasks executing on APs.
