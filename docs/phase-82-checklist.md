> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 82 Checklist: `getcwd` Syscall

## Scope

- [x] `GetCwd = 79` copies normalized process cwd to user buffer.
- [x] Covered by boot gate `path_exec` (`AresOS-BootGate: name=path_exec ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 82 --timeout 180

## Deferred

- [ ] Ring-3 HW probe-only getcwd ELF (optional).
