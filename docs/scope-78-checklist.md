> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 78 Checklist: IPI TLB Shootdown Stub

## Scope

- [x] `IPI_SHOOTDOWN_SENT` / `IPI_SHOOTDOWN_ACKED` on `request_tlb_shootdown`.
- [x] BSP still performs `flush_all`; logical IPI accounting for QEMU.
- [x] Covered by boot gate `syscall_ring3` (`ClanOS-BootGate: name=syscall_ring3 ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate syscall_ring3 --timeout 180`

## Deferred

- [ ] Real LAPIC IPI delivery to remote CPUs.
