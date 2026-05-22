# Shared Libraries and Dynamic Relocations

Phases 39, 41, and 42 extend the guarded ELF pipeline with `DT_NEEDED` detection, in-kernel shared library mapping, and import relocations.

## Phase 39 — Detection

Seed ELFs may include a dynamic section. `parse_dt_needed` records the dependency name (for example `libc_stub`). Phase 39 applies static relocations only; it does not load a separate ELF yet.

Boot smoke:

```text
Phase39-Dynamic: needed=..., reloc_ok=true
```

## Phase 41 — Mapping

`shared_loader::attach_shared_library` maps `/bin/libc_stub.elf` at virtual address `0x700000` when a main image reports `DT_NEEDED`. If the stub file is absent, bring-up falls back to `/bin/hello.elf` for validation.

Boot smoke:

```text
Phase41-SharedLib: needed=..., mapped=..., base=0x700000, pages=...
```

## Phase 42 — Import Relocations

`elf_reloc` applies `R_X86_64_GLOB_DAT` entries against the mapped shared library base. Static `R_X86_64_RELATIVE` / `R_X86_64_64` relocs from Phase 27 still run for the main image.

Boot smoke:

```text
Phase42-DynReloc: glob_dat=..., applied=..., ok=true
```

## Validation

```bash
python scripts/phase41_shared_lib_check.py --timeout 180
python scripts/phase42_dyn_reloc_check.py --timeout 180
```

## Deferred

- Multiple shared objects and soname search paths
- Lazy PLT / `JUMP_SLOT` binding and IFUNC relocations
- Per-process library namespaces
