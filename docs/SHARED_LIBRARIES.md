# Shared Libraries and Dynamic Relocations

Scopes 39, 41, and 42 extend the guarded ELF pipeline with `DT_NEEDED` detection, in-kernel shared library mapping, and import relocations.

## Scope 39 — Detection

Seed ELFs may include a dynamic section. `parse_dt_needed` records the dependency name (for example `libc_stub`). Scope 39 applies static relocations only; it does not load a separate ELF yet.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## Scope 41 — Mapping

`shared_loader::attach_shared_library` maps `/bin/libc_stub.elf` at virtual address `0x700000` when a main image reports `DT_NEEDED`. If the stub file is absent, bring-up falls back to `/bin/hello.elf` for validation.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## Scope 42 — Import Relocations

`elf_reloc` applies `R_X86_64_GLOB_DAT` entries against the mapped shared library base. Static `R_X86_64_RELATIVE` / `R_X86_64_64` relocs from Scope 27 still run for the main image.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## Validation

```bash
python scripts/gate/boot.py --gate dynamic_runtime --timeout 180
python scripts/gate/boot.py --gate dynamic_runtime --timeout 180
```

## Deferred

- Multiple shared objects and soname search paths
- Lazy PLT / `JUMP_SLOT` binding and IFUNC relocations
- Per-process library namespaces
