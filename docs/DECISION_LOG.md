# Architecture Decision Log

```yaml
status: authoritative
semantics_version: 1.0.0
```

Records alternatives considered, rationale, and epoch. **Routine decisions** are encouraged but non-gating.

**Gated decisions** (see `phase_checklist_schema.toml` `[required_decisions]`): a `DECISION_LOG` entry is **required before phase close** for the epoch where the decision is made.

---

## Entry format

```markdown
### <id> — <title> (epoch N)

**Status:** proposed | accepted | superseded
**Alternatives:** ...
**Decision:** ...
**Rationale:** ...
**Consequences:** ...
```

---

## Gated decisions (templates — resolve before listed phase)

### scheduler_priority_inversion — Scheduler priority inversion policy (epoch 0)

**Status:** proposed  
**Alternatives:** priority inheritance; priority ceiling; explicit denial (no inheritance)  
**Decision:** TBD — document in `SCHEDULER_MODEL.md`  
**Rationale:** Cap-chain blocking must not cause unbounded priority inversion without named policy  
**Consequences:** Broker and IPC paths depend on chosen model  

### r_destroy_notify_ordering — R-destroy-notify delivery ordering (epoch 0)

**Status:** proposed  
**Alternatives:** simultaneous terminal to all holders; named serialized order  
**Decision:** TBD — default recommendation: simultaneous (no order guarantee)  
**Rationale:** Holders cannot re-query destroyed object; ordering affects fairness only  
**Consequences:** Kani harness scope differs by choice  

### mint_vs_delegation_authority — Mint vs delegation authority (epoch 0)

**Status:** proposed  
**Alternatives:** named mint authority role higher than delegate; all caps from kernel root mint only  
**Decision:** TBD — document in `KERNEL_OBJECT_MODEL.md`  
**Rationale:** Silent conflation enables amplification paths  
**Consequences:** Broker and bootstrap ceremony depend on model  

### cap_reference_cycle_policy — Cap reference cycle policy (epoch 0)

**Status:** proposed  
**Alternatives:** permit with unordered teardown + timeout; forbid at kernel-object level  
**Decision:** TBD  
**Rationale:** Mutual service caps create teardown ordering hazards  
**Consequences:** Service loader and restart semantics  

### wait_set_revocation_policy — Partial vs all-terminal wait-set revocation (epoch 0)

**Status:** proposed  
**Alternatives:** (a) partial return — terminal for revoked caps, valid for live; (b) entire wait terminal  
**Decision:** TBD — document in `SCHEDULER_MODEL.md`  
**Rationale:** `select`/poll-equivalent multi-cap wait behavior  
**Consequences:** Kani coverage required before epoch 1 brokers  

### audit_tamper_policy — Audit tamper policy (epoch 1)

**Status:** proposed  
**Alternatives:** chain hash; privileged-write with named threat node only  
**Decision:** TBD — resolve at epoch 1 per gap #169  
**Rationale:** Forensic admissibility vs implementation cost  
**Consequences:** AUDIT_SUBSYSTEM wire format  

### driver_isolation_model — Driver isolation model (epoch 2 planning)

**Status:** proposed  
**Alternatives:** kernel TCB driver; process + device caps; hybrid  
**Decision:** TBD — `DRIVER_MODEL.md`  
**Rationale:** virtio-blk/net safety boundary  
**Consequences:** Epoch 2 driver host design  

### suspend_flush_timeout — Suspend flush timeout behavior (epoch 0)

**Status:** proposed  
**Alternatives:** block suspend on flush timeout; hard terminate (tier 3)  
**Decision:** TBD — document in `FAULT_ESCALATION.md`  
**Rationale:** Security partition audit events must not be silently dropped  
**Consequences:** ERROR_TAXONOMY suspend paths  

### formal-semantics-framework — Formal semantics framework choice (post-150)

**Status:** proposed  
**Alternatives:** TLA+; Lean; Isabelle/HOL; other  
**Decision:** TBD — blocks Tier D formal model work per `prereq_graph.toml`  
**Rationale:** Gap #279 — document before writing formal model  
**Consequences:** `FORMAL_MODEL.md` authoring gated  

---

## Routine decisions

(Add epoch-scoped routine decisions below as they occur.)
