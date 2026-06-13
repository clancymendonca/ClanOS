```yaml
status: superseded-by: docs/process/DEPENDENCY_POLICY.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/process/DEPENDENCY_POLICY.md`](process/DEPENDENCY_POLICY.md). This flat copy retained until migration squash reconciles any differences.

# Dependency Policy

```yaml
status: authoritative
semantics_version: 1.0.0
```

Crate acceptance by layer.

---

## Layers

| Layer | Policy |
|-------|--------|
| Kernel TCB | Zero or curated allowlist with review |
| Kernel non-TCB | Restricted allowlist |
| Build tooling | Moderate policy |
| Fuzz / test only | Permissive with audit |
