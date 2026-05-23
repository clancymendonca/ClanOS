# Phase 99 Checklist: LAPIC ICR Write Stub

## Scope

- [x] `lapic_icr_send_stub()` records `LAPIC_ICR_WRITES` via discard slot (no real MMIO in QEMU tests).
- [x] `Phase99-LapicIcr` boot output; [`SMP.md`](SMP.md) IPI section extended.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase99_lapic_icr_check.py --timeout 180`

## Deferred

- [ ] Real LAPIC ICR MMIO programming; IPI reschedule.
