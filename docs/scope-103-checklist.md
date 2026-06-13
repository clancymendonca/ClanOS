> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 103 Checklist: IPC Endpoint Guarantees

## Layer
kernel

## Tag
native

## Mode
documentation (deliverables landed)

## Scope

- [x] Deliverable: ABI_IPC.md G3 E-*
- [x] Consistent with [AXIOMS.md](AXIOMS.md)
- [x] Listed in [ROADMAP_POST100.md](ROADMAP_POST100.md)

## Gate G3

[ABI_IPC.md](ABI_IPC.md) required before endpoint implementation.

## Validation

- [x] `python scripts/semantic_lint.py`
- [x] Scopes 101–109: documentation deliverables complete
- [ ] Scopes 111+: `cargo check -p kernel` + smoke script TBD


## Deferred

- See [ROADMAP_POST100.md](ROADMAP_POST100.md) and [AXIOMS.md](AXIOMS.md) gates.
