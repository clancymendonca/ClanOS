> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 42 Checklist: Dynamic Import Relocations

## Scope

- [x] `R_X86_64_GLOB_DAT` import relocs against mapped `libc_stub`.
- [x] Covered by boot gate `dynamic_runtime` (`AresOS-BootGate: name=dynamic_runtime ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 42 --timeout 180

## Deferred

- [ ] Lazy PLT binding and ifunc relocations.
