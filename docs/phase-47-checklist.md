# Phase 47 Checklist: File-Backed Demand Paging

## Scope

- [x] File-backed region at `0x500000` with demand map from storage.
- [x] Emit `Phase47-FileDemand` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase47_file_demand_check.py --timeout 180`

## Deferred

- [ ] Writable file-backed pages; mmap syscall surface.
