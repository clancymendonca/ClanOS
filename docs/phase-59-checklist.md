> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 59 Checklist: Per-CPU Runqueue Skeleton

## Scope

- [x] Per-CPU enqueue counters on scheduler preempt.
- [x] APs remain parked; BSP accounts runnable work.
- [x] Covered by boot gate `fd_mmap` (`AresOS-BootGate: name=fd_mmap ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 59 --timeout 180

## Deferred

- [ ] Real per-CPU ready queues; AP scheduling; IPI wakeups.
