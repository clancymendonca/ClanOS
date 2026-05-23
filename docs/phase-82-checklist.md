# Phase 82 Checklist: `getcwd` Syscall

## Scope

- [x] `GetCwd = 79` copies normalized process cwd to user buffer.
- [x] `phase82_smoke` and `Phase82-Getcwd` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase82_getcwd_check.py --timeout 180`

## Deferred

- [ ] Ring-3 HW probe-only getcwd ELF (optional).
