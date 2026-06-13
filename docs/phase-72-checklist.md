> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 72 Checklist: Ring 3 `chdir` from User

## Scope

- [x] Ring 3 HW stub passes user path in `rdi` for `Chdir`.
- [x] `RING3_CHDIRS` counter and `phase72_smoke`.
- [x] Covered by boot gate `syscall_ring3` (`AresOS-BootGate: name=syscall_ring3 ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 72 --timeout 180

## Deferred

- [ ] Dedicated `chdirprobe` user ELF manifest.
