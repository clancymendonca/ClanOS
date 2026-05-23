# Phase 67 Checklist: Lazy PLT Resolution

## Scope

- [x] `apply_dynamic_imports_lazy` defers `R_X86_64_JUMP_SLOT`.
- [x] `bind_lazy_plt` applies slots on demand.
- [x] Emit `Phase67-LazyPlt` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase67_lazy_plt_check.py --timeout 180`

## Deferred

- [ ] Resolve on first user call from Ring 3; IFUNC.
