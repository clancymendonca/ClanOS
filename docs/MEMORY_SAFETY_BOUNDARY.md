```yaml
status: superseded-by: docs/architecture/MEMORY_SAFETY_BOUNDARY.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/architecture/MEMORY_SAFETY_BOUNDARY.md`](architecture/MEMORY_SAFETY_BOUNDARY.md). This flat copy retained until migration squash reconciles any differences.

# Memory Safety Boundary

```yaml
status: authoritative
semantics_version: 1.0.0
```

Crate/module `unsafe` allow map. Directory-level CI rules.

---

## TCB crates

Kernel core: `unsafe` per [`UNSAFE_AUDIT.md`](UNSAFE_AUDIT.md). Count reported in `STATUS.md` by module.

---

## Policy

No new `unsafe` in TCB without second reviewer. No recursion beyond documented stack depth policy.
