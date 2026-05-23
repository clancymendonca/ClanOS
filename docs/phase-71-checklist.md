# Phase 71 Checklist: HW `syscall` / `sysret` Return Path

## Scope

- [x] `run_hw_syscall_probe` uses hardware `syscall` stub and `sysret` return.
- [x] `SYSRET_APPLIED` and `HW_SYSCALL_PROBES` counters in `user_syscall_hw`.
- [x] Emit `Phase71-Sysret` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase71_sysret_check.py --timeout 180`

## Deferred

- [ ] Full user ELF using only HW syscall entry (no int 0x80 bring-up).
