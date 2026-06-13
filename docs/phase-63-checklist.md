> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 63 Checklist: Per-Process VMA Registry

## Scope

- [x] `kernel/src/vma.rs` region list on `Process`.
- [x] mmap/munmap register and unregister regions; overlap rejection.
- [x] Covered by boot gate `vm_fork` (`AresOS-BootGate: name=vm_fork ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 63 --timeout 180

## Deferred

- [ ] Full Linux-compatible VMA tree; shared mappings.
