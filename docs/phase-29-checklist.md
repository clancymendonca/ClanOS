> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 29 Checklist: Allowlisted ELF Programs

## Scope

- [x] Allowlist `hello` and `exit42`.
- [x] Seed `/bin/exit42` manifest and ELF fixture.
- [x] Covered by boot gate `hw_paging` (`AresOS-BootGate: name=hw_paging ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 29 --timeout 180

## Deferred

- [ ] Manifest-discovered arbitrary ELFs.
