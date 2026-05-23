# Phase 72 Checklist: Ring 3 `chdir` from User

## Scope

- [x] Ring 3 HW stub passes user path in `rdi` for `Chdir`.
- [x] `RING3_CHDIRS` counter and `phase72_smoke`.
- [x] Emit `Phase72-Ring3Chdir` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase72_ring3_chdir_check.py --timeout 180`

## Deferred

- [ ] Dedicated `chdirprobe` user ELF manifest.
