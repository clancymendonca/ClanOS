> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 26 Checklist: Validated User Copyin

## Scope

- [x] Add bounded `copy_from_user` and `copy_to_user`.
- [x] Prove a user-buffer round-trip under active page tables.
- [x] Covered by boot gate `hw_paging` (`AresOS-BootGate: name=hw_paging ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 26 --timeout 180

## Deferred

- [ ] Storage syscalls with user pointers.
