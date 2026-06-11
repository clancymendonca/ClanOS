# Scheduler Model

```yaml
status: authoritative
semantics_version: 1.0.0
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

See `DECISION_LOG.md#wait_set_revocation_policy`:

- **(a)** Partial return — terminal for revoked caps, valid for live, **or**
- **(b)** Entire wait terminal

Kani coverage required before epoch 1 brokers.

---

## R-cascade-revoke

Delegation-chain revocation only. Parent revoke invalidates subtree at checkpoint (depth-first). Not object destruction (`R-destroy-notify`).

SMP: atomicity at per-core checkpoint in QEMU era; distributed cross-core protocol is post-150 obligation.

---

## Priority inversion

See `DECISION_LOG.md#scheduler_priority_inversion`. Choose one: inheritance, ceiling, or explicit denial.

---

## Memory ordering

Cap table ops classified SeqCst / acquire-release / relaxed. Loom harnesses 141–142 must not assume x86 TSO only.

---

## Context switch

Document barriers relative to authority checkpoint definition. Caps held across syscall boundary documented for TOCTOU analysis.

---

## Deferred

Deadline scheduling, full fairness metrics — epoch 5.
