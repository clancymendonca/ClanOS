```yaml
status: superseded-by: docs/specs/WIRE_SCHEMA_REGISTRY.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/specs/WIRE_SCHEMA_REGISTRY.md`](specs/WIRE_SCHEMA_REGISTRY.md). This flat copy retained until migration squash reconciles any differences.

# Wire Schema Registry

```yaml
status: authoritative
semantics_version: 1.0.0
```

Versioned binary schemas for audit, errors, IPC, and cap serialization.

---

## Registry

| Schema id | Version | Document | Status |
|-----------|---------|----------|--------|
| `error.v1` | 1 | ERROR_TAXONOMY.md | epoch 1 stub |
| `audit.v1` | 1 | AUDIT_SUBSYSTEM.md | epoch 1 — chain hash |
| `ipc.interim.v1` | 1 | IPC_INTERIM_BRIDGE.md | scopes 122–133 |
| `cap.wire.v1` | 1 | KERNEL_OBJECT_MODEL.md | stub |
| `oom.shed.stub.v1` | 1 | scope 121 | never-stabilize |

Each record carries `error_schema_version` or equivalent on wire.

---

## Compat matrix

| Consumer | error.v1 | ipc.interim.v1 |
|----------|----------|----------------|
| Brokers 122–126 | required | required |
| Native endpoints 134+ | required | N/A |
