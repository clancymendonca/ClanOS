> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 77 Checklist: Ring 3 Lazy PLT First Call

## Scope

- [x] `RING3_PLT_BOUND` when lazy bind runs under Ring 3 smoke flag.
- [x] `phase77_smoke` extends lazy PLT bring-up.
- [x] Covered by boot gate `syscall_ring3` (`AresOS-BootGate: name=syscall_ring3 ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 77 --timeout 180

## Deferred

- [ ] PLT resolve from Ring 3 page fault on first call.
