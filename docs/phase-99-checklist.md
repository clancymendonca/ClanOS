> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 99 Checklist: LAPIC ICR Write Stub

## Scope

- [x] `lapic_icr_send_stub()` records `LAPIC_ICR_WRITES` via discard slot (no real MMIO in QEMU tests).
- [x] Covered by boot gate `smp_depth` (`AresOS-BootGate: name=smp_depth ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 99 --timeout 180

## Deferred

- [ ] Real LAPIC ICR MMIO programming; IPI reschedule.
