# Phase 113 Checklist: Rights Delegation Smoke

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
- [x] `cargo test -p kernel --features preemption --test preemption_integration` (phase120_cap_compat_smoke_works)
- [x] `python scripts/semantic_lint.py`
- [x] `python scripts/phase120_cap_integration_check.py --timeout 300`
- [x] `Phase120-CapCompat` boot smoke (phases 111-120 integration)


## Deferred

- See [ROADMAP_POST100.md](ROADMAP_POST100.md) and [AXIOMS.md](AXIOMS.md) gates.
