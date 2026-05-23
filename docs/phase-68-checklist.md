# Phase 68 Checklist: Cross-CPU TLB Shootdown Accounting

## Scope

- [x] `request_tlb_shootdown` records per-CPU request/complete counts.
- [x] munmap and demand-map paths use shootdown helper.
- [x] Emit `Phase68-TlbShootdown` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase68_tlb_shootdown_check.py --timeout 180`

## Deferred

- [ ] Real IPI-based shootdown to remote APs.
