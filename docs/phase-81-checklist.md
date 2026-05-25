# Phase 81 Checklist: Real HW `syscall` / `sysret`

## Scope

- [x] `HW_SYSRET_REAL` counter distinct from int 0x80 `SYSRET_APPLIED`.
- [x] `phase81_hw_sysret_smoke` and `Phase81-HwSysret` boot output.
- [x] Optional `hw-sysret-probe` feature for QEMU HW probe (host tests use counter stub).

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase81_hw_sysret_check.py --timeout 180` (uses `preemption,hw-sysret-probe`)

## Deferred

- [x] QEMU `hw-sysret-probe`: dedicated syscall stack, `syscall` dispatch, kernel resume after probe (`sysret` #GPs under QEMU).
