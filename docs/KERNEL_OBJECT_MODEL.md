# Universal Kernel Object Model

```yaml
status: authoritative
semantics_version: 1.1.0
```

**Gate G1** — phases **115+** must not introduce new handle semantics without charter revision.

Phase **110** constitutional default: **immutable object identity + generation invalidation**.

See: [AXIOMS.md](AXIOMS.md), [RIGHTS_ALGEBRA.md](RIGHTS_ALGEBRA.md), [SEMANTIC_SPECS.md](SEMANTIC_SPECS.md) (R-03, E-03, T-02).

---

## Design decision (phase 110)

**Adopted:** each kernel object has a stable `ObjectId` and a monotonic **generation** counter. Authority changes invalidate derived capabilities via generation bump — not in-place mutation of object rights.

**Rejected for native (unless charter exception):** mutable authority containers where the same `ObjectId` silently changes rights in place. That model complicates aliasing, temporal visibility (A6), borrow/move, and meta-semantics.

---

## Universal interface (conceptual)

Not a literal Rust trait in the kernel yet — architectural contract:

| Field | Meaning |
|-------|---------|
| `ObjectId` | Stable identity for object lifetime |
| `Kind` | One of the kinds below |
| `Generation` | Invalidation epoch; bump on revoke / restart / teardown |
| `Rights` | Subset of rights for this handle (see RIGHTS_ALGEBRA) |
| `Metadata` | Kind-specific, non-authority data |

**One handle table:** `CapHandle` references `(ObjectId, Kind, Rights subset, Generation)` for any kind.

---

## Object kinds

| Kind | Role | Examples |
|------|------|----------|
| **Process** | Schedulable task + cap table + credentials | User ELF, native app |
| **Endpoint** | Async IPC port / mailbox | Native IPC (replaces pipe-as-path for native) |
| **MemoryRegion** | Cap-scoped mapping | Shared buffers, anon mappings |
| **Service** | Restartable platform instance | Storage broker, permission broker |
| **Device** | Gated hardware access | Block device cap |
| **FsNode** | Broker-mediated storage view | Not ambient path visibility |
| **GpuContext** | Compositor / GPU session | Userspace driver stack |
| **BrokerSession** | Authority delegation channel | Permission broker minting |

---

## Handle semantics (frozen at G1)

1. **Create** — mint cap with initial rights subset ≤ object’s max rights for that mint path
2. **Transfer** — move (consume sender) or borrow (time-bounded, non-delegable) per RIGHTS_ALGEBRA
3. **Delegate** — attenuate rights to new cap; no amplification (A1)
4. **Revoke** — generation bump and/or slot invalidation per TEMPORAL_SEMANTICS
5. **Close** — drop handle slot; may not destroy object if other caps exist

Phase 115 **path broker** uses compat handles only — must not add a parallel handle type.

---

## Generation invalidation

When generation increments on object `O`:

- All caps derived from `O` at older generation become invalid at documented visibility point ([TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md))
- Spec case **R-03** defines expected behavior

Triggers (non-exhaustive): hard revoke, service restart, broker session end, endpoint teardown.

---

## Implementation phases (future)

| Phase | Work |
|------:|------|
| 111 | `CapHandle` → `KernelObject` ref, single table |
| 112–113 | Lifecycle syscalls (G2) |
| 114 | Storage grant object (no paths) |
| 115 | Path broker (**compat only**) |

---

## Lifecycle states (epoch 0 extension)

Per-kind state machine (conceptual):

| State | Meaning |
|-------|---------|
| **Created** | Object allocated; caps may be minted per mint path |
| **Active** | Normal operations per cap rights |
| **Teardown** | Draining; no new grants; in-flight ops complete at checkpoint |
| **Invalidated** | Generation bumped or object destroyed; caps terminal at checkpoint |

Valid transitions: Created → Active → Teardown → Invalidated. Invalidated is terminal.

Generation increment events cross-ref [`GENERATION_COUNTER.md`](GENERATION_COUNTER.md).

---

## Mint vs delegation

See `DECISION_LOG.md#mint_vs_delegation_authority`. Either:

- Named **mint authority** role (strictly higher than delegate) with threat node, **or**
- Explicit policy: all caps originate from **kernel root mint** — no silent conflation of mint and delegate paths

---

## Reference cycles

See `DECISION_LOG.md#cap_reference_cycle_policy`. Mutual caps between services:

- **Permitted** with unordered teardown + timeout, **or**
- **Forbidden** at kernel-object level

---

## Object destruction (R-destroy-notify)

Lifecycle transition to **Invalidated** triggers **R-destroy-notify**: all cap holders receive terminal error at authority checkpoint.

Distinct from **R-cascade-revoke** (delegation-chain only). Third-party independent caps to the same object are unaffected by single-cap revoke.

Delivery ordering: `DECISION_LOG.md#r_destroy_notify_ordering`.

---

## Bootstrap cap ceremony

PID-1 equivalent receives an explicit cap set — the **only** caps created without prior cap authorization. Threat node: `T-bootstrap-scope-creep`.

---

## Orphan endpoints

Endpoint owner process death:

- Pending queue **dropped**
- Embedded caps → R-destroy-notify
- Senders receive terminal at checkpoint

---

## MemoryRegion rights

Read, write, execute, resize (or documented subset). All cross-process shared memory is **cap-mediated**.

---

## Process hierarchy

Parent/child relationships documented. Parent tier-2 fault propagation; child cap fate on parent death; reaper policy stub (no reparent → children terminated unless charter exception).

**Process audit token:** stable **root cap_id** of process — no ambient POSIX UID model.

---

## Cap kind schema version

On wire; unrecognized kind version → **structural** error (not terminal).

---

## Cap send / confinement

Non-sendable (confined) right bit **or** confinement out-of-scope pre-150 with threat node — affects IPC wire format. See gap registry #293.

---

## Kind semantics freeze

Once a cap kind graduates the never-stabilize list (`DESIGN_NORTH_STAR.md`), semantics are **frozen**. Reinterpretation requires a **new kind**, not field reuse.

---

## Registry ground truth

Machine-readable kinds and operations: [`CAP_REGISTRY.toml`](CAP_REGISTRY.toml). CI verifies registry ↔ markdown parity.
