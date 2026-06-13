> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 67 Checklist: Lazy PLT Resolution

## Scope

- [x] `apply_dynamic_imports_lazy` defers `R_X86_64_JUMP_SLOT`.
- [x] `bind_lazy_plt` applies slots on demand.
- [x] Covered by boot gate `vm_fork` (`ClanOS-BootGate: name=vm_fork ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate vm_fork --timeout 180`

## Deferred

- [ ] Resolve on first user call from Ring 3; IFUNC.
