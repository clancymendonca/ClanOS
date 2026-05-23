# Phase 91 Checklist: Fork-Lite COW Break

## Scope

- [x] `break_cow_page` / shared anon mapping after `fork_lite`; parent/child write isolation.
- [x] `FORK_COW_BREAKS` / `FORK_COW_ISOLATED` counters.
- [x] `Phase91-ForkCow` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase91_fork_cow_check.py --timeout 180`

## Deferred

- [ ] Full COW on file-backed mappings; `#PF`-driven break on every user write.
