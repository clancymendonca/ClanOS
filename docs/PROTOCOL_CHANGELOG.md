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
| `docs/architecture/KERNEL_OBJECT_MODEL.md` | 1.3.0.additive.0 | **Additive:** sections absent from pre-reconcile canonical — phase-110 design decision, universal interface table, G1 handle semantics list, generation invalidation, full mint/delegation authority, cap schema version, confinement, kind freeze, implementation-phase table, BrokerSession kind row, TOCTOU diagram (implementation-verified). **Clarification:** merged wording for reference cycles, bootstrap ceremony, R-destroy-notify where canonical already had shorter forms. No wire format change. |
