# Native Model (Post-100)

Clan OS after scope 100 is a **formally governed post-Unix capability system** — semantic constitutionalism, not “Linux but smaller.”

**Central truth:** preserving **semantic coherence across decades** is harder than building the kernel.

See: [AXIOMS.md](AXIOMS.md), [ROADMAP_POST100.md](ROADMAP_POST100.md), [KERNEL_OBJECT_MODEL.md](KERNEL_OBJECT_MODEL.md).

---

## Post-Unix capability civilization

| Pillar | Characteristic |
|--------|----------------|
| Authority | No ambient authority; explicit transfer; amplification exceptional (A1) |
| Identity | Immutable object identity + generation invalidation |
| IPC | Endpoints; semantics are platform ABI (`clan-semantics-v*`) |
| Runtime | Async-native OS contract; language-neutral [ABI_RUNTIME.md](ABI_RUNTIME.md) |
| Scheduling | Service-centric; actor-like mailboxes (compat remains process-centric) |
| Governance | Axioms, G1–G5, spec cases, jurisdiction, lint |
| Compat | ELF / FD / path substrate — **not truth** (A5) |

---

## What “native” means

| Contract | Document |
|----------|----------|
| Native binary / load | `clan-native-v1` (future `clan-bin`; ELF = compat) |
| Native runtime ABI | [ABI_RUNTIME.md](ABI_RUNTIME.md) |
| Native manifest | Permissions, caps requested, service declarations |
| Native service model | Restartable platform services ([SEMANTIC_JURISDICTION.md](SEMANTIC_JURISDICTION.md)) |
| Native permission system | Manifest + permission broker |
| Native IPC | [ABI_IPC.md](ABI_IPC.md), [ABI_ASYNC.md](ABI_ASYNC.md) |
| Native handles | [KERNEL_OBJECT_MODEL.md](KERNEL_OBJECT_MODEL.md) |

**Legacy-ish (compat):** ELF, POSIX (future shim), FD table, path strings, compat syscall IDs in [ABI_SYSCALL.md](ABI_SYSCALL.md).

---

## Constitutional semantic hierarchy

Lower layers constrain upper layers — prevents circular contradictions.

```text
AXIOMS (constitutional)
  → Object ontology (KERNEL_OBJECT_MODEL)
  → Rights / authority calculus (RIGHTS_ALGEBRA)
  → Temporal visibility (TEMPORAL_SEMANTICS)
  → IPC / endpoints (ABI_IPC)
  → Runtime (ABI_RUNTIME, ABI_ASYNC)
  → Scheduler (SCHEDULING_UNIFIED — post-150)
  → Meta-semantics / precedence (TEMPORAL_SEMANTICS § Meta — post-150 full table)
```

---

## Native vs compat (paths)

| Native | Compat |
|--------|--------|
| No global FS namespace visibility | Paths via `OpenFile`, CWD |
| Storage via capability grants + broker `FsNode` | Path strings in syscalls |
| No unrestricted `/` | Ambient discovery allowed |

Paths are **broker-mediated**, **compat-only**, or **developer-facing labels** ([NATIVE_DEVELOPER_EXPERIENCE.md](NATIVE_DEVELOPER_EXPERIENCE.md)) — not kernel ambient authority.

---

## Core dangers

| Danger | Defense |
|--------|---------|
| **Semantic explosion** (cancel × revoke × async × sched × restart × borrow) | Hierarchy, laws, spec cases, meta-semantics (later) |
| **Semantic inflation** (subsystem doctrine sprawl) | A10 minimization |
| **Governance complexity** (framework too heavy) | Protect simplicity; jurisdiction; lint |

---

## Four system layers (scope 150 review)

| Layer | Trust | Contains |
|-------|-------|----------|
| **Kernel** | Minimal TCB | Objects, caps, schedule, VM, syscall dispatch |
| **Platform** | OS identity | Brokers, identity epochs, immutable image |
| **Runtime** | App ecosystem | Native ABI, async executor, language adapters |
| **Compat** | Legacy | ELF, FD, paths, POSIX shim |

---

## Philosophy before implementation

Scopes **111+** are blocked until scope **110** signs off gates G1–G5 ([AXIOMS.md](AXIOMS.md)).

Capabilities are the **system language** (resource, authority, IPC, scheduling) — not permission flags.

---

## Identity separation (local; federation deferred)

| Identity | Purpose |
|----------|---------|
| Machine | Hardware / install instance |
| User | Account principal |
| App | Sandboxed application instance |
| Service | Broker / daemon instance |
| Update | Atomic system revision / rollback epoch |

Federation (multi-device trust, sync) is deferred beyond scope 150 — see [ROADMAP_POST100.md](ROADMAP_POST100.md) § Beyond 150.
