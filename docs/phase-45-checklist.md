> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 45 Checklist: File Descriptor Table

## Scope

- [x] Bring-up FD table with `OpenFile` / `CloseFile` syscalls.
- [x] Covered by boot gate `dynamic_runtime` (`AresOS-BootGate: name=dynamic_runtime ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 45 --timeout 180

## Deferred

- [ ] Per-process FD tables; dup; cwd-relative paths.
