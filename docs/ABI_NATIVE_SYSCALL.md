```yaml
status: superseded-by: docs/specs/ABI_NATIVE_SYSCALL.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/specs/ABI_NATIVE_SYSCALL.md`](specs/ABI_NATIVE_SYSCALL.md). This flat copy retained until migration squash reconciles any differences.

# Native Syscall ABI (G4)

```yaml
status: authoritative
semantics_version: 1.0.0
```

Gate **G4** — scope **128** mandatory `clan-native-v1` manifest before ring-3 native enforcement.

---

## ID range

Native syscalls: **256+** (`governance::NATIVE_SYSCALL_ID_BASE`).

| ID | Name | Scope |
|----|------|-------|
| 256 | CapCreate | 112 |
| 257 | CapClose | 112 |
| 258 | CapTransfer | 112 |

Ring-3 allowlist expansion gated on valid `clan-native-v1` manifest (scope 128).

---

## V-01 validation

Manifest must declare requested caps ⊆ broker-granted caps. Violation → load rejected, no allowlist expansion.

See `native_manifest.rs` and NATIVE_MODEL.md.
