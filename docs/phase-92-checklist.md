# Phase 92 Checklist: `PollLite` Syscall

## Scope

- [x] `Poll = 82`: single-fd readiness (read = 1) on pipe fds.
- [x] `POLL_CALLS` / `POLL_READY` counters; `Phase92-PollLite` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase92_poll_lite_check.py --timeout 180`

## Deferred

- [ ] Multi-fd `select`/`poll`; sockets and non-pipe fds.
