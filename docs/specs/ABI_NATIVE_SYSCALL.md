# Native Syscall ABI (G4)

```yaml
status: authoritative
semantics_version: 1.0.0
epoch: 0
authored_by: migration
```

```yaml
status: authoritative
semantics_version: 1.0.0
```

Gate **G4** — phase **128** mandatory `ares-native-v1` manifest before ring-3 native enforcement.

---

## ID range

Native syscalls: **256+** (`governance::NATIVE_SYSCALL_ID_BASE`).

| ID | Name | Phase |
|----|------|-------|
| 256 | CapCreate | 112 |
| 257 | CapClose | 112 |
| 258 | CapTransfer | 112 |

Ring-3 allowlist expansion gated on valid `ares-native-v1` manifest (phase 128).

---

## V-01 validation

Manifest must declare requested caps ⊆ broker-granted caps. Violation → load rejected, no allowlist expansion.

See `native_manifest.rs` and NATIVE_MODEL.md.

---

## State machine

```mermaid
stateDiagram-v2
    [*] --> Active
    Active --> [*]
```

