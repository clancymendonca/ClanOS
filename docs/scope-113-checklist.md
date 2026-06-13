> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 113 Checklist: Rights Delegation Smoke

## Layer
kernel

## Tag
native

## Mode
implementation (deliverables landed)

## Scope

- [x] Deliverable: R-01 R-06
- [x] Consistent with [AXIOMS.md](AXIOMS.md)
- [x] Listed in [ROADMAP_POST100.md](ROADMAP_POST100.md)

## Gate G2

[RIGHTS_ALGEBRA.md](RIGHTS_ALGEBRA.md) required before cap implementation.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration` (smoke_cap_compat_works)
- [x] `python scripts/semantic_lint.py`
- [x] `python scripts/gate/boot.py --gate boot --timeout 180`
- [x] Covered by unified boot/system gate (see VALIDATION_GATES.md)


## Deferred

- See [ROADMAP_POST100.md](ROADMAP_POST100.md) and [AXIOMS.md](AXIOMS.md) gates.
