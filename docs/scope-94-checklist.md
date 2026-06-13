> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 94 Checklist: `ExecLite` Argv from User

## Scope

- [x] `exec_lite_with_argv`: bounded argv strings from user pointer vector.
- [x] `EXEC_ARGV_OK` counter; `Process.exec_argv` metadata.
- [x] Covered by boot gate `smp_depth` (`ClanOS-BootGate: name=smp_depth ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate smp_depth --timeout 180`

## Deferred

- [ ] Full `execve` envp; arbitrary unsigned ELF.
