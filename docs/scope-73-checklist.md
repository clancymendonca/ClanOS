> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 73 Checklist: `munmap` with Length

## Scope

- [x] `Munmap` uses `arg1` as page-aligned length.
- [x] `vma::truncate_region` for partial unmap; image base rejected.
- [x] Covered by validation gate `syscall_ring3` (`ClanOS-Gate: name=syscall_ring3 ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate syscall_ring3 --timeout 180`

## Deferred

- [ ] Full Linux-style VMA split/merge tree.
