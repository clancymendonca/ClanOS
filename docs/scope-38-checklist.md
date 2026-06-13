> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 38 Checklist: Demand-Zero Page Faults

## Scope

- [x] `#PF` handler delegates to `demand_paging`.
- [x] `map_demand_zero_page` for user growth region.
- [x] Covered by boot gate `sched_userspace` (`ClanOS-BootGate: name=sched_userspace ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate sched_userspace --timeout 180`

## Deferred

- [ ] File-backed demand read.
- [ ] SMP TLB shootdown.
