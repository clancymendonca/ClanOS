> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 22 Checklist: Controlled CR3 Activation

## Scope

- [x] Activate user CR3 and restore kernel CR3 with interrupts disabled.
- [x] Verify entry-point translation under active user page tables.
- [x] Add blocked `Cr3Activated` process metadata.
- [x] Covered by boot gate `hw_paging` (`AresOS-BootGate: name=hw_paging ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 22 --timeout 180

## Deferred

- [ ] iretq user execution.
