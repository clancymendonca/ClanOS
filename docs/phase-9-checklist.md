> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 9 Checklist (Stored Program Loader)

**Date**: 2026-05-13  
**Status**: Complete

## 1. Executable Manifest Format

- [x] `ares-exec-v1` text manifest format
- [x] `name`, `kind`, `entry`, and `description` fields
- [x] `builtin-alias` executable kind
- [x] Parser rejects invalid version, missing fields, invalid fields, and unsupported kinds

## 2. Program Registry and Discovery

- [x] `/bin/*` discovery through the storage API
- [x] Program metadata includes name, source path, kind, entry, and description
- [x] Default utilities seeded as executable manifests
- [x] Malformed `/bin` records are skipped without panics

## 3. File-Backed Run Path

- [x] `run_program()` resolves stored programs before dispatch
- [x] Built-in entry dispatch preserved for `echo`, `time`, `sysinfo`, and `fsinfo`
- [x] Launch success/failure accounting
- [x] Program launches create/terminate process records through existing process metadata

## 4. Shell, Syscalls, and Observability

- [x] Shell commands: `programs`, `bin list`, `bin info <program>`
- [x] Program count, launch count, and failed launch count syscalls
- [x] `fsinfo` reports program count
- [x] Covered by boot gate `loader_security` (`AresOS-BootGate: name=loader_security ok=true`)

## 5. Validation

- [x] `scripts/gate/boot.py --phase 9` for QEMU-backed validation
- [x] `scripts/validation_matrix.py` includes `boot-gate-check`
- [x] Integration tests cover parser, discovery, run path, malformed files, and loader syscalls

## Validation

```bash
cargo check -p kernel
python scripts/gate/boot.py --phase 9 --timeout 180
python scripts/validation_matrix.py --smoke-timeout 180
```

See [VALIDATION_GATES.md](VALIDATION_GATES.md).


## Known Limits

- Phase 9 manifests map stored program files to existing built-in entry targets.
- Real ELF parsing, relocation, paging isolation, and raw binary execution are deferred.
- Program permissions, signatures, ownership, and executable memory protections are deferred.
