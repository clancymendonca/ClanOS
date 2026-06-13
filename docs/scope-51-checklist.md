> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 51 Checklist: Per-Process FD Tables

## Scope

- [x] `MAX_FILES` raised to 24; directory expanded to 3 sectors.
- [x] Per-process `fds` on `Process`; global `FD_TABLE` removed.
- [x] `current_process_id` / `process_for_cr3` on CR3 activate.
- [x] Covered by boot gate `fd_mmap` (`ClanOS-BootGate: name=fd_mmap ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate fd_mmap --timeout 180`

## Deferred

- [ ] FD inheritance across `fork`; `fcntl`.
