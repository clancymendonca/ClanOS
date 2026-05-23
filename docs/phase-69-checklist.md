# Phase 69 Checklist: AP Idle Trampoline Accounting

## Scope

- [x] AP idle tick counter when `cpus > 1`.
- [x] BSP still runs all scheduler work.
- [x] Emit `Phase69-ApIdle` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase69_ap_idle_check.py --timeout 180`

## Deferred

- [ ] Real AP entry trampoline; runnable tasks on APs.
