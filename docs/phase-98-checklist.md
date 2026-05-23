# Phase 98 Checklist: AP Runnable Enqueue Stub

## Scope

- [x] `enqueue_ap_runnable()` on CPU1 when `CPU_COUNT > 1`.
- [x] `AP_RUNNABLE_ENQUEUED` counter; no AP scheduler loop or BSP `hlt`.
- [x] `Phase98-ApRunnable` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase98_ap_runnable_check.py --timeout 180`

## Deferred

- [ ] ACPI MADT AP startup; runnable tasks executing on APs.
