# Phase 44 Checklist: User Path Copyin

## Scope

- [x] `ReadPathProbe` syscall with bounded path validation.
- [x] Emit `Phase44-UserPath` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase44_user_path_check.py --timeout 180`

## Deferred

- [ ] Write paths from user space; symlink following.
