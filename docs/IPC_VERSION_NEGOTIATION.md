```yaml
status: superseded-by: docs/specs/IPC_VERSION_NEGOTIATION.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/specs/IPC_VERSION_NEGOTIATION.md`](specs/IPC_VERSION_NEGOTIATION.md). This flat copy retained until migration squash reconciles any differences.

# IPC Version Negotiation

```yaml
status: authoritative
semantics_version: 1.0.0
```

Required before scope **134** endpoint cutover.

---

## Policy

- Discovery handshake returns supported `ipc.interim.v1` / future `endpoint.v1` ranges
- Max spread documented; downgrade edges proptest-covered before 134
- **P-134 property:** interim FIFO-per-session behaviors ⊆ native endpoint ordering smoke corpus (populated scope 133)

---

## Epoch 1 stub

Brokers use `ipc.interim.v1` only. Negotiation API deferred to epoch 3 planning commit.
