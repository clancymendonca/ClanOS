> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 36 Checklist: Storage Syscalls With Copyin

## Scope

- [x] `ReadFileProbe` / `WriteFileProbe` syscalls via `invoke_raw`.
- [x] `storage_read_probe` using validated `copy_to_user`.
- [x] Covered by boot gate `sched_userspace` (`ClanOS-BootGate: name=sched_userspace ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate sched_userspace --timeout 180`

## Deferred

- [ ] Arbitrary path strings from user space.
