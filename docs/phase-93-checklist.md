# Phase 93 Checklist: Gap-Aware `mmap` Hint

## Scope

- [x] `vma::next_anon_hint` prefers lowest gap ≥ `MMAP_ANON_BASE` before high-water.
- [x] `MMAP_GAPS_USED` counter; `Phase93-MmapGap` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase93_mmap_gap_check.py --timeout 180`

## Deferred

- [ ] Full VMA red-black tree; `MAP_SHARED` across processes.
