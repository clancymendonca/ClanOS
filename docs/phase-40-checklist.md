> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 40 Checklist: Integration Milestone

## Scope

- [x] `phase40_integration_smoke` validates cumulative phase 31–39 counters.
- [x] Covered by boot gate `sched_userspace` (`AresOS-BootGate: name=sched_userspace ok=true`)
- [x] Validation matrix entries for phases 31–40.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 40 --timeout 180
- [x] `python scripts/validation_matrix.py` (phases 31–40 in full matrix; PASS 2026-05-22)

## Deferred

- [ ] Full arbitrary ELF execution; production SMP scheduling (see phases 43–49).
