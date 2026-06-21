> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 93 Checklist: Gap-Aware `mmap` Hint

## Scope

- [x] `vma::next_anon_hint` prefers lowest gap ≥ `MMAP_ANON_BASE` before high-water.
- [x] Covered by validation gate `smp_depth` (`ClanOS-Gate: name=smp_depth ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate smp_depth --timeout 180`

## Deferred

- [ ] Full VMA red-black tree; `MAP_SHARED` across processes.
