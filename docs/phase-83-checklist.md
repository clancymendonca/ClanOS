# Phase 83 Checklist: `chdirprobe` User ELF

## Scope

- [x] `/bin/chdirprobe` manifest + ELF seeds; allowlisted loader name.
- [x] Smoke: `Chdir` to `/tmp` then `GetCwd` verify; `CHDIRPROBE_OK` counter.
- [x] `Phase83-Chdirprobe` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase83_chdirprobe_check.py --timeout 180`

## Deferred

- [ ] Dedicated HW-only chdirprobe without kernel smoke helpers.
