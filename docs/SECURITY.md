# Security Model

Scope 10 adds policy groundwork, not hardware-enforced isolation. The kernel now has static credentials, file permissions, executable trust metadata, and process ownership checks that future ELF loading and address-space isolation can build on.

## Identity

The security module defines four roles:

- `Kernel`: boot and internal kernel ownership.
- `Admin`: privileged management identity.
- `User`: default shell identity.
- `Guest`: read-biased low-trust identity for validation and future sessions.

The default shell starts as `UserId(100)` with role `User`. `su admin`, `su user`, and `su guest` are static role switches for validation only; there is no password or multi-session login layer in this scope.

## File Policy

Each filesystem directory entry stores an owner and a three-bit mode:

- read
- write
- execute

Bootstrapped files use conservative defaults:

- `/bin/*` is admin-owned and readable/executable.
- `/README.txt` is kernel-owned and read-only.
- user-created files are owned by the current user and readable/writable.

Shell and syscall read/write/delete paths use checked storage APIs. Legacy unchecked storage helpers remain for bootstrapping and older internal tests, but user-facing paths enforce policy.

## Program Trust

`clan-exec-v1` manifests now accept:

- `requires=execute`
- `trust=system` or `trust=user`
- `owner=<name>`

The loader rejects unsupported trust or requirement values without panicking. Before dispatching a stored built-in alias, the loader checks execute permission on the manifest file and records denied launches separately from normal failed launches.

Scope 11 extends this policy to executable image manifests. `kind=elf64-image` records require execute permission on both the manifest and referenced image file before validation. The image can be parsed and described, but execution is blocked until a future scope adds executable mappings and privilege separation.

## Process Ownership

Process records now capture creator credentials. Process display includes owner role, and checked termination follows these rules:

- admin/kernel can terminate any process
- users can terminate their own processes
- users cannot terminate admin/kernel-owned processes

## Observability

The shell exposes:

- `whoami`
- `stat <path>`
- `chmod +x|-x <path>`
- `su admin|user|guest`

Syscalls expose current user, current role, denied access count, and denied execute count. Boot emits:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## Limits

Scope 10 intentionally does not provide CPU privilege separation, page-table isolation, real executable memory protections, cryptographic program signatures, groups, ACLs, or capabilities. Those are deferred until the kernel has raw ELF loading and per-process address spaces.

Scope 11 adds descriptor-only address spaces and ELF64 validation, but still does not switch page tables or run arbitrary stored code.

Scope 12 adds load-plan and reservation accounting for validated images. It still does not allocate executable user frames, mutate process page tables, switch CR3, enter Ring 3, or jump to stored ELF entry points.

Scope 13 adds deterministic mapping stubs for prepared images. These stubs record owner credentials, frame tokens, mapped pages, copy bytes, and zero-fill bytes, but they remain policy and accounting records rather than hardware-enforced user mappings.

## Trust-Gated Execution (Scope 43)

Hardware ELF launch still requires an allowlisted program name (`hello`, `exit42`, `tickprobe`) for `trust=user` manifests. Programs with `trust=system` may run through `execute_trusted_manifest_elf` without appearing on the name allowlist.

The seed manifest `/bin/systrust` references `/bin/tickprobe.elf` with `trust=system` and is used by `See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

Validation:

```bash
python scripts/gate/run.py --gate dynamic_runtime --timeout 180
```

## Manifest Digest (Scope 58)

Manifests may include `digest=sha256:<hex>` over the referenced ELF bytes. `execute_trusted_manifest_elf` verifies the digest before running `trust=system` programs. This is integrity checking only, not a public-key signature chain.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

Deferred: cryptographic signatures, capability tokens, and per-user trust policies beyond static manifest fields.

## Epoch-450 signed ELF gate corpus (ADR-0002)

The production gate pinned corpus (`config/signed_elf_test_corpus/`) uses a **public, deterministic Ed25519 development seed** in host tooling (`scripts/gate/signed_elf_lib.py`). Anyone with the repository can derive the private key. That is intentional for CI reproducibility and **must never anchor anything beyond the gate test corpus and `scripts/gate/fixtures/signed_elf/`**.

- Do **not** reuse `sign_test_corpus.py` or `epoch450_dev_private_key()` to sign real `/bin/*` payloads, loader manifests, or shipping userland.
- This is separate from `clan-exec-v1` manifest digest checks (`digest=sha256:`) — see above; ADR-0002 adds a signature layer for the gate smoke only until loader integration lands.

Wire format: [`config/signed_elf_test_corpus/WIRE_FORMAT.txt`](../config/signed_elf_test_corpus/WIRE_FORMAT.txt). Kernel verification must test against committed fixture bytes verbatim.

## Epoch-460 loader exec signing (ADR-0003 PR1)

Loader `/bin/*` signing uses a **separate** trust anchor and canonical signed body from ADR-0002:

| Item | Path |
|------|------|
| Anchor (public key only) | `config/trust_anchor_epoch460_loader.toml` |
| Wire format + golden bytes | `config/loader_signed_exec/WIRE_FORMAT.txt`, `canonical_body.utf8` |
| Host library | `scripts/gate/loader_signed_exec_lib.py` |
| Dev seed domain | `clanos-epoch460-loader-exec-signing-anchor-v1` (public, deterministic) |

- Do **not** reuse `epoch450_dev_private_key()`, `signed_elf_lib.canonical_signed_body*`, or the epoch-450 anchor for loader exec manifests.
- `trust=system-signed` requires `sig=ed25519:` verified against epoch-460 anchor; `trust=system` digest-only is allowlist-governed until scope **465** (`loader_signing_sunset_check.py`).
- Kernel `program_loader` hook is **not** part of PR1 — host fixtures and negative gauntlet first.
