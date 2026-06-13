> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 33 Checklist: Concurrent Allowlisted ELFs

## Scope

- [x] Run `hello` and `exit42` under distinct hardware page tables.
- [x] Verify address-space isolation metadata.
- [x] Covered by boot gate `sched_userspace` (`AresOS-BootGate: name=sched_userspace ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 33 --timeout 180

## Deferred

- [ ] Scheduler-driven concurrent Ring 3 execution.
