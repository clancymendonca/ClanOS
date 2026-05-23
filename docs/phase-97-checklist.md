# Phase 97 Checklist: Work-Stealing Stub

## Scope

- [x] `try_work_steal()` when BSP runqueue empty and CPU1 has work.
- [x] `WORK_STEAL_ATTEMPTS` / `WORK_STEALS` counters.
- [x] `Phase97-WorkSteal` boot output; [`SMP.md`](SMP.md) updated.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase97_work_steal_check.py --timeout 180`

## Deferred

- [ ] Real task migration; per-AP scheduler loops.
