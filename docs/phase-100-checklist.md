> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 100 Checklist: Integration Milestone (91–99)

## Scope

- [x] `phase100_integration_smoke` validates cumulative phase 91–99 counters (no nested re-run).
- [x] Covered by boot gate `smp_depth` (`AresOS-BootGate: name=smp_depth ok=true`)
- [x] Validation matrix entries for phases 91–100.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 100 --timeout 180
- [ ] `python scripts/validation_matrix.py --from-check phase91-fork-cow-check` (optional full matrix)

## Deferred

- [ ] TCP/UDP sockets; multi-fd `select`; full `execve` envp.
- [ ] Full COW for file-backed mappings; ACPI MADT AP bring-up; real LAPIC ICR MMIO.
- [ ] IFUNC; soname versioning; shared `MAP_SHARED` across processes.
