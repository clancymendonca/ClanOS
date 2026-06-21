> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 60 Checklist: Integration gate (51–59)

## Scope

- [x] `smoke_fd_mmap_integration` validates cumulative scope 51–59 counters.
- [x] Covered by validation gate `fd_mmap` (`ClanOS-Gate: name=fd_mmap ok=true`)
- [x] Validation matrix entries for scopes 51–60.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate fd_mmap --timeout 180`
- [ ] `python scripts/validation_matrix.py --from-check scope51-proc-fd-check` (optional full matrix)

## Deferred

- [ ] Production SMP; arbitrary ELF; full VMA tree.
