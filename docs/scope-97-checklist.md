> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 97 Checklist: Work-Stealing Stub

## Scope

- [x] `try_work_steal()` when BSP runqueue empty and CPU1 has work.
- [x] `WORK_STEAL_ATTEMPTS` / `WORK_STEALS` counters.
- [x] Covered by boot gate `smp_depth` (`ClanOS-BootGate: name=smp_depth ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate smp_depth --timeout 180`

## Deferred

- [ ] Real task migration; per-AP scheduler loops.
