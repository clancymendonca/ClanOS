# ABI and Semantic Stability

Two independent version surfaces — **semantics are platform ABI** ([AXIOMS.md](AXIOMS.md) A9).

---

## Syscall / register ABI — `clan-abi-v1`

| Property | Value |
|----------|--------|
| Status | Frozen at scope 101 for compat IDs 1–82 |
| Document | [ABI_SYSCALL.md](ABI_SYSCALL.md) |
| Break | Syscall number change, register meaning change, allowlist change without compat bump |

Bump to `clan-abi-v2` requires: migration notes, compat shim period, matrix smoke updates.

---

## Semantic laws — `clan-semantics-v1` (draft at scope 109)

| Property | Value |
|----------|--------|
| Status | Draft ratified scope 110 |
| Documents | [RIGHTS_ALGEBRA.md](RIGHTS_ALGEBRA.md), [ABI_IPC.md](ABI_IPC.md), [ABI_ASYNC.md](ABI_ASYNC.md), [TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md) |
| Break examples | Endpoint ordering class change; revoke visibility timing change; cancel propagation change; ownership move rules change |

Bump to `clan-semantics-v2` requires:

1. [SEMANTIC_SPECS.md](SEMANTIC_SPECS.md) case updates
2. [SEMANTIC_LINT.md](SEMANTIC_LINT.md) CI pass (scopes 159–160+)
3. Minimization audit (A10)
4. `clan-semantics-v*` noted in release notes

---

## Deprecation process

| Step | Action |
|------|--------|
| 1 | Propose change with law diff + spec case diff |
| 2 | A10 minimization review |
| 3 | Jurisdiction check ([SEMANTIC_JURISDICTION.md](SEMANTIC_JURISDICTION.md)) |
| 4 | Version bump if observable guarantee changes |
| 5 | Implement with executable semantic tests (same case IDs) |

---

## Scope 110 sign-off

- [ ] `clan-abi-v1` compat table matches `ALLOWED_HW_SYSCALLS`
- [ ] `clan-semantics-v1` draft covers IPC + rights + temporal outlines
- [ ] No native law hidden in compat-only docs without `compat-scope` tag
