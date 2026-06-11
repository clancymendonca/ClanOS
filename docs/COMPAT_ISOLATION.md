# Compat Shim Isolation

```yaml
status: authoritative
semantics_version: 1.0.0
```

Threat node: `T-compat-shim-escape`.

---

## Policy

- Compat syscalls operate on **per-caller FD sessions** — no ambient shim capability
- Path broker (phase 115) is **compat-only** — no parallel native handle type (G1)
- `compat-internal` IPC bridge is **not** PipeLite (A5) and **not** native truth
- Native processes cannot enumerate global namespace (phase 117)

---

## Broker boundary

Platform brokers mint caps only via documented grant paths (`storage_broker`, `permission_broker`). Compat ELF loading unchanged (phase 119 bridge).
