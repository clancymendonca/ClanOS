> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 70 Checklist: Integration Milestone (61–69)

## Scope

- [x] `phase70_integration_smoke` validates cumulative phase 61–69 counters.
- [x] Covered by boot gate `vm_fork` (`AresOS-BootGate: name=vm_fork ok=true`)
- [x] Validation matrix entries for phases 61–70.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 70 --timeout 180
- [ ] `python scripts/validation_matrix.py --from-check phase61-chdir-check` (optional full matrix)

## Deferred

- [ ] Arbitrary unsigned ELF; PKI signatures; COW fork; production SMP scheduling.
