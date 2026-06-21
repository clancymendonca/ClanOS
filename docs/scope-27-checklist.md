> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 27 Checklist: Static ELF Relocations

## Scope

- [x] Apply static `R_X86_64_RELATIVE` fixups for seeded images.
- [x] Write image bytes into frame-backed pages during backing.
- [x] Covered by validation gate `hw_paging` (`ClanOS-Gate: name=hw_paging ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate hw_paging --timeout 180`

## Deferred

- [ ] Dynamic linking and `DT_NEEDED`.
