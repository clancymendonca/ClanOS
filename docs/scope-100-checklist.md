> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 100 Checklist: Integration Milestone (91–99)

## Scope

- [x] `smoke_smp_depth_integration` validates cumulative scope 91–99 counters (no nested re-run).
- [x] Covered by boot gate `smp_depth` (`ClanOS-BootGate: name=smp_depth ok=true`)
- [x] Validation matrix entries for scopes 91–100.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate smp_depth --timeout 180`
- [ ] `python scripts/validation_matrix.py --from-check scope91-fork-cow-check` (optional full matrix)

## Deferred

- [ ] TCP/UDP sockets; multi-fd `select`; full `execve` envp.
- [ ] Full COW for file-backed mappings; ACPI MADT AP bring-up; real LAPIC ICR MMIO.
- [ ] IFUNC; soname versioning; shared `MAP_SHARED` across processes.
