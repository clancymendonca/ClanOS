> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 18 Checklist: Controlled Ring 3 Trampoline

## Scope

- [x] Add a controlled user trap vector.
- [x] Add Ring 3 trampoline result and error records.
- [x] Model controlled entry and trap-back behavior from prepared user contexts.
- [x] Add blocked `UserTrapped` process metadata.
- [x] Expose Ring 3 trampoline counters through shell and syscalls.
- [x] Covered by validation gate `userspace_bootstrap` (`ClanOS-Gate: name=userspace_bootstrap ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate userspace_bootstrap --timeout 180`

## Deferred

- [ ] Execute a hardware `iretq` transition.
- [ ] Run arbitrary ELF entry points.
- [ ] Implement user syscall return for real user code.
