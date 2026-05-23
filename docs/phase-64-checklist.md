# Phase 64 Checklist: Fork-Lite with FD Inheritance

## Scope

- [x] `ForkLite = 76` creates child with inherited `fds` and `cwd`.
- [x] No page-table clone (FD isolation smoke only).
- [x] Emit `Phase64-ForkLite` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase64_forklite_check.py --timeout 180`

## Deferred

- [ ] COW fork; wait on child PID.
