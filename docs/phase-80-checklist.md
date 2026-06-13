> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 80 Checklist: Integration Milestone (71–79)

## Scope

- [x] `phase80_integration_smoke` validates cumulative phase 71–79 counters.
- [x] Covered by boot gate `syscall_ring3` (`AresOS-BootGate: name=syscall_ring3 ok=true`)
- [x] Validation matrix entries for phases 71–80.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 80 --timeout 180
- [ ] `python scripts/validation_matrix.py --from-check phase71-sysret-check` (optional full matrix)

## Deferred

- [ ] COW fork; `execve`; pipes/sockets/`poll`; arbitrary unsigned ELF; production SMP scheduling.
