> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 66 Checklist: Minimal fcntl Stub

## Scope

- [x] `Fcntl = 77` supports `F_GETFD` and `F_DUPFD`.
- [x] Unknown commands rejected with counter.
- [x] Covered by boot gate `vm_fork` (`ClanOS-BootGate: name=vm_fork ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate vm_fork --timeout 180`

## Deferred

- [ ] `F_SETFD`, close-on-exec, full flag semantics.
