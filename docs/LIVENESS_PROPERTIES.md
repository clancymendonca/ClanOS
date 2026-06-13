```yaml
status: superseded-by: docs/proofs/LIVENESS_PROPERTIES.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/proofs/LIVENESS_PROPERTIES.md`](proofs/LIVENESS_PROPERTIES.md). This flat copy retained until migration squash reconciles any differences.

# Liveness Properties

```yaml
status: authoritative
semantics_version: 1.0.0
```

Safety vs liveness split. Tier D formal models post-150.

---

## Documented liveness obligations

- IPC cancel must not block on saturated queue
- Cap quota release-retry terminates
- Suspend flush timeout bounded

---

## Out of scope pre-150

Full temporal logic proofs — pointer to `FORMAL_MODEL.md` when framework decision recorded.
