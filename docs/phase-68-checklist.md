> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 68 Checklist: Cross-CPU TLB Shootdown Accounting

## Scope

- [x] `request_tlb_shootdown` records per-CPU request/complete counts.
- [x] munmap and demand-map paths use shootdown helper.
- [x] Covered by boot gate `vm_fork` (`AresOS-BootGate: name=vm_fork ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 68 --timeout 180

## Deferred

- [ ] Real IPI-based shootdown to remote APs.
