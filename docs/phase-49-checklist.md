> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 49 Checklist: SMP Groundwork

## Scope

- [x] CPU count detection and AP accounting skeleton.
- [x] TLB flush hook on user map paths.
- [x] Covered by boot gate `dynamic_runtime` (`AresOS-BootGate: name=dynamic_runtime ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 49 --timeout 180

## Deferred

- [ ] Per-CPU runqueues; IPI TLB shootdown; real AP trampolines.
