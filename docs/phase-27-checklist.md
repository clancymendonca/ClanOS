> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 27 Checklist: Static ELF Relocations

## Scope

- [x] Apply static `R_X86_64_RELATIVE` fixups for seeded images.
- [x] Write image bytes into frame-backed pages during backing.
- [x] Covered by boot gate `hw_paging` (`AresOS-BootGate: name=hw_paging ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 27 --timeout 180

## Deferred

- [ ] Dynamic linking and `DT_NEEDED`.
