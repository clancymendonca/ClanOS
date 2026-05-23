# Phase 78 Checklist: IPI TLB Shootdown Stub

## Scope

- [x] `IPI_SHOOTDOWN_SENT` / `IPI_SHOOTDOWN_ACKED` on `request_tlb_shootdown`.
- [x] BSP still performs `flush_all`; logical IPI accounting for QEMU.
- [x] Emit `Phase78-IpiTlb` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase78_ipi_tlb_check.py --timeout 180`

## Deferred

- [ ] Real LAPIC IPI delivery to remote CPUs.
