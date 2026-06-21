> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 34 Checklist: Exit and Wait Syscalls

## Scope

- [x] `SyscallId::ExitProcess` and `WaitProcess`.
- [x] Kernel exit/wait accounting smoke.
- [x] Covered by validation gate `sched_userspace` (`ClanOS-Gate: name=sched_userspace ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate sched_userspace --timeout 180`

## Deferred

- [ ] Per-PID wait queues and parent/child linkage.
