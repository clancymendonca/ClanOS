# Phase 65 Checklist: Ring 3 HW Syscall Probes

## Scope

- [x] Hardware `syscall` stub exercises `WritePathProbe` and `Mprotect`.
- [x] Ring 3 counters in `user_syscall_hw`.
- [x] Emit `Phase65-Ring3Syscall` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase65_ring3_syscall_check.py --timeout 180`

## Deferred

- [ ] Dedicated user ELF manifest for syscall probe suite.
