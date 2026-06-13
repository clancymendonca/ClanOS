# Dependency Policy

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

Crate acceptance by layer.

---

## Layers

| Layer | Policy |
|-------|--------|
| Kernel TCB | Zero or curated allowlist with review |
| Kernel non-TCB | Restricted allowlist |
| Build tooling | Moderate policy |
| Fuzz / test only | Permissive with audit |