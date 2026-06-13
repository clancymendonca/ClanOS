> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 30 Checklist: Per-Process CR3 Switching

## Scope

- [x] Switch between distinct user CR3 values and restore kernel CR3.
- [x] Verify distinct translations after switches.
- [x] Covered by boot gate `hw_paging` (`ClanOS-BootGate: name=hw_paging ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate hw_paging --timeout 180`

## Deferred

- [x] Scheduler-integrated CR3 switching on every context switch (Scope 31).
- [ ] Demand paging and SMP TLB shootdown (demand-zero slice in Scope 38).
