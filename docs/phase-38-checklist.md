> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 38 Checklist: Demand-Zero Page Faults

## Scope

- [x] `#PF` handler delegates to `demand_paging`.
- [x] `map_demand_zero_page` for user growth region.
- [x] Covered by boot gate `sched_userspace` (`AresOS-BootGate: name=sched_userspace ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 38 --timeout 180

## Deferred

- [ ] File-backed demand read.
- [ ] SMP TLB shootdown.
