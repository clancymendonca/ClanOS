> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 46 Checklist: FD Read/Write

## Scope

- [x] `ReadFd` / `WriteFd` syscalls with validated user buffers.
- [x] Covered by validation gate `dynamic_runtime` (`ClanOS-Gate: name=dynamic_runtime ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate dynamic_runtime --timeout 180`

## Deferred

- [ ] Non-blocking I/O; scatter/gather.
