> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 93 Checklist: Gap-Aware `mmap` Hint

## Scope

- [x] `vma::next_anon_hint` prefers lowest gap ≥ `MMAP_ANON_BASE` before high-water.
- [x] Covered by boot gate `smp_depth` (`AresOS-BootGate: name=smp_depth ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 93 --timeout 180

## Deferred

- [ ] Full VMA red-black tree; `MAP_SHARED` across processes.
