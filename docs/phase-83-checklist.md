> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 83 Checklist: `chdirprobe` User ELF

## Scope

- [x] `/bin/chdirprobe` manifest + ELF seeds; allowlisted loader name.
- [x] Smoke: `Chdir` to `/tmp` then `GetCwd` verify; `CHDIRPROBE_OK` counter.
- [x] Covered by boot gate `path_exec` (`AresOS-BootGate: name=path_exec ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 83 --timeout 180

## Deferred

- [ ] Dedicated HW-only chdirprobe without kernel smoke helpers.
