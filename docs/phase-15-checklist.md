> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 15 Checklist: Frame-Backed Images

## Scope

- [x] Add frame-backed image, region, and page records.
- [x] Consume owned frames from the Phase 14 frame ownership service.
- [x] Preserve Phase 13 mapping stub records and unsupported execution behavior.
- [x] Account copy and zero-fill bytes per backed page.
- [x] Add loader counters and blocked process metadata for `FrameBacked` records.
- [x] Expose frame-backed status through shell and syscall surfaces.
- [x] Covered by boot gate `memory_layout` (`AresOS-BootGate: name=memory_layout ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 15 --timeout 180

## Deferred

- [ ] Install frame-backed pages into inactive user page tables.
- [ ] Copy bytes into executable virtual mappings.
- [ ] Enter Ring 3 or jump to ELF entry points.
