> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 79 Checklist: AP Idle Trampoline Entry

## Scope

- [x] `ap_idle_trampoline` executes `hlt` and increments idle counters.
- [x] `AP_TRAMPOLINE_ENTERED` accounting on SMP init.
- [x] Covered by boot gate `syscall_ring3` (`AresOS-BootGate: name=syscall_ring3 ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 79 --timeout 180

## Deferred

- [ ] ACPI MADT-driven AP startup and per-AP runqueues.
