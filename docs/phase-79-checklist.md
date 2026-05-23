# Phase 79 Checklist: AP Idle Trampoline Entry

## Scope

- [x] `ap_idle_trampoline` executes `hlt` and increments idle counters.
- [x] `AP_TRAMPOLINE_ENTERED` accounting on SMP init.
- [x] Emit `Phase79-ApTrampoline` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase79_ap_trampoline_check.py --timeout 180`

## Deferred

- [ ] ACPI MADT-driven AP startup and per-AP runqueues.
