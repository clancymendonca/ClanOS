> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 33 Checklist: Concurrent Allowlisted ELFs

## Scope

- [x] Run `hello` and `exit42` under distinct hardware page tables.
- [x] Verify address-space isolation metadata.
- [x] Covered by validation gate `sched_userspace` (`ClanOS-Gate: name=sched_userspace ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate sched_userspace --timeout 180`

## Deferred

- [ ] Scheduler-driven concurrent Ring 3 execution.
