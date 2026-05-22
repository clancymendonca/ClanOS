# Phase 23 Checklist: Real iretq User Entry

## Scope

- [x] Enter Ring 3 through `iretq` to a controlled `ud2` stub.
- [x] Resume kernel execution through a modified trap frame.
- [x] Add blocked `UserEnteredHw` process metadata.
- [x] Emit `Phase23-Iretq` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase23_iretq_check.py --timeout 180`

## Deferred

- [ ] Vector 0x80 cooperative return.
- [ ] CPU `syscall` instruction.
