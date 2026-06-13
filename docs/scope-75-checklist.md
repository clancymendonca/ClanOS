> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 75 Checklist: `syscallprobe` User ELF Manifest

## Scope

- [x] `/bin/syscallprobe` manifest and ELF seeded in storage.
- [x] HW syscall probes for `WritePathProbe` and `Mprotect`.
- [x] Covered by boot gate `syscall_ring3` (`ClanOS-BootGate: name=syscall_ring3 ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate syscall_ring3 --timeout 180`

## Deferred

- [ ] Dedicated probe ELF bytecode exercising every allowed HW syscall.
