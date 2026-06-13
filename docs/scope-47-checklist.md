> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 47 Checklist: File-Backed Demand Paging

## Scope

- [x] File-backed region at `0x500000` with demand map from storage.
- [x] Covered by boot gate `dynamic_runtime` (`ClanOS-BootGate: name=dynamic_runtime ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate dynamic_runtime --timeout 180`

## Deferred

- [ ] Writable file-backed pages; mmap syscall surface.
