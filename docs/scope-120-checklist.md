> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 120 Checklist: Integration Cap Compat

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
- [x] `cargo test -p kernel --features preemption --test preemption_integration` (smoke_cap_compat_works)
- [x] `python scripts/semantic_lint.py`
- [x] `python scripts/gate/boot.py --gate capabilities --timeout 180`
- [x] Covered by boot gate `capabilities` (`ClanOS-BootGate: name=capabilities ok=true`)


## Deferred

- See [ROADMAP_POST100.md](ROADMAP_POST100.md) and [AXIOMS.md](AXIOMS.md) gates.
