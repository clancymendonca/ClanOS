> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 76 Checklist: `fcntl` `F_SETFD` / Close-on-Exec

## Scope

- [x] `F_SETFD` sets per-FD flags (`FD_CLOEXEC`).
- [x] `F_GETFD` returns stored flags.
- [x] Covered by boot gate `syscall_ring3` (`AresOS-BootGate: name=syscall_ring3 ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 76 --timeout 180

## Deferred

- [ ] Close-on-exec applied during `execve` (not yet implemented).
