# Phase 43 Checklist: Trust-Gated ELF Execution

## Scope

- [x] Seed `systrust` (`trust=system`, not on name allowlist).
- [x] `execute_trusted_manifest_elf` for system-trust programs only.
- [x] Emit `Phase43-TrustExec` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase43_trust_exec_check.py --timeout 180`

## Deferred

- [ ] Cryptographic signatures; arbitrary unsigned ELFs.
