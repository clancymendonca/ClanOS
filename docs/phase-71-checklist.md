> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 71 Checklist: HW `syscall` / `sysret` Return Path

## Scope

- [x] `run_hw_syscall_probe` uses hardware `syscall` stub and `sysret` return.
- [x] `SYSRET_APPLIED` and `HW_SYSCALL_PROBES` counters in `user_syscall_hw`.
- [x] Covered by boot gate `syscall_ring3` (`AresOS-BootGate: name=syscall_ring3 ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 71 --timeout 180

## Deferred

- [ ] Full user ELF using only HW syscall entry (no int 0x80 bring-up).
