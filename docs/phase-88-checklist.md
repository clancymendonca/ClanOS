# Phase 88 Checklist: Ring 3 PLT Lazy Bind on `#PF`

## Scope

- [x] `try_ring3_plt_fault` from demand paging; `RING3_PLT_FAULT` / `RING3_PLT_BOUND`.
- [x] `phase88_smoke` and `Phase88-Ring3PltFault` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase88_ring3_plt_fault_check.py --timeout 180`

## Deferred

- [ ] Full Ring-3 ELF first PLT call under HW syscall only.
