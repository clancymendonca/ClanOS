> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 13 Checklist: Frame-Backed Mapping Stubs

## Scope

- [x] Define mapped image, mapped region, mapped page, frame token, action result, and mapping errors.
- [x] Convert load plans into deterministic mapping stubs with one frame token per planned page.
- [x] Track copy and zero-fill accounting without writing executable memory.
- [x] Reject unsafe writable+executable mappings at mapping time.
- [x] Add a bounded mapping registry with add, list, lookup, and aggregate status behavior.
- [x] Add loader map path and counters for mapped images, rejected mappings, mapped pages, copied bytes, and zero-filled bytes.
- [x] Attach mapped-stub metadata to blocked process records.
- [x] Expose mapping summaries through shell commands and syscalls.
- [x] Covered by boot gate `loader_security` (`ClanOS-BootGate: name=loader_security ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate loader_security --timeout 180`

## Deferred

- [ ] Allocate real physical frames for user executable mappings.
- [ ] Write image bytes into mapped executable memory.
- [ ] Build per-process page tables and switch CR3.
- [ ] Enter Ring 3 or jump to ELF entry points.
