> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 96 Checklist: VMA Adjacent Coalesce

## Scope

- [x] `vma::coalesce_adjacent` on munmap when regions share prot/backing.
- [x] Covered by validation gate `smp_depth` (`ClanOS-Gate: name=smp_depth ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate smp_depth --timeout 180`

## Deferred

- [ ] Full VMA red-black tree.
