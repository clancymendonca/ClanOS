# Phase 84 Checklist: VMA In-Region Split

## Scope

- [x] Middle `munmap` of multi-page anon mapping splits VMA registry (`VMA_SPLITS`).
- [x] `phase84_smoke` and `Phase84-VmaSplit` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase84_vma_split_check.py --timeout 180`

## Deferred

- [ ] Variable mmap hint polish across registry gaps.
