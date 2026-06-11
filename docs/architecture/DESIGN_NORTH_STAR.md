# Design North Star

```yaml
status: authoritative
version: 0.1.0
epoch: 0
authored_by: architecture
```

Non-negotiable design principles for AresOS. Deviations require charter amendment and ADR.

---

## Kernel architecture

**Capability-secured hybrid microkernel** with verified TCB (~50k LOC target in Rust):

- **In kernel TCB:** scheduler, capability system, memory management, IPC primitives, fault escalation.
- **Out of kernel:** device drivers, filesystems, network stack, graphics, AI — isolated server processes.
- **HAL:** thin `arch/` layer for ISA differences (x86_64 primary).

Rationale: seL4-class isolation, Linux-class driver crash containment, tractable formal verification of critical paths.

---

## Authorization

- Object-capability model only in kernel.
- No ambient authority anywhere.
- DAC exists only in POSIX compat server.

---

## IPC

- Synchronous register-passing (≤4 words) for small messages — zero allocation fast path.
- Asynchronous shared-memory ring buffers for bulk — capability-mediated DMA-safe regions.
- Every endpoint is a kernel object; no global namespace without explicit delegation.

---

## Scheduler

- Hierarchical earliest-deadline-first with capability-secured priority assignment.
- Priority ceiling protocol for inversion (see `DECISION_LOG.md` sched-001).
- CPU partitions as first-class capabilities.
- AI workload class (distinct partition semantics post-M400).

---

## Memory

- Buddy physical allocator with typed regions.
- VSpace and MemoryRegion capabilities for all mappings.
- Copy-on-write, demand paging, NUMA-aware stubs for server workloads.

---

## Filesystem

- **AresFS** native: CoW B-trees, crash consistency without journaling.
- Path-scoped FsNode capabilities — no ambient traversal.
- POSIX compat as untrusted translation layer above AresFS.

---

## Networking

- Async stack as server process; drivers isolated.
- Socket capabilities with protocol/address scope.
- QUIC primary for native apps; TCP/UDP via compat server.

---

## Kind semantics freeze

Once a cap kind graduates the never-stabilize list (`never_stabilize_graduated.toml`), semantics are **frozen**. Reinterpretation requires a **new kind**, not field reuse.

---

## Evidence discipline

Never claim tier A or B coverage is "proven correct." Document harness bounds explicitly. Tier D (TLA+) deferred for liveness post-milestone-150.

---

## Cross-references

- [SECURITY_MODEL.md](SECURITY_MODEL.md)
- [KERNEL_OBJECT_MODEL.md](KERNEL_OBJECT_MODEL.md)
- [../../CHARTER.md](../../CHARTER.md)
- [../../STATUS.md](../../STATUS.md)
