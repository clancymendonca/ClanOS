```yaml
status: superseded-by: docs/process/DECISION_LOG.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/process/DECISION_LOG.md`](process/DECISION_LOG.md). This flat copy retained until migration squash reconciles any differences.

# Architecture Decision Log

```yaml
status: authoritative
semantics_version: 1.1.0
```

Records alternatives considered, rationale, and epoch. **Routine decisions** are encouraged but non-gating.

**Gated decisions** (see `scope_checklist_schema.toml` `[required_decisions]`): a `DECISION_LOG` entry is **required before scope close** for the epoch where the decision is made.

---

## Gated decisions (accepted)

### scheduler_priority_inversion — Scheduler priority inversion policy (epoch 0)

**Status:** accepted  
**Alternatives:** priority inheritance; priority ceiling; explicit denial (no inheritance)  
**Decision:** **Priority ceiling** — when a task holds a cap blocking a higher-priority client, the holder runs at the ceiling of blocked priorities for the duration of the critical section (cap held across syscall/wait).  
**Rationale:** Bounded latency without unbounded inheritance chains; matches broker-centric epoch 1 architecture.  
**Consequences:** `SCHEDULER_MODEL.md` § Priority ceiling; Kani target before scope 128 brokers.

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
**Consequences:** `DRIVER_MODEL.md`; VIRTIO_SAFETY.md boundary; scope 122+ driver host checklist.

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

### ADR-0003-loader-signed-exec — Loader exec manifest signing (scope 460–465)

**Status:** accepted (Q1–Q5 locked; PR1 host, PR2 kernel, anchor guard **done**)  
**Alternatives:** sidecar manifest (Q1-B); reuse epoch-450 anchor (Q2-A); hard cutover / forward-only grandfather (Q3-A/C); host-only verify  
**Decision:** Extend **`clan-exec-v1`** with optional `sig=ed25519:` and a **distinct** canonical signed body from ADR-0002 `clan-signed-manifest-v1` (shared syntax, different bytes signed). Separate **`trust_anchor_epoch460_loader.toml`** — never epoch-450 gate seed. Trust classes: `trust=system` digest-only **only** on explicit `config/loader_digest_only_allowlist.toml`; `trust=system-signed` requires epoch-460 sig (kernel, fail closed). Sunset: **`sunset_scope = 465`** enforced by `scripts/gate/loader_signing_sunset_check.py` + `architecture_state.toml` `loader_digest_only_grace` — non-empty allowlist after scope 465 fails CI (same hard-deny shape as `has_external_network`).  
**Rationale:** Closes ADR-0002 deferral without conflating gate-corpus proof with loader trust; public dev seed cannot become loader root; prevents digest-only exception class calcifying like gap-registry stubs.  
**Consequences:** [`docs/architecture/ADR/ADR-0003-loader-signed-exec-manifests.md`](architecture/ADR/ADR-0003-loader-signed-exec-manifests.md); gate `2.6.0`; seed migration **complete** (scope 465 closed).

### ADR-0003-seed-migration — Seed `/bin/*` signing rollout (scopes 461–465)

**Status:** accepted (2026-06-21)  
**Alternatives:** batch-sign all seed binaries then empty allowlist; one-way signed cutover without digest fallback  
**Decision:** **One program per commit/PR.** Allowlist is **rollback staging**: on bad sign/manifest, revert to `trust=system` and re-add `name=` to allowlist. Remove from allowlist only after host verify + applicable QEMU gate smokes pass for that binary. Progress = **`len(allowlist)`** countdown to scope 465 — not a single cutover event.  
**Rationale:** First migration touches live dependencies; staged rollback beats unbootable batch failure.  
**Consequences:** ADR-0003 § Seed migration workflow; `loader_digest_only_allowlist.toml` header comments. **Closed 2026-06-21:** 16/16 signed; scope 465 (`loader_digest_only_grace=false`).

### ADR-0003-hello-exempt — `/bin/hello` outside seed migration inventory

**Status:** accepted (2026-06-21)  
**Alternatives:** migrate `hello` as 17th seed (`trust=system-signed`); leave undocumented  
**Decision:** **`/bin/hello` is intentionally exempt** from ADR-0003 seed migration. It remains `trust=user` — a hardware ELF validation fixture on the name allowlist, not an admin system seed. Loader signing inventory documented in `GATE_AUDIT_401_500.md` § Scope honesty.  
**Rationale:** Seed migration scope was the 16 `trust=system` allowlist programs; `hello` serves ADR-0002 / HW bring-up with a different trust class. Promoting it requires ADR amendment + `execute_minimal_user_elf_descriptor` verify wiring.  
**Consequences:** Gate audit honesty row; ADR Q4 exempt row unchanged until deliberate revisit.

---

## Routine decisions

(Add epoch-scoped routine decisions below as they occur.)
