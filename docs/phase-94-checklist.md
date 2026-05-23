# Phase 94 Checklist: `ExecLite` Argv from User

## Scope

- [x] `exec_lite_with_argv`: bounded argv strings from user pointer vector.
- [x] `EXEC_ARGV_OK` counter; `Process.exec_argv` metadata.
- [x] `Phase94-ExecArgv` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase94_exec_argv_check.py --timeout 180`

## Deferred

- [ ] Full `execve` envp; arbitrary unsigned ELF.
