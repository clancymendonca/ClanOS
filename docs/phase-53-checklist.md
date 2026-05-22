# Phase 53 Checklist: mprotect and Guard Pages

## Scope

- [x] `Mprotect` syscall with W^X enforcement.
- [x] Stack guard page probe below default user stack.
- [x] Emit `Phase53-Mprotect` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase53_mprotect_check.py --timeout 180`

## Deferred

- [ ] User-triggered guard faults from Ring 3; `mprotect` on file-backed pages.
