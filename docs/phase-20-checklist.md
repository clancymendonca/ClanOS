> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 20 Checklist: Minimal ELF Execution MVP

## Scope

- [x] Allow the seeded `/bin/hello` ELF program to complete through the guarded user pipeline.
- [x] Return deterministic output and exit status for `run hello`.
- [x] Keep arbitrary ELF execution, dynamic linking, relocation, and demand paging out of scope.
- [x] Add blocked `UserElfExited` process metadata.
- [x] Expose ELF execution counters through shell and syscalls.
- [x] Covered by boot gate `userspace_bootstrap` (`AresOS-BootGate: name=userspace_bootstrap ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 20 --timeout 180

## Deferred

- [ ] Run arbitrary user ELF instructions.
- [ ] Implement relocations and dynamic linking.
- [ ] Implement demand paging and full process isolation.
