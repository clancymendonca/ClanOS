```yaml
status: superseded-by: docs/architecture/SCHEDULING_UNIFIED.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/architecture/SCHEDULING_UNIFIED.md`](architecture/SCHEDULING_UNIFIED.md). This flat copy retained until migration squash reconciles any differences.

# Scheduling Unified Model

```yaml
status: authoritative
semantics_version: 0.1.0-draft
```

Post-150 service-centric scheduler. Extends [SCHEDULER_MODEL.md](SCHEDULER_MODEL.md). Epoch 8 deliverable (phases 176–200).

---

## S-* spec cases (draft)

| ID | Property |
|----|----------|
| S-01 | Endpoint-driven wake: runnable service blocked only on cap-mediated wait |
| S-02 | E-00 priority ceiling under cap chains |
| S-03 | Revoke while runnable: R-revoke-blocked at checkpoint |
| S-04 | Partial wait-set revocation |
| S-05 | AP bring-up gated on loom harness pass |

Implementation: [kernel/src/service_scheduler.rs](../kernel/src/service_scheduler.rs).
