> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 44 Checklist: User Path Copyin

## Scope

- [x] `ReadPathProbe` syscall with bounded path validation.
- [x] Covered by boot gate `dynamic_runtime` (`AresOS-BootGate: name=dynamic_runtime ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 44 --timeout 180

## Deferred

- [ ] Write paths from user space; symlink following.
