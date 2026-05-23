# Phase 90 Checklist: Integration Milestone (81–89)

## Scope

- [x] `phase90_integration_smoke` validates cumulative phase 81–89 counters (no nested re-run).
- [x] Emit `Phase90-Integration` boot smoke output.
- [x] Validation matrix entries for phases 81–90.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase90_integration_check.py --timeout 300`
- [ ] `python scripts/validation_matrix.py --from-check phase81-hw-sysret-check` (optional full matrix)

## Deferred

- [ ] Full COW fork; `poll`/`select`; TCP/UDP sockets; arbitrary unsigned ELF.
- [ ] Work-stealing; IPI reschedule; runnable tasks on APs; ACPI MADT AP bring-up.
