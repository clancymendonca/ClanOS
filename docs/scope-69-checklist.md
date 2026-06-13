> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 69 Checklist: AP Idle Trampoline Accounting

## Scope

- [x] AP idle tick counter when `cpus > 1`.
- [x] BSP still runs all scheduler work.
- [x] Covered by boot gate `vm_fork` (`ClanOS-BootGate: name=vm_fork ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate vm_fork --timeout 180`

## Deferred

- [ ] Real AP entry trampoline; runnable tasks on APs.
