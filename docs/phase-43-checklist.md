> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 43 Checklist: Trust-Gated ELF Execution

## Scope

- [x] Seed `systrust` (`trust=system`, not on name allowlist).
- [x] `execute_trusted_manifest_elf` for system-trust programs only.
- [x] Covered by boot gate `dynamic_runtime` (`AresOS-BootGate: name=dynamic_runtime ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 43 --timeout 180

## Deferred

- [ ] Cryptographic signatures; arbitrary unsigned ELFs.
