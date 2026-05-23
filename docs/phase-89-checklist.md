# Phase 89 Checklist: LAPIC IPI Send Stub

## Scope

- [x] `LAPIC_IPI_SEND` counter on `request_tlb_shootdown`.
- [x] `phase89_smoke` and `Phase89-IpiSend` boot output.
- [x] `docs/SMP.md` updated.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase89_ipi_send_check.py --timeout 180`

## Deferred

- [ ] Real LAPIC ICR MMIO and AP ack paths.
