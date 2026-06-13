> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 41 Checklist: Shared Library Mapping

## Scope

- [x] Seed `/bin/libc_stub.elf` and manifest.
- [x] `attach_shared_library` maps dependency at `0x700000`.
- [x] Covered by boot gate `dynamic_runtime` (`AresOS-BootGate: name=dynamic_runtime ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 41 --timeout 180

## Deferred

- [ ] Multiple shared libraries and soname search paths.
