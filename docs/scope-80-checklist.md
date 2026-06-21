> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 80 Checklist: Integration gate (71–79)

## Scope

- [x] `smoke_syscall_ring3_integration` validates cumulative scope 71–79 counters.
- [x] Covered by validation gate `syscall_ring3` (`ClanOS-Gate: name=syscall_ring3 ok=true`)
- [x] Validation matrix entries for scopes 71–80.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate syscall_ring3 --timeout 180`
- [ ] `python scripts/validation_matrix.py --from-check scope71-sysret-check` (optional full matrix)

## Deferred

- [ ] COW fork; `execve`; pipes/sockets/`poll`; arbitrary unsigned ELF; production SMP scheduling.
