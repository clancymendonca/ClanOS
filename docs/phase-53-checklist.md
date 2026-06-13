> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 53 Checklist: mprotect and Guard Pages

## Scope

- [x] `Mprotect` syscall with W^X enforcement.
- [x] Stack guard page probe below default user stack.
- [x] Covered by boot gate `fd_mmap` (`AresOS-BootGate: name=fd_mmap ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 53 --timeout 180

## Deferred

- [ ] User-triggered guard faults from Ring 3; `mprotect` on file-backed pages.
