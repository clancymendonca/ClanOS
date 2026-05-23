# Phase 96 Checklist: VMA Adjacent Coalesce

## Scope

- [x] `vma::coalesce_adjacent` on munmap when regions share prot/backing.
- [x] `VMA_COALESCED` counter; `Phase96-VmaCoalesce` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase96_vma_coalesce_check.py --timeout 180`

## Deferred

- [ ] Full VMA red-black tree.
