# Interim IPC Bridge (compat-internal)

```yaml
status: authoritative
semantics_version: 1.0.0
```

Phases **122–133** only. Removed at phase **134** (CI counter must be zero).

Tagged `compat-internal` — not PipeLite (A5), not native endpoint truth.

---

## Semantics

| Property | Policy |
|----------|--------|
| Ordering | FIFO per `(sender_pid, session_id)` |
| Cross-session | No ordering guarantee |
| Max message | 256 bytes (`MAX_MSG_BYTES`) |
| Backpressure | Bounded queue (4 msgs); saturated → **E-00** transient |
| Revoke mid-message | Terminal at checkpoint (single documented outcome) |

---

## Wire

Schema: `ipc.interim.v1` in WIRE_SCHEMA_REGISTRY.md.

---

## CI

`ipc_bridge_compat_internal_count()` — grep + runtime counter; **must be 0** at phase 134 gate.

Compat sockets (epoch 4) use COMPAT_SUNSET only — not this counter.
