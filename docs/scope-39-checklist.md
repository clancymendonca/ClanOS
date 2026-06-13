> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 39 Checklist: Dynamic Linking Groundwork

## Scope

- [x] `parse_dt_needed` for CLAN seed ELFs.
- [x] `apply_dynamic_needed` wraps static relocations.
- [x] Covered by boot gate `sched_userspace` (`ClanOS-BootGate: name=sched_userspace ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate sched_userspace --timeout 180`

## Deferred

- [ ] Lazy PLT binding and multiple shared libraries (see scopes 41–42).
