> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 32 Checklist: User Trap Frame Persistence

## Scope

- [x] `UserHwFrame` save/resume registry for scheduler preemption bring-up.
- [x] Smoke saves frame, yields scheduler, resumes saved frame.
- [x] Covered by boot gate `sched_userspace` (`AresOS-BootGate: name=sched_userspace ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 32 --timeout 180

## Deferred

- [ ] Full Ring 3 GPR save on timer interrupt.
- [ ] FPU/SSE state.
