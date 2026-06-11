# Protocol Changelog

```yaml
status: authoritative
semantics_version: 1.0.0
```

Per-bump rationale for protocol documents. Semver: `breaking.additive.clarification`.

---

## Epoch 0

Initial constitutional docs — no wire protocol bumps yet.

---

## Track 1 — KOM reconciliation (batch 1)

| Doc | Bump | Rationale |
|-----|------|-----------|
| `docs/architecture/KERNEL_OBJECT_MODEL.md` | 1.3.0.clarification.0 | Merge flat `docs/KERNEL_OBJECT_MODEL.md` into canonical; add TOCTOU state diagram, BrokerSession kind, implementation-phase table, cap confinement/schema sections; unify cross-references. No wire format change. |
