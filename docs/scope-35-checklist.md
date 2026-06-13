> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 35 Checklist: Hardware Syscall Dispatch Table

## Scope

- [x] `ALLOWED_HW_SYSCALLS` allowlist in `user_syscall_hw`.
- [x] Reject unknown syscall IDs with accounting.
- [x] Covered by boot gate `sched_userspace` (`ClanOS-BootGate: name=sched_userspace ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate sched_userspace --timeout 180`

## Deferred

- [ ] Unbounded syscall IDs from user programs.
- [ ] User buffer arguments without validated copyin.
