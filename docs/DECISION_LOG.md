# Architecture Decision Log

```yaml
status: authoritative
semantics_version: 1.1.0
```

Records alternatives considered, rationale, and epoch. **Routine decisions** are encouraged but non-gating.

**Gated decisions** (see `phase_checklist_schema.toml` `[required_decisions]`): a `DECISION_LOG` entry is **required before phase close** for the epoch where the decision is made.

---

## Gated decisions (accepted)

### scheduler_priority_inversion — Scheduler priority inversion policy (epoch 0)

**Status:** accepted  
**Alternatives:** priority inheritance; priority ceiling; explicit denial (no inheritance)  
**Decision:** **Priority ceiling** — when a task holds a cap blocking a higher-priority client, the holder runs at the ceiling of blocked priorities for the duration of the critical section (cap held across syscall/wait).  
**Rationale:** Bounded latency without unbounded inheritance chains; matches broker-centric epoch 1 architecture.  
**Consequences:** `SCHEDULER_MODEL.md` § Priority ceiling; Kani target before phase 128 brokers.

### r_destroy_notify_ordering — R-destroy-notify delivery ordering (epoch 0)

**Status:** accepted  
**Alternatives:** simultaneous terminal to all holders; named serialized order  
**Decision:** **Simultaneous** — all holders receive terminal at the same authority checkpoint; **no ordering guarantee** among holders.  
**Rationale:** Destroyed object is unqueryable; fairness ordering adds complexity without security benefit.  
**Consequences:** Dedicated Kani harness; cross-instance same-kind operations explicitly out of scope.

### mint_vs_delegation_authority — Mint vs delegation authority (epoch 0)

**Status:** accepted  
**Alternatives:** named mint authority role higher than delegate; all caps from kernel root mint only  
**Decision:** **Kernel root mint only** for QEMU era — every cap traces to bootstrap ceremony or an auditable broker mint path from root. Delegate and attenuate never create authority ex nihilo.  
**Rationale:** Eliminates silent mint/delegate conflation; simplest invariant for epoch 1 brokers.  
**Consequences:** `KERNEL_OBJECT_MODEL.md` § Mint authority; threat node on bootstrap scope creep.

### cap_reference_cycle_policy — Cap reference cycle policy (epoch 0)

**Status:** accepted  
**Alternatives:** permit with unordered teardown + timeout; forbid at kernel-object level  
**Decision:** **Permitted** with **unordered teardown + 5s default timeout** at service restart/teardown. Cycle detected at restart → all cycle participants enter Teardown; caps terminal at checkpoint if timeout expires.  
**Rationale:** Mutual service dependencies are practical; forbidden cycles push complexity to userland without kernel enforcement benefit.  
**Consequences:** Service loader restart path; FAILURE_MODES_LEDGER entry.

### wait_set_revocation_policy — Partial vs all-terminal wait-set revocation (epoch 0)

**Status:** accepted  
**Alternatives:** (a) partial return; (b) entire wait terminal  
**Decision:** **(a) Partial return** — revoked caps return terminal in wait result set; live caps remain waitable.  
**Rationale:** Matches `select`/poll semantics expected by compat and native IPC migration.  
**Consequences:** Kani multi-cap wait harness required before epoch 1 brokers.

### audit_tamper_policy — Audit tamper policy (epoch 1 target)

**Status:** accepted  
**Alternatives:** chain hash; privileged-write with named threat node only  
**Decision:** **Chain hash** — each audit record includes hash of prior record; kernel-only append; verification on read-cap export. Privileged-write without chain is rejected at epoch 1 implementation.  
**Rationale:** Forensic admissibility and `T-audit-tamper` closure path.  
**Consequences:** `AUDIT_SUBSYSTEM.md` wire schema epoch 1; implementation gate for epoch 1.

### driver_isolation_model — Driver isolation model (epoch 2)

**Status:** accepted  
**Alternatives:** kernel TCB driver; process + device caps; hybrid  
**Decision:** **Hybrid** — kernel provides MMIO/IRQ trampoline and DMA mapping gates; **userspace driver host** holds `device.*` caps and virtio protocol stack.  
**Rationale:** Minimizes TCB while meeting virtio-blk/net epoch 2 schedule; aligns with GpuContext/userspace driver pattern.  
**Consequences:** `DRIVER_MODEL.md`; VIRTIO_SAFETY.md boundary; phase 122+ driver host checklist.

### suspend_flush_timeout — Suspend flush timeout behavior (epoch 0)

**Status:** accepted  
**Alternatives:** block suspend on flush timeout; hard terminate (tier 3)  
**Decision:** **Hard terminate (tier 3)** on flush timeout — never enter suspend with undrained security-partition audit queue.  
**Rationale:** Audit drop on suspend is unacceptable; blocking suspend indefinitely is a host DoS.  
**Consequences:** `FAULT_ESCALATION.md` § Suspend flush; ERROR_TAXONOMY system class mapping.

### formal-semantics-framework — Formal semantics framework choice (post-150)

**Status:** accepted  
**Alternatives:** TLA+; Lean; Isabelle/HOL; other  
**Decision:** **TLA+** for Tier D stub models post-150; machine-checked proofs remain Lean/Verus-selective if escalated.  
**Rationale:** Industry norm for distributed/concurrency protocol stubs; aligns with plan Tier D pointer.  
**Consequences:** `prereq_graph.toml` gate satisfied; `FORMAL_MODEL.md` authoring unblocked post-150.

---

## Routine decisions

(Add epoch-scoped routine decisions below as they occur.)
