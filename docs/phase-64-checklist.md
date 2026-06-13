> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 64 Checklist: Fork-Lite with FD Inheritance

## Scope

- [x] `ForkLite = 76` creates child with inherited `fds` and `cwd`.
- [x] No page-table clone (FD isolation smoke only).
- [x] Covered by boot gate `vm_fork` (`AresOS-BootGate: name=vm_fork ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 64 --timeout 180

## Deferred

- [ ] COW fork; wait on child PID.
