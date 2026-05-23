# Phase 62 Checklist: munmap

## Scope

- [x] `Munmap = 75` syscall for anon and file mmap pages.
- [x] Reject unmap of image-backed executable ranges.
- [x] TLB shootdown via `smp::request_tlb_shootdown`.
- [x] Emit `Phase62-Munmap` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase62_munmap_check.py --timeout 180`

## Deferred

- [ ] Partial unmap within a region; munmap by length.
