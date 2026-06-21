> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 81 Checklist: Real HW `syscall` / `sysret`

## Scope

- [x] `HW_SYSRET_REAL` counter distinct from int 0x80 `SYSRET_APPLIED`.
- [x] Covered by validation gate `path_exec` (`ClanOS-Gate: name=path_exec ok=true`)
- [x] Optional `hw-sysret-probe` feature for QEMU HW probe (host tests use counter stub).

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate path_exec --timeout 180`

## Deferred

- [x] QEMU `hw-sysret-probe`: dedicated syscall stack, `syscall` dispatch, kernel resume after probe (`sysret` #GPs under QEMU).
