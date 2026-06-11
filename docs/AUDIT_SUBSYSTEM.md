# Audit Subsystem

```yaml
status: authoritative
semantics_version: 1.0.0
```

Epoch 1 implementation prereq. Epoch 0 positions documented.

---

## Epoch 0 positions

- Kernel-only write path (threat node `T-audit-tamper`)
- Tamper policy: `DECISION_LOG.md#audit_tamper_policy` — resolve epoch 1
- Bootstrap unaudited window scoped explicitly at implementation
- Forensic admissibility assumptions in `DESIGN_NORTH_STAR.md`

---

## Deferred

Overflow policy, read cap, versioned binary schema, IPC correlation — epoch 1 prereq.
