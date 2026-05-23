# Phase 61 Checklist: chdir and Path Normalization

## Scope

- [x] `Chdir = 74` syscall with bounded path copyin.
- [x] `normalize_absolute_path` collapses `..` segments in resolved paths.
- [x] Emit `Phase61-Chdir` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase61_chdir_check.py --timeout 180`

## Deferred

- [ ] `chdir` from Ring 3 user ELF without smoke PID.
