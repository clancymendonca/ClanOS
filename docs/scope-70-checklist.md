> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 70 Checklist: Integration gate (61–69)

## Scope

- [x] `smoke_vm_fork_integration` validates cumulative scope 61–69 counters.
- [x] Covered by validation gate `vm_fork` (`ClanOS-Gate: name=vm_fork ok=true`)
- [x] Validation matrix entries for scopes 61–70.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate vm_fork --timeout 180`
- [ ] `python scripts/validation_matrix.py --from-check scope61-chdir-check` (optional full matrix)

## Deferred

- [ ] Arbitrary unsigned ELF; PKI signatures; COW fork; production SMP scheduling.
