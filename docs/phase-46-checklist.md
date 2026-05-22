# Phase 46 Checklist: FD Read/Write

## Scope

- [x] `ReadFd` / `WriteFd` syscalls with validated user buffers.
- [x] Emit `Phase46-FdIO` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase46_fd_io_check.py --timeout 120`

## Deferred

- [ ] Non-blocking I/O; scatter/gather.
