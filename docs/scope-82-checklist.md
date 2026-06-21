> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 82 Checklist: `getcwd` Syscall

## Scope

- [x] `GetCwd = 79` copies normalized process cwd to user buffer.
- [x] Covered by validation gate `path_exec` (`ClanOS-Gate: name=path_exec ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate path_exec --timeout 180`

## Deferred

- [ ] Ring-3 HW probe-only getcwd ELF (optional).
