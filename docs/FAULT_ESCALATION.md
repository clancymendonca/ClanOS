# Fault Escalation Model

```yaml
status: superseded-by: docs/architecture/FAULT_ESCALATION.md
semantics_version: 1.1.0
```

> **Canonical:** [`docs/architecture/FAULT_ESCALATION.md`](architecture/FAULT_ESCALATION.md). This flat copy retained until migration squash reconciles any differences.

Three-tier fault model for services, processes, and kernel paths. Aligns with [`TEMPORAL_SEMANTICS.md`](TEMPORAL_SEMANTICS.md) authority checkpoints.

See: [`KERNEL_OBJECT_MODEL.md`](KERNEL_OBJECT_MODEL.md), [`SCHEDULER_MODEL.md`](SCHEDULER_MODEL.md), [`ERROR_TAXONOMY.md`](ERROR_TAXONOMY.md).

---

## Tiers

| Tier | Name | Typical response |
|------|------|------------------|
| 1 | Recoverable | Retry, shed load, return transient error |
| 2 | Service restart | Restart service; generation bump; R-cascade-revoke on delegation chain |
| 3 | Hard terminate / halt | Process or system halt; R-destroy-notify; audit flush attempt |

---

## Authority checkpoints

Revocation and destruction effects apply at the next **authority checkpoint**: syscall return, cap operation completion, endpoint wait completion, unless **hard revoke** is specified.

**R-revoke-blocked:** thread blocked in syscall on cap C — when C revoked, wait ends with **terminal error** at checkpoint.

---

## R-cascade-revoke vs R-destroy-notify

| Property | Scope | Effect |
|----------|-------|--------|
| **R-cascade-revoke** | Delegation chain only | Parent revoke invalidates subtree at checkpoint (depth-first) |
| **R-destroy-notify** | Object lifecycle teardown | All independent cap holders get terminal at checkpoint |

Third-party holders of the same object are unaffected by single-cap revoke; object destruction notifies **all** holders.

**R-destroy-notify delivery:** see `DECISION_LOG.md#r_destroy_notify_ordering` — simultaneous (default recommendation) or serialized order.

---

## In-flight messages

Messages carrying revoked caps: **one** documented outcome — intercepted or terminal error per T-* spec. No double-use.

---

## Kernel panic

Unrecoverable kernel fault: audit flush attempt → halt/reboot/spin in QEMU. Maps to tier 3 or explicit panic class; observable via serial smoke test.

---

## Fault handler under exhaustion

Tier-2 handlers use **reserved MEM_BUDGET partition**. If exhausted → unconditional tier 3 (no tier-2 retry loop).

---

## Audit write failure

Security policy choice (epoch 1): tier-3 halt **or** tier-2 escalation with in-memory counter + named error. See `DECISION_LOG.md#audit_tamper_policy`.

---

## Pre-restart IPC notification

Tier-3 halt/reboot: deliver terminal to active IPC callers before halt (max timeout) **or** callers use own timeout as terminal equivalent (QEMU-era policy — document choice).

---

## Suspend vs checkpoint

| State | Scope 150 |
|-------|---------------|
| **Suspend** (scope 147) | Frozen-in-memory; no persistent checkpoint |
| **Checkpoint/restore** | Out of scope until post-150 |

### Suspend/resume policy

| Survives suspend? | Policy |
|-------------------|--------|
| Generation counters | Yes (in-memory) |
| Cap table contents | No checkpoint persistence |
| Pending audit events | Security partition **non-droppable** — flush before suspend completes |
| Monotonic clock | Reset on resume (QEMU skew); suspend-boundary marker in audit |
| IPC in-flight | Terminal or cancel per R-revoke-blocked |

### Suspend flush protocol

Kernel attempts audit flush with **max timeout** (default **2s** QEMU era).

**Adopted (DECISION_LOG `#suspend_flush_timeout`):** on timeout → **hard terminate (tier 3)**. Never suspend with undrained security-partition audit queue.

Hard terminate applies defined cap table teardown, final audit flush attempt, process-hierarchy effects, and `R-destroy-notify` where applicable — not an ad-hoc fault path.

---

## Post-150 dormancy

Deferred threat surface when `architecture_state.toml` `has_persisted_cap_state = true`:

- Serialized memory images expose cap state on disk
- Generation persistence across power cycles
- Device state reconstruction

Re-evaluate before checkpoint work. Threat node: `T-checkpoint-persisted-caps`.
