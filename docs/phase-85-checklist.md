# Phase 85 Checklist: Fork-Lite Address-Space Duplicate

## Scope

- [x] `fork_lite` assigns child CR3 via shallow `fork_duplicate_cr3` (shared frames, no COW).
- [x] `FORK_DUP_CHILDREN` / `FORK_DUP_CR3` counters.
- [x] `Phase85-ForkDup` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase85_fork_dup_check.py --timeout 180`

## Deferred

- [ ] Per-page COW and write isolation smoke.
