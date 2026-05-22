# Phase 52 Checklist: Dup FD and CWD-Relative Open

## Scope

- [x] `DupFd` syscall; per-process `cwd` (default `/`).
- [x] Relative paths resolved under `cwd` for `OpenFile`.
- [x] Emit `Phase52-FdDup` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase52_fd_dup_check.py --timeout 180`

## Deferred

- [ ] `chdir` syscall; `..` normalization.
