# Phase 21 Checklist: Hardware User Page Tables

## Scope

- [x] Build real x86_64 page tables from inactive descriptors.
- [x] Map user stack pages for later Ring 3 entry.
- [x] Verify hardware translations match descriptor translations.
- [x] Add blocked `HwPageTableReady` process metadata.
- [x] Emit `Phase21-HwPageTables` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --test preemption_integration`
- [x] `python scripts/phase21_hw_page_table_check.py --timeout 120`

## Deferred

- [ ] Switch CR3 for execution.
- [ ] Enter Ring 3.
