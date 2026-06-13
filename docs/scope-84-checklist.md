> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 84 Checklist: VMA In-Region Split

## Scope

- [x] Middle `munmap` of multi-page anon mapping splits VMA registry (`VMA_SPLITS`).
- [x] Covered by boot gate `path_exec` (`ClanOS-BootGate: name=path_exec ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate path_exec --timeout 180`

## Deferred

- [ ] Variable mmap hint polish across registry gaps.
