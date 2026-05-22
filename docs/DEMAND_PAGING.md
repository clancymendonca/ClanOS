# Demand Paging

Phases 38 and 47 add user `#PF` handling: anonymous demand-zero growth and file-backed slices from storage.

## Phase 38 — Demand-Zero

`demand_paging` maps previously unmapped user pages on fault with zero-filled frames. Used for guarded stack/heap growth during hardware bring-up.

Boot smoke:

```text
Phase38-DemandZero: faults=..., mapped=..., ok=true
```

## Phase 47 — File-Backed

On fault, the handler can satisfy a mapping by reading a page from the simple filesystem when the faulting address corresponds to a prepared file-backed region.

Boot smoke:

```text
Phase47-FileDemand: faults=..., file_pages=..., ok=true
```

## W^X Interaction

Phase 48 rejects user mappings that combine writable and executable flags. Demand paging must not install W+X pages; see [USER_PAGE_TABLES.md](USER_PAGE_TABLES.md).

## Validation

```bash
python scripts/phase38_demand_zero_check.py --timeout 180
python scripts/phase47_file_demand_check.py --timeout 180
```

## Deferred

- Copy-on-write, shared mappings, and mmap-style syscalls
- Unified VMA tree instead of bring-up fault hooks
