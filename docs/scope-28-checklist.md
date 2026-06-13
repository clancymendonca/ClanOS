> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 28 Checklist: Hardware Hello Execution

## Scope

- [x] Run `hello` through hardware syscall path.
- [x] Preserve `hello: exit=0 tick=...` output format.
- [x] Add blocked `UserHwElfExited` process metadata.
- [x] Covered by boot gate `hw_paging` (`ClanOS-BootGate: name=hw_paging ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate hw_paging --timeout 180`

## Deferred

- [ ] Multiple allowlisted programs.
