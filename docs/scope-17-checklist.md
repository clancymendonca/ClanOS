> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 17 Checklist: User Context And Entry Frames

## Scope

- [x] Add user code and data selectors to the GDT.
- [x] Expose user selector descriptors for validation.
- [x] Build initial user entry frames with RIP, RSP, RFLAGS, CS, and SS.
- [x] Add user stack descriptors.
- [x] Add blocked `UserContextReady` process metadata.
- [x] Expose user-context status through shell and syscall surfaces.
- [x] Covered by boot gate `userspace_bootstrap` (`ClanOS-BootGate: name=userspace_bootstrap ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate userspace_bootstrap --timeout 180`

## Deferred

- [ ] Execute the interrupt-return transition to Ring 3.
- [ ] Switch CR3 to user page tables.
- [ ] Run ELF entry points.
