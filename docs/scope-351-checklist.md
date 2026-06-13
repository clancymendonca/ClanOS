> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 351 Checklist: VGA Framebuffer Desktop Shell

## Layer
kernel

## Tag
native

## Mode
implemented

## Scope

- [x] Deliverable: mode 13h framebuffer + compositor pixel frame
- [x] Consistent with [ABI_COMPOSITOR_IPC.md](ABI_COMPOSITOR_IPC.md)
- [x] Listed in [ROADMAP_351_400.md](ROADMAP_351_400.md)

## Validation

- [x] `cargo check -p kernel`
- [x] `python scripts/gate/system.py --gate desktop_preview --timeout 180`

## Completed

- Scope 351: VGA desktop shell via compositor frame submit
