> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 21 Checklist: Hardware User Page Tables

## Scope

- [x] Build real x86_64 page tables from inactive descriptors.
- [x] Map user stack pages for later Ring 3 entry.
- [x] Verify hardware translations match descriptor translations.
- [x] Add blocked `HwPageTableReady` process metadata.
- [x] Covered by validation gate `hw_paging` (`ClanOS-Gate: name=hw_paging ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate hw_paging --timeout 180`

## Deferred

- [ ] Switch CR3 for execution.
- [ ] Enter Ring 3.
