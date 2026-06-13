> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 74 Checklist: `WaitLite` on Fork-Lite Child

## Scope

- [x] `WaitLite = 78` waits for fork-lite child exit code.
- [x] `ExitProcess` records exit on current/smoke process.
- [x] Covered by boot gate `syscall_ring3` (`ClanOS-BootGate: name=syscall_ring3 ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate syscall_ring3 --timeout 180`

## Deferred

- [ ] Copy-on-write fork and real child scheduler exit.
