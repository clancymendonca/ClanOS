> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 19 Checklist: Syscall Entry And Return ABI

## Scope

- [x] Add user register-frame syscall ABI records.
- [x] Dispatch user syscall frames through the existing syscall table.
- [x] Record return values and syscall errors for user-mode return.
- [x] Add a user syscall probe path for validated image programs.
- [x] Add blocked `UserSyscallReturned` process metadata.
- [x] Expose user syscall counters through shell and syscalls.
- [x] Covered by validation gate `userspace_bootstrap` (`ClanOS-Gate: name=userspace_bootstrap ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate userspace_bootstrap --timeout 180`

## Deferred

- [ ] Use CPU syscall/sysret instructions.
- [ ] Copy buffers through validated user pointers.
- [ ] Execute arbitrary ELF syscall instructions.
