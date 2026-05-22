# Phase 24 Checklist: Hardware User Trap Return

## Scope

- [x] Wire IDT vector `0x80` for cooperative user return.
- [x] Enter Ring 3 through `int 0x80` stub path.
- [x] Add blocked `UserHwTrapped` process metadata.
- [x] Emit `Phase24-UserTrap` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase24_user_trap_check.py --timeout 180`

## Deferred

- [ ] CPU `syscall`/`sysret` path.
