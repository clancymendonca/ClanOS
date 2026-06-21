> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 86 Checklist: `ExecLite` + Close-on-Exec

## Scope

- [x] `ExecLite = 81` replaces process image from allowlisted name; sweeps `FD_CLOEXEC` fds.
- [x] Covered by validation gate `path_exec` (`ClanOS-Gate: name=path_exec ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate path_exec --timeout 180`

## Deferred

- [ ] Full `execve` argv/env and unsigned ELF load.
