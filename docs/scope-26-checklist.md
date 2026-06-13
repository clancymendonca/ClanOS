> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 26 Checklist: Validated User Copyin

## Scope

- [x] Add bounded `copy_from_user` and `copy_to_user`.
- [x] Prove a user-buffer round-trip under active page tables.
- [x] Covered by boot gate `hw_paging` (`ClanOS-BootGate: name=hw_paging ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate hw_paging --timeout 180`

## Deferred

- [ ] Storage syscalls with user pointers.
