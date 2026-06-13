> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 25 Checklist: CPU syscall / sysret Path

## Scope

- [x] Configure syscall MSRs and entry stub.
- [x] Run tick-probe syscall from hardware user code.
- [x] Return to kernel through `int 0x80` after `syscall`.
- [x] Add blocked `UserHwSyscallReturned` process metadata.
- [x] Covered by boot gate `hw_paging` (`ClanOS-BootGate: name=hw_paging ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate hw_paging --timeout 180`

## Deferred

- [ ] Arbitrary syscall IDs from user programs.
- [ ] User buffer arguments without copyin validation.
