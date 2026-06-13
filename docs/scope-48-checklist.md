> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 48 Checklist: W^X Mapping Policy

## Scope

- [x] Reject writable+executable user page flags in map paths.
- [x] Covered by boot gate `dynamic_runtime` (`ClanOS-BootGate: name=dynamic_runtime ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate dynamic_runtime --timeout 180`

## Deferred

- [ ] `mprotect` syscall; guard pages.
