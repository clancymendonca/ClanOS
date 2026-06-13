> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 85 Checklist: Fork-Lite Address-Space Duplicate

## Scope

- [x] `fork_lite` assigns child CR3 via shallow `fork_duplicate_cr3` (shared frames, no COW).
- [x] `FORK_DUP_CHILDREN` / `FORK_DUP_CR3` counters.
- [x] Covered by boot gate `path_exec` (`ClanOS-BootGate: name=path_exec ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate path_exec --timeout 180`

## Deferred

- [ ] Per-page COW and write isolation smoke.
