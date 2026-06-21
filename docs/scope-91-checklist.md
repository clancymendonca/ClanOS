> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 91 Checklist: Fork-Lite COW Break

## Scope

- [x] `break_cow_page` / shared anon mapping after `fork_lite`; parent/child write isolation.
- [x] `FORK_COW_BREAKS` / `FORK_COW_ISOLATED` counters.
- [x] Covered by validation gate `smp_depth` (`ClanOS-Gate: name=smp_depth ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate smp_depth --timeout 180`

## Deferred

- [ ] Full COW on file-backed mappings; `#PF`-driven break on every user write.
