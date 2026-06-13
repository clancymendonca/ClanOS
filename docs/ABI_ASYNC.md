# Async OS Contract (`clan-semantics-v1` draft)

Async is part of the **operating system contract**, not only a userspace library. Documented scope **104**; implementation scopes **131–137**.

See: [AXIOMS.md](AXIOMS.md), [ABI_IPC.md](ABI_IPC.md), [TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md), [ABI_RUNTIME.md](ABI_RUNTIME.md).

---

## Primitives

| Primitive | Role |
|-----------|------|
| **Endpoint** | IPC port (ABI_IPC) |
| **Mailbox** | Service-owned collection of endpoints |
| **Wait set** | Block on multiple endpoints / cancel tokens |
| **Cancel token** | Structured cancellation scope |

Compat `PollLite` on pipe FDs remains; native default is **wait set on endpoints**.

---

## Event-driven wake

Blocked native tasks wake on:

- Endpoint message ready
- Cancel acknowledged
- Timeout expiry

Scheduler integration scope **141–142** — mailbox wake precedes best-effort compat idle (S-01 outline).

---

## Structured cancellation

| Rule | Detail |
|------|--------|
| Scope | Cancel token bound to cap or task lifetime |
| Propagation | Parent cancel → child waits per TEMPORAL + IPC rules |
| Resource | Cancel does not implicitly amplify authority |

---

## Compat boundary

ELF programs may use blocking syscall patterns + `PollLite`. Native programs should use wait sets — runtime adapters map language async/await to OS wait sets ([ABI_RUNTIME.md](ABI_RUNTIME.md)).

---

## Implementation scopes (reference)

| Scope | Feature |
|------:|---------|
| 134 | Endpoint object |
| 135 | Mailbox + cancel |
| 136 | Wait set |
| 137 | Shared MemoryRegion IPC |
| 138 | Zero-copy transfer |
