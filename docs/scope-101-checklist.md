> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 101 Checklist: Compat Syscall ABI Freeze

## Layer
governance

## Tag
compat

## Mode
documentation (deliverables landed)

## Scope

- [x] Deliverable: ABI_SYSCALL.md clan-abi-v1
- [x] Consistent with [AXIOMS.md](AXIOMS.md)
- [x] Listed in [ROADMAP_POST100.md](ROADMAP_POST100.md)

## Validation

- [x] `python scripts/semantic_lint.py`
- [x] Scopes 101–109: documentation deliverables complete
- [ ] Scopes 111+: `cargo check -p kernel` + smoke script TBD


## Deferred

- See [ROADMAP_POST100.md](ROADMAP_POST100.md) and [AXIOMS.md](AXIOMS.md) gates.
