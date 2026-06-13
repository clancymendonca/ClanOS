> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 21 Checklist: Hardware User Page Tables

## Scope

- [x] Build real x86_64 page tables from inactive descriptors.
- [x] Map user stack pages for later Ring 3 entry.
- [x] Verify hardware translations match descriptor translations.
- [x] Add blocked `HwPageTableReady` process metadata.
- [x] Covered by boot gate `hw_paging` (`AresOS-BootGate: name=hw_paging ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 21 --timeout 180

## Deferred

- [ ] Switch CR3 for execution.
- [ ] Enter Ring 3.
