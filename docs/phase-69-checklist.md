> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 69 Checklist: AP Idle Trampoline Accounting

## Scope

- [x] AP idle tick counter when `cpus > 1`.
- [x] BSP still runs all scheduler work.
- [x] Covered by boot gate `vm_fork` (`AresOS-BootGate: name=vm_fork ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 69 --timeout 180

## Deferred

- [ ] Real AP entry trampoline; runnable tasks on APs.
