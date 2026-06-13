> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 62 Checklist: munmap

## Scope

- [x] `Munmap = 75` syscall for anon and file mmap pages.
- [x] Reject unmap of image-backed executable ranges.
- [x] TLB shootdown via `smp::request_tlb_shootdown`.
- [x] Covered by boot gate `vm_fork` (`AresOS-BootGate: name=vm_fork ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 62 --timeout 180

## Deferred

- [ ] Partial unmap within a region; munmap by length.
