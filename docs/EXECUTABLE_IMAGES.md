# Executable Image Groundwork

Phase 11 adds executable-image recognition and address-space descriptors. Phase 12 adds load plans that model page-aligned placement, copy actions, zero-fill actions, and reservation accounting. Neither phase executes arbitrary machine code yet.

## Image Manifest

Image programs use the existing `ares-exec-v1` envelope:

```text
ares-exec-v1
name=hello
kind=elf64-image
entry=0x400000
image=/bin/hello.elf
requires=execute
trust=user
owner=user
description=ELF image validation fixture
```

The loader still supports `kind=builtin-alias` for current stored programs. `kind=elf64-image` is discoverable and validatable, but `run hello` returns an unsupported-execution error until a future phase adds executable mappings and privilege transitions.

## ELF64 Validation

The image parser accepts a deliberately small subset:

- ELF64 little-endian images
- x86_64 machine type
- loadable program headers
- bounded image and segment counts
- non-overlapping segments
- no writable+executable segments

The parser rejects invalid magic, unsupported architecture, invalid header layout, malformed segments, oversized images, and unsupported execution attempts with typed errors.

## Address-Space Descriptors

Phase 11 introduces descriptor-only address spaces:

- `AddressSpaceId`
- `VirtualRegion`
- `RegionKind`
- mapping flags derived from image segment flags

Descriptors validate user ranges, overlap, empty regions, and writable+executable mappings. They do not switch CR3 or create per-process page tables.

## Load Plans

Phase 12 converts validated images into load plans:

- page-aligned regions
- file-backed copy ranges
- zero-fill ranges for memory beyond file bytes
- planned page counts
- stack reservation accounting

These plans feed descriptor reservation metadata only. They do not allocate real user frames or mutate active page tables.

## Observability

The shell exposes:

- `bin validate <program>`
- `bin prepare <program>`
- richer `bin info <program>` output
- `bin plans`
- `ps` image/source display for loader-created process records

Boot emits:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## Deferred Work

- actual frame allocation and executable memory mapping
- per-process page tables and CR3 switching
- Ring 3 entry and syscall return paths
- demand paging and memory-mapped executable files
- dynamic linking and cryptographic signatures
