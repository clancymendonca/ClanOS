```yaml
status: superseded-by: docs/architecture/FORMAL_MODEL.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/architecture/FORMAL_MODEL.md`](architecture/FORMAL_MODEL.md). This flat copy retained until migration squash reconciles any differences.

# Formal Model (Tier D)

```yaml
status: ratified-stub
semantics_version: 0.2.0-epoch13
framework: selective-verus
```

Tier D formal semantics — epoch 13 graduation. Framework decision: **selective Verus** on cap transfer + checkpoint paths; Kani tier B elsewhere per [PROOF_COVERAGE.md](PROOF_COVERAGE.md).

Prereq satisfied: `formal_semantics_framework_decision` recorded in epoch-13 signoff.

---

## Scope (planned)

- Cap transfer / delegation state machine
- Generation invalidation across power cycle (checkpoint epoch 13)
- Selective Verus paths per [PROOF_COVERAGE.md](PROOF_COVERAGE.md) triggers
