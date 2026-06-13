> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 65 Checklist: Ring 3 HW Syscall Probes

## Scope

- [x] Hardware `syscall` stub exercises `WritePathProbe` and `Mprotect`.
- [x] Ring 3 counters in `user_syscall_hw`.
- [x] Covered by boot gate `vm_fork` (`AresOS-BootGate: name=vm_fork ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 65 --timeout 180

## Deferred

- [ ] Dedicated user ELF manifest for syscall probe suite.
