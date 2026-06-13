> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 90 Checklist: Integration Milestone (81–89)

## Scope

- [x] `smoke_path_exec_integration` validates cumulative scope 81–89 counters (no nested re-run).
- [x] Covered by boot gate `path_exec` (`ClanOS-BootGate: name=path_exec ok=true`)
- [x] Validation matrix entries for scopes 81–90.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate path_exec --timeout 180`
- [ ] `python scripts/validation_matrix.py --from-check scope81-hw-sysret-check` (optional full matrix)

## Deferred

- [ ] Full COW fork; `poll`/`select`; TCP/UDP sockets; arbitrary unsigned ELF.
- [ ] Work-stealing; IPI reschedule; runnable tasks on APs; ACPI MADT AP bring-up.
