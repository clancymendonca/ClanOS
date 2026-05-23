# Phase 73 Checklist: `munmap` with Length

## Scope

- [x] `Munmap` uses `arg1` as page-aligned length.
- [x] `vma::truncate_region` for partial unmap; image base rejected.
- [x] Emit `Phase73-MunmapLen` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase73_munmap_len_check.py --timeout 180`

## Deferred

- [ ] Full Linux-style VMA split/merge tree.
