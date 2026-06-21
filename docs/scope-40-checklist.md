> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 40 Checklist: Integration gate

## Scope

- [x] `smoke_sched_userspace_integration` validates cumulative scope 31–39 counters.
- [x] Covered by validation gate `sched_userspace` (`ClanOS-Gate: name=sched_userspace ok=true`)
- [x] Validation matrix entries for scopes 31–40.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate sched_userspace --timeout 180`
- [x] `python scripts/validation_matrix.py` (scopes 31–40 in full matrix; PASS 2026-05-22)

## Deferred

- [ ] Full arbitrary ELF execution; production SMP scheduling (see scopes 43–49).
