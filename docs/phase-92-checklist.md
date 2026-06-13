> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 92 Checklist: `PollLite` Syscall

## Scope

- [x] `Poll = 82`: single-fd readiness (read = 1) on pipe fds.
- [x] Covered by boot gate `smp_depth` (`AresOS-BootGate: name=smp_depth ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 92 --timeout 180

## Deferred

- [ ] Multi-fd `select`/`poll`; sockets and non-pipe fds.
