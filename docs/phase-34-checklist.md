> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 34 Checklist: Exit and Wait Syscalls

## Scope

- [x] `SyscallId::ExitProcess` and `WaitProcess`.
- [x] Kernel exit/wait accounting smoke.
- [x] Covered by boot gate `sched_userspace` (`AresOS-BootGate: name=sched_userspace ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 34 --timeout 180

## Deferred

- [ ] Per-PID wait queues and parent/child linkage.
