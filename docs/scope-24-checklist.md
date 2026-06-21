> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 24 Checklist: Hardware User Trap Return

## Scope

- [x] Wire IDT vector `0x80` for cooperative user return.
- [x] Enter Ring 3 through `int 0x80` stub path.
- [x] Add blocked `UserHwTrapped` process metadata.
- [x] Covered by validation gate `hw_paging` (`ClanOS-Gate: name=hw_paging ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate hw_paging --timeout 180`

## Deferred

- [ ] CPU `syscall`/`sysret` path.
