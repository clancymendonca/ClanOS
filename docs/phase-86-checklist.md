# Phase 86 Checklist: `ExecLite` + Close-on-Exec

## Scope

- [x] `ExecLite = 81` replaces process image from allowlisted name; sweeps `FD_CLOEXEC` fds.
- [x] `phase86_smoke` and `Phase86-ExecLite` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase86_exec_lite_check.py --timeout 180`

## Deferred

- [ ] Full `execve` argv/env and unsigned ELF load.
