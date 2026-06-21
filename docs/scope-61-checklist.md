> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 61 Checklist: chdir and Path Normalization

## Scope

- [x] `Chdir = 74` syscall with bounded path copyin.
- [x] `normalize_absolute_path` collapses `..` segments in resolved paths.
- [x] Covered by validation gate `vm_fork` (`ClanOS-Gate: name=vm_fork ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate vm_fork --timeout 180`

## Deferred

- [ ] `chdir` from Ring 3 user ELF without smoke PID.
