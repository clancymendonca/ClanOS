# Compat Sunset Policy

```yaml
status: authoritative
semantics_version: 1.0.0
epoch: 0
authored_by: migration
```

```yaml
status: authoritative
semantics_version: 1.0.0
```

---

## Metric

Fixed test corpus denominator: compat scenarios that must run **native-only end-to-end** by milestone 150.

`native_only_pct = native_only_passing / corpus_size`

Reviewed at **every epoch gate** in commit body + validation matrix.

---

## Distinction from interim IPC counter

| Mechanism | Tracks |
|-----------|--------|
| `ipc-bridge-compat-internal` | IPC bridge call sites (phases 122–133) → **zero by phase 134** |
| COMPAT_SUNSET metric | Compat socket ABI, ELF path, FD substrate (epoch 4+) |

---

## Graduation

Item graduates when: full spec, zero callers in matrix, compat review entry zero.