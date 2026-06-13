> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 23 Checklist: Real iretq User Entry

## Scope

- [x] Enter Ring 3 through `iretq` to a controlled `ud2` stub.
- [x] Resume kernel execution through a modified trap frame.
- [x] Add blocked `UserEnteredHw` process metadata.
- [x] Covered by boot gate `hw_paging` (`ClanOS-BootGate: name=hw_paging ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate hw_paging --timeout 180`

## Deferred

- [ ] Vector 0x80 cooperative return.
- [ ] CPU `syscall` instruction.
