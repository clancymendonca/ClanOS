# Phase 76 Checklist: `fcntl` `F_SETFD` / Close-on-Exec

## Scope

- [x] `F_SETFD` sets per-FD flags (`FD_CLOEXEC`).
- [x] `F_GETFD` returns stored flags.
- [x] Emit `Phase76-Fcntl` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase76_fcntl_setfd_check.py --timeout 180`

## Deferred

- [ ] Close-on-exec applied during `execve` (not yet implemented).
