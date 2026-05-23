# Phase 74 Checklist: `WaitLite` on Fork-Lite Child

## Scope

- [x] `WaitLite = 78` waits for fork-lite child exit code.
- [x] `ExitProcess` records exit on current/smoke process.
- [x] Emit `Phase74-WaitLite` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase74_waitlite_check.py --timeout 180`

## Deferred

- [ ] Copy-on-write fork and real child scheduler exit.
