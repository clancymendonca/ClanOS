# Scheduler Model

```yaml
status: authoritative
semantics_version: 1.1.0
```

Minimum spec before epoch 1 brokers. Full fairness policy deferred to epoch 5 planning commit.

See: [`FAULT_ESCALATION.md`](FAULT_ESCALATION.md), [`DECISION_LOG.md`](DECISION_LOG.md).

---

## Cap handles

Scheduler operates on **cap handles**, not raw object mutation.

---

## Revoke while runnable

Caps invalid at next authority checkpoint on the task — not necessarily immediately if not at checkpoint.

---

## R-revoke-blocked

Thread blocked in long syscall/wait on cap C: when C revoked, wait ends with **terminal error** at checkpoint. Kani state machine required.

---

## Partial wait-set revocation

**Adopted (DECISION_LOG `#wait_set_revocation_policy`):** **partial return**.

When a multi-cap wait (`select`/poll-equivalent) includes revoked and live caps:

- Revoked entries return **terminal** in the result set at checkpoint
- Live entries remain waitable; wait does not whole-sale abort

Kani multi-cap wait harness required before epoch 1 brokers.

---

## R-cascade-revoke

Delegation-chain revocation only. Parent revoke invalidates subtree at checkpoint (depth-first). Not object destruction (`R-destroy-notify`).

SMP: atomicity at per-core checkpoint in QEMU era; distributed cross-core protocol is post-150 obligation.

---

## Priority ceiling

**Adopted (DECISION_LOG `#scheduler_priority_inversion`):** **priority ceiling**.

When task T holds cap C and higher-priority task H is blocked on C, T runs at priority `max(T.base, H.priority)` while C is held across the blocking syscall/wait region. Ceiling drops at checkpoint when C is released or wait completes.

No priority inheritance chains beyond this ceiling rule.

---

## Memory ordering

| Operation | Ordering |
|-----------|----------|
| Generation bump / invalidate | `SeqCst` |
| Cap table slot read/write | `AcqRel` |
| Stats / metrics on cap path | `Relaxed` (must not publish authority) |

Loom harnesses 141–142 must not assume x86 TSO only.

---

## Context switch

Document barriers relative to authority checkpoint definition. Caps held across syscall boundary documented for TOCTOU analysis.

---

## Deferred

Deadline scheduling, full fairness metrics — epoch 5.
