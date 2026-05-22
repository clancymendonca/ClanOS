# Phase 25 Checklist: CPU syscall / sysret Path

## Scope

- [x] Configure syscall MSRs and entry stub.
- [x] Run tick-probe syscall from hardware user code.
- [x] Return to kernel through `int 0x80` after `syscall`.
- [x] Add blocked `UserHwSyscallReturned` process metadata.
- [x] Emit `Phase25-SyscallHw` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase25_syscall_hw_check.py --timeout 180`

## Deferred

- [ ] Arbitrary syscall IDs from user programs.
- [ ] User buffer arguments without copyin validation.
