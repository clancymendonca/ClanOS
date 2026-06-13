> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 55 Checklist: User Write Path

## Scope

- [x] `WritePathProbe` syscall for `/tmp/*` paths.
- [x] Storage round-trip smoke.
- [x] Covered by boot gate `fd_mmap` (`AresOS-BootGate: name=fd_mmap ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 55 --timeout 180

## Deferred

- [ ] HW syscall smoke from Ring 3; writes outside `/tmp`.
