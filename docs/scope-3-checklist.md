> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 3 Completion Checklist (Memory)

Date: 2026-03-17

## Scope
Scope 3 roadmap goals:
- paging implementation
- frame allocator
- heap allocation

## Completion Criteria
- [x] Level-4 page table walker (`memory::init` with physical memory offset)
- [x] `BootInfoFrameAllocator` allocates physical frames from bootloader memory map
- [x] Kernel heap mapped (2 MiB region at `HEAP_START`)
- [x] `linked_list_allocator` registered as global `#[global_allocator]`
- [x] `alloc` crate available kernel-wide (`extern crate alloc`)
- [x] Heap allocation tests: `simple_allocation`, `large_vec`, `many_boxes`,
  `many_boxes_long_lived`, `rc_allocation` — all pass
- [x] `alloc_error_handler` defined for OOM panics

## Validation Snapshot
- Last full validation command: `cargo test -p kernel`
- Result: pass — all 5 `heap_allocation::*` tests `[ok]`

## Notes
- Physical memory is identity-mapped via the bootloader's `map_physical_memory` feature.
- Virtual memory address space management beyond the initial heap is a Scope 6 concern.

## Validation

```bash
cargo check -p kernel
python scripts/gate/boot.py --gate boot --timeout 360
python scripts/validation_matrix.py --smoke-timeout 180
```

See [VALIDATION_GATES.md](VALIDATION_GATES.md).
