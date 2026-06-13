> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 53 Checklist: mprotect and Guard Pages

## Scope

- [x] `Mprotect` syscall with W^X enforcement.
- [x] Stack guard page probe below default user stack.
- [x] Covered by boot gate `fd_mmap` (`ClanOS-BootGate: name=fd_mmap ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate fd_mmap --timeout 180`

## Deferred

- [ ] User-triggered guard faults from Ring 3; `mprotect` on file-backed pages.
