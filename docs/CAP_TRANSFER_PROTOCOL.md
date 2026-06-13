```yaml
status: superseded-by: docs/specs/CAP_TRANSFER_PROTOCOL.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/specs/CAP_TRANSFER_PROTOCOL.md`](specs/CAP_TRANSFER_PROTOCOL.md). This flat copy retained until migration squash reconciles any differences.

# Capability Transfer Protocol

```yaml
status: authoritative
semantics_version: 1.0.0
```

TOCTOU state machine for tier B Kani evidence. See [`KERNEL_OBJECT_MODEL.md`](KERNEL_OBJECT_MODEL.md), [`FAULT_ESCALATION.md`](FAULT_ESCALATION.md).

---

## Atomic unit

Single syscall boundary unless documented multi-step with named intermediate states.

---

## States

| State | Source slot | Destination |
|-------|-------------|-------------|
| **Idle** | Active cap | Empty |
| **Reserved** | Consumed/pending | Pending |
| **Complete** | Empty | Active cap |

Cap must never appear in both tables simultaneously.

---

## Panic mid-transfer

Maps to `FAULT_ESCALATION` tier 3 path. Audit flush attempt before halt.

---

## Receiver acknowledgment

Explicit policy per transfer kind: required or not — document per operation in `CAP_REGISTRY.toml`.

---

## Cross-references

- Authority checkpoints: `TEMPORAL_SEMANTICS.md`
- Revoke-in-flight: `ERROR_TAXONOMY.md`
- Audit transfer events: `AUDIT_SUBSYSTEM.md` (epoch 1)
