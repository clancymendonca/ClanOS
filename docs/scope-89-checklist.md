> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 89 Checklist: LAPIC IPI Send Stub

## Scope

- [x] `LAPIC_IPI_SEND` counter on `request_tlb_shootdown`.
- [x] Covered by boot gate `path_exec` (`ClanOS-BootGate: name=path_exec ok=true`)
- [x] `docs/SMP.md` updated.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate path_exec --timeout 180`

## Deferred

- [ ] Real LAPIC ICR MMIO and AP ack paths.
