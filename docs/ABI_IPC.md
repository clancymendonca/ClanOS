# IPC ABI — Endpoints (`clan-semantics-v1` draft)

**Gate G3** — native endpoint implementation (scope 134+) blocked until guarantees here are signed at scope 110.

PipeLite (scope 87) is **compat** only (`/@pipe/` FD paths). Native uses **Endpoint** kernel objects.

See: [AXIOMS.md](AXIOMS.md), [SEMANTIC_SPECS.md](SEMANTIC_SPECS.md) E-*, [TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md), [ABI_ASYNC.md](ABI_ASYNC.md).

---

## Guarantees (frozen at scope 103/110)

### Ordering

| Mode | Semantics |
|------|-----------|
| **Per-sender FIFO** (default) | Messages from sender A to endpoint E are delivered in send order |
| Cross-sender | No global order unless endpoint documents total order |
| Sequence numbers | Optional for at-least-once idempotency |

### Delivery

| Class | Native default (draft) |
|-------|------------------------|
| At-most-once | Fire-and-forget without ack |
| **At-least-once** | Default with idempotency key on handler |
| Exactly-once | Not default; requires app-level dedup |

### Backpressure

| Rule | Detail |
|------|--------|
| Bounded queues | Max messages and/or max bytes per endpoint (configured at create) |
| Full queue | Producer **blocks** or returns `QUEUE_FULL` — **no silent drop** (E-01) |
| Kernel memory | E-05 — no unbounded growth |

### Ownership transfer

| Mode | Semantics |
|------|-----------|
| **Copy** | Bytes copied into receiver buffer; sender retains cap |
| **Move** | `MemoryRegion` cap or inline buffer cap transferred; sender slot cleared (R-05) |

Visibility per [TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md).

### Cancellation

- Cancel token on endpoint or wait set
- In-flight messages: fate defined in E-02 (discard vs deliver-to-dead-letter)
- Propagation to blocked waiters per [ABI_ASYNC.md](ABI_ASYNC.md)

### Timeouts

Wait-set wait with timeout → E-04 deterministic wake ordering among ready endpoints.

### Crash propagation

Peer service death → generation bump / endpoint teardown (E-03, R-03). Waiters receive documented error, not hang.

---

## Compat PipeLite (unchanged)

| Property | PipeLite |
|----------|----------|
| Syscall | `Pipe = 80` |
| Path | `/@pipe/{id}/r`, `/@pipe/{id}/w` |
| Poll | `Poll = 82` single-fd readiness |
| Capacity | 64 bytes ring, max 4 pipes (scope 87) |

Spec **139** (future checklist): compat pipe preserved when native endpoints ship.

---

## Semantic spec cases

E-01 through E-05 in [SEMANTIC_SPECS.md](SEMANTIC_SPECS.md).
