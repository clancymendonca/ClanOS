> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 31 Checklist: Scheduler CR3 Binding

## Scope

- [x] Bind `cr3_phys` to process records when hardware page tables are built.
- [x] Apply user CR3 on preemptive context switch via `apply_scheduler_cr3_for_next`.
- [x] Optional `bound_cr3` on context tasks and `bind_context_task_cr3`.
- [x] Covered by boot gate `sched_userspace` (`ClanOS-BootGate: name=sched_userspace ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate sched_userspace --timeout 180`

## Deferred

- [ ] SMP TLB shootdown.
- [ ] CR3 switching from IRQ handlers without audit.
