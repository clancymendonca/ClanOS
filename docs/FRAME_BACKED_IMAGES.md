# Frame-Backed Images

Scope 15 converts Scope 13 mapped-image stubs into frame-backed image records. These records consume owned frames from the Scope 14 frame ownership service and attach them to the mapped executable pages. Scope 16 uses these records to build inactive user page-table descriptors.

## Backed Image Contents

A `FrameBackedImage` records:

- source image name and path
- mapping id and address-space id
- backed regions and pages
- owned frame records for each mapped page
- copied and zero-filled byte counts
- page permission counts
- owner credentials
- `MappingState::FrameBacked`

The copy and zero-fill operations are still accounting records. They are associated with owned backing frames, but Scope 15 does not install those frames into process page tables or execute from them.

## Loader Flow

```mermaid
flowchart TD
Prepare[PrepareProgramImage] --> MapStub[MappingStub]
MapStub --> FrameOwnership[FrameOwnership]
FrameOwnership --> BackedImage[FrameBackedImage]
BackedImage --> ProcessMetadata[Blocked Process Metadata]
```

The loader exposes `back_mapped_program(credentials, name)`. It prepares the image, creates a mapping stub, consumes owned frames for each mapped page, records a blocked `FrameBacked` process record, and updates status counters.

## Shell And Smoke

The shell exposes:

- `bin back <program>`
- `bin plans`
- `frames`

Boot emits:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## Safety Boundary

`run hello` remains unsupported in Scope 15. Frame-backed records are the data needed by later page-table work, not executable user mappings. Scope 16 adds descriptor translation, but still does not switch CR3.
