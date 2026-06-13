# Security ABI — No Ambient Authority

Scope **105** — native vs compat syscall surface policy.

See: [SECURITY.md](SECURITY.md) (scopes 10–58 compat), [AXIOMS.md](AXIOMS.md) A2, A5, [NATIVE_DEVELOPER_EXPERIENCE.md](NATIVE_DEVELOPER_EXPERIENCE.md).

---

## Native policy (post-111 target)

| Rule | Detail |
|------|--------|
| No global FS namespace | Native processes cannot enumerate `/` |
| No path-string open | Native syscall surface omits compat `OpenFile` with user paths |
| Storage | **FsNode** caps minted by storage broker after grant check |
| Manifest | `clan-native-v1` required for native ELFs (scope 128+) |

---

## Compat policy (frozen scopes 1–100)

| Mechanism | Source |
|-----------|--------|
| Users / roles | [SECURITY.md](SECURITY.md) |
| File owner/mode | SimpleFs metadata |
| Trust / digest | Scopes 43, 58 |
| Allowlisted ELF names | Scope 29 |
| Path syscalls | `OpenFile`, `Chdir`, probes — hardware allowlist |

Compat **never defines** native semantics (A5).

---

## Path broker (scope 115 — compat only)

Translates compat `open("/path")` to broker-internal resolution. Native code must not call path broker directly.

---

## Permission broker (scope 123+)

Enforces manifest scopes:

```toml
# clan-native-v1 example (illustrative)
[permissions]
filesystem = ["Documents/Projects"]
network = ["api.example.com"]
camera = false
```

Scopes map to cap grants — not ambient directories.

---

## Broker trust chain

Platform brokers hold attenuated Device / FsNode mint authority from kernel bootstrap caps — documented per [SEMANTIC_JURISDICTION.md](SEMANTIC_JURISDICTION.md).
