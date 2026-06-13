> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 11 Checklist

Status: Complete

- [x] Add executable image, segment, format, flag, and image-load error types.
- [x] Add conservative ELF64 header and load-segment validation.
- [x] Extend `ares-exec-v1` with `kind=elf64-image` and `image=<path>`.
- [x] Seed a small `/bin/hello` image manifest and `/bin/hello.elf` validation fixture.
- [x] Require execute permission on both image manifests and referenced image files.
- [x] Reject actual ELF image execution with a clear unsupported-execution result.
- [x] Add process image metadata for loader-created process records.
- [x] Add descriptor-only address-space and virtual-region validation.
- [x] Expose `bin validate <program>` and richer `bin info` image fields.
- [x] Covered by boot gate `loader_security` (`AresOS-BootGate: name=loader_security ok=true`)
- [x] Add Phase 11 QEMU validation and validation matrix coverage.

Exit gate:

- [x] Built-in aliases still launch.
- [x] Valid image manifests are discoverable and validate cleanly.
- [x] Malformed or non-executable image records do not panic.
- [x] No Phase 11 path executes arbitrary binary code.

## Validation

```bash
cargo check -p kernel
python scripts/gate/boot.py --phase 11 --timeout 180
python scripts/validation_matrix.py --smoke-timeout 180
```

See [VALIDATION_GATES.md](VALIDATION_GATES.md).
