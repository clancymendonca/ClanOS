> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 88 Checklist: Ring 3 PLT Lazy Bind on `#PF`

## Scope

- [x] `try_ring3_plt_fault` from demand paging; `RING3_PLT_FAULT` / `RING3_PLT_BOUND`.
- [x] Covered by validation gate `path_exec` (`ClanOS-Gate: name=path_exec ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate path_exec --timeout 180`

## Deferred

- [ ] Full Ring-3 ELF first PLT call under HW syscall only.
