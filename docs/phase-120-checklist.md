> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 120 Checklist: Integration Cap Compat

## Layer
kernel

## Tag
governance

## Mode
implementation (deliverables landed)

## Scope

- [x] Deliverable: milestone 120
- [x] Consistent with [AXIOMS.md](AXIOMS.md)
- [x] Listed in [ROADMAP_POST100.md](ROADMAP_POST100.md)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration` (phase120_cap_compat_smoke_works)
- [x] `python scripts/semantic_lint.py`
- [x] `python scripts/gate/boot.py --phase 120 --timeout 180
- [x] Covered by boot gate `capabilities` (`AresOS-BootGate: name=capabilities ok=true`)


## Deferred

- See [ROADMAP_POST100.md](ROADMAP_POST100.md) and [AXIOMS.md](AXIOMS.md) gates.
