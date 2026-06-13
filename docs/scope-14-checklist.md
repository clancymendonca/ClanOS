> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 14 Checklist: Frame Ownership Service

## Scope

- [x] Add a persistent frame ownership registry initialized from the bootloader memory map.
- [x] Track bounded frame records, owners, allocations, releases, and failed allocation attempts.
- [x] Preserve Scope 13 deterministic mapping stubs without consuming owned frames.
- [x] Expose frame ownership status through shell and syscall surfaces.
- [x] Covered by boot gate `memory_layout` (`ClanOS-BootGate: name=memory_layout ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate memory_layout --timeout 180`

## Deferred

- [ ] Use owned frames as backing storage for executable load plans.
- [ ] Install owned frames into inactive user page tables.
- [ ] Reclaim frames from terminated user processes.
