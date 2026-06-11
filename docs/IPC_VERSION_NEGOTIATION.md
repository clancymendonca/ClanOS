# IPC Version Negotiation

```yaml
status: authoritative
semantics_version: 1.0.0
```

Required before phase **134** endpoint cutover.

---

## Policy

- Discovery handshake returns supported `ipc.interim.v1` / future `endpoint.v1` ranges
- Max spread documented; downgrade edges proptest-covered before 134
- **P-134 property:** interim FIFO-per-session behaviors ⊆ native endpoint ordering smoke corpus (populated phase 133)

---

## Epoch 1 stub

Brokers use `ipc.interim.v1` only. Negotiation API deferred to epoch 3 planning commit.
