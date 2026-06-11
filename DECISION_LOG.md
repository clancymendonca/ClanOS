# AresOS Architecture Decision Log

```yaml
status: authoritative
version: 0.1.0
epoch: 0
```

Top-level architectural decisions resolved at project inception. Phase- and epoch-specific gated decisions are recorded in [`docs/DECISION_LOG.md`](docs/DECISION_LOG.md).

---

## Kernel architecture

**Decision:** Capability-secured hybrid microkernel with verified TCB.

**Alternatives considered:** monolithic (Linux-style), pure microkernel (seL4/Minix), hybrid (NT/XNU), exokernel (MIT/Nemesis).

**Rationale:** Monolithic kernels offer performance but weak fault containment and intractable whole-kernel verification. Pure microkernels offer isolation but IPC overhead and ecosystem friction. Hybrids tend to bloat. Exokernels optimize specialized workloads at the cost of isolation and usability.

**Selected approach:** Kernel TCB limited to scheduler, capability system, memory management, and IPC primitives (~50k LOC Rust, Kani/Verus bounded verification). Drivers, filesystems, network, graphics, and AI run as isolated servers with typed capability-secured IPC. Shared-memory channels negotiated after capability handshake for hot-path performance. HAL handles ISA differences.

**Consequences:** seL4-class isolation, driver crash containment, tractable verification scope, structured IPC with managed overhead.

**Security implications:** Small TCB reduces attack surface; capability model is sole authorization in kernel.

**Verification:** Tier B Kani on cap transfer, rights algebra, generation, revocation; tier C Verus selective for intractable paths.

---

## Scheduler

**Decision:** Hierarchical earliest-deadline-first (EDF) with capability-secured priority assignment and priority ceiling protocol for inversion.

**Alternatives:** fixed-priority preemptive, CFS-style fair queue, priority inheritance.

**Rationale:** EDF provides optimal uniprocessor scheduling for deadline tasks; ceiling protocol bounds inversion without inheritance chains. CPU time is a capability-managed resource, not ambient.

**Consequences:** `docs/SCHEDULER_MODEL.md`; partition capabilities for real-time and AI workload classes.

**Verification:** R-revoke-blocked Kani state machine; loom tests for SMP ordering (phases 141–142).

**Reference:** `docs/DECISION_LOG.md` entry `scheduler_priority_inversion`.

---

## Memory allocator

**Decision:** Capability-secured two-level allocator — buddy system for physical frames with typed memory regions; VSpace and MemoryRegion capabilities for virtual mappings.

**Alternatives:** slab-only, zone allocator, single-level buddy.

**Rationale:** Buddy system provides predictable coalescing for physical memory; typed regions enable IOMMU/DMA policy. Capability mediation eliminates ambient memory access.

**Consequences:** `docs/FRAME_OWNERSHIP.md`, `docs/USER_PAGE_TABLES.md`; NUMA-aware stubs for server workloads.

---

## IPC model

**Decision:** Synchronous register-passing for small messages (≤4 machine words); asynchronous capability-mediated shared-memory ring buffers for bulk.

**Alternatives:** pure message-passing, pure shared memory, Unix-domain-socket semantics in kernel.

**Rationale:** Register-passing achieves zero-allocation bounded latency for control plane. Shared memory with capability gates solves bulk data without memcpy on hot paths.

**Consequences:** `docs/ABI_IPC.md`, `docs/CAP_TRANSFER_PROTOCOL.md`; wire schema versioned in registry.

**Verification:** IPC reply rights bound (tier B); compat-internal bridge retired at M400.

---

## Rights attenuation

**Decision:** Monotone-decreasing rights through delegation and attenuation.

**Invariant:** `attenuate(attenuate(r, m1), m2) == attenuate(r, m1 | m2)`.

**Alternatives:** policy-based ACL overlay, role-based ambient grants.

**Rationale:** Monotone algebra is verifiable and prevents silent privilege growth.

**Verification:** Tier A proptest + tier B Kani before any delegation code ships.

---

## Fault escalation

**Decision:** Three-tier fault model.

| Tier | Behavior |
|------|----------|
| 1 | Transient, retryable; no state change |
| 2 | Service crash; caps to crashed service terminal; callers notified; restart with new identity |
| 3 | Kernel fault; audit flush attempted; system halt/reboot; pre-restart IPC terminal notifications |

**Consequences:** `docs/FAULT_ESCALATION.md`; suspend flush timeout maps to tier 3.

---

## Security model

**Decision:** Object-capability model as sole kernel authorization. No DAC in kernel.

**Consequences:** `docs/architecture/SECURITY_MODEL.md`, `docs/THREAT_NODES.toml`.

---

## Open items

### kernel-002 — clippy lint enforcement pending

**Status:** open  
**Context:** `#![deny(clippy::all)]` not enabled on kernel crate. Clippy on a 1700+ line kernel without per-lint review would obscure Track 1 doc commits.  
**Reopen trigger:** first implementation phase commit after Track 1 squash gate.  
**Resolution:** Dedicated phase commit with clippy allowlist reviewed per-lint; full Kani re-run.

---

### ares-rt-001 — ares-rt `no_std` enforcement pending

**Status:** open  
**Context:** Workspace `cargo check` fails on the host target because `ares-rt` (`userland/`) does not declare `#![no_std]`. The crate is built for `x86_64-unknown-none` in the OS context; `cargo check -p kernel` passes. This is documentation and build-matrix debt, not a kernel soundness issue.  
**Reopen trigger:** `architecture_state.toml` → `has_no_std_enforcement = false` (CI-readable; must flip to `true` only after enforcement lands).  
**Resolution:** Dedicated phase commit adding `#![no_std]` to `ares-rt`, host/workspace `cargo check` matrix update, and full in-scope Kani re-run. **Out of scope** during the doc migration epoch — do not fix opportunistically.

---

Other epoch-scoped open items: see [`docs/DECISION_LOG.md`](docs/DECISION_LOG.md).
