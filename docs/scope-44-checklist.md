> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 44 Checklist: User Path Copyin

## Scope

- [x] `ReadPathProbe` syscall with bounded path validation.
- [x] Covered by boot gate `dynamic_runtime` (`ClanOS-BootGate: name=dynamic_runtime ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate dynamic_runtime --timeout 180`

## Deferred

- [ ] Write paths from user space; symlink following.
