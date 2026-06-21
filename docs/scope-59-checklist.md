> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 59 Checklist: Per-CPU Runqueue Skeleton

## Scope

- [x] Per-CPU enqueue counters on scheduler preempt.
- [x] APs remain parked; BSP accounts runnable work.
- [x] Covered by validation gate `fd_mmap` (`ClanOS-Gate: name=fd_mmap ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate fd_mmap --timeout 180`

## Deferred

- [ ] Real per-CPU ready queues; AP scheduling; IPI wakeups.
