# Phase 75 Checklist: `syscallprobe` User ELF Manifest

## Scope

- [x] `/bin/syscallprobe` manifest and ELF seeded in storage.
- [x] HW syscall probes for `WritePathProbe` and `Mprotect`.
- [x] Emit `Phase75-SyscallProbe` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase75_syscallprobe_check.py --timeout 180`

## Deferred

- [ ] Dedicated probe ELF bytecode exercising every allowed HW syscall.
