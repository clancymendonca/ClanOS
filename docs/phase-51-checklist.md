# Phase 51 Checklist: Per-Process FD Tables

## Scope

- [x] `MAX_FILES` raised to 24; directory expanded to 3 sectors.
- [x] Per-process `fds` on `Process`; global `FD_TABLE` removed.
- [x] `current_process_id` / `process_for_cr3` on CR3 activate.
- [x] Emit `Phase51-ProcFd` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase51_proc_fd_check.py --timeout 180`

## Deferred

- [ ] FD inheritance across `fork`; `fcntl`.
