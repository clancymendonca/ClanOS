# Phase 49 Checklist: SMP Groundwork

## Scope

- [x] CPU count detection and AP accounting skeleton.
- [x] TLB flush hook on user map paths.
- [x] Emit `Phase49-Smp` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase49_smp_check.py --timeout 180`

## Deferred

- [ ] Per-CPU runqueues; IPI TLB shootdown; real AP trampolines.
