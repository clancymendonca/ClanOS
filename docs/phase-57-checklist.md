> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 57 Checklist: PLT JUMP_SLOT Relocations

## Scope

- [x] `R_X86_64_JUMP_SLOT` applied in `apply_dynamic_imports`.
- [x] PLT counters in `plt_status()`.
- [x] Covered by boot gate `fd_mmap` (`AresOS-BootGate: name=fd_mmap ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 57 --timeout 180

## Deferred

- [ ] Lazy PLT resolution on first call; IFUNC.
