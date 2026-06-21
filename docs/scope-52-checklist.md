> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 52 Checklist: Dup FD and CWD-Relative Open

## Scope

- [x] `DupFd` syscall; per-process `cwd` (default `/`).
- [x] Relative paths resolved under `cwd` for `OpenFile`.
- [x] Covered by validation gate `fd_mmap` (`ClanOS-Gate: name=fd_mmap ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate fd_mmap --timeout 180`

## Deferred

- [ ] `chdir` syscall; `..` normalization.
