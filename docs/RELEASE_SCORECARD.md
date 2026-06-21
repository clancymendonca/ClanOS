# Clan OS Release Scorecard — Fully Operational OS

```yaml
status: authoritative
validation_gate_version: "2.2.0"
semantics_version: 1.0.0
```

| Criterion | Target | Gate |
|-----------|--------|------|
| Validation gate | `VALIDATION_GATE_VERSION = 2.2.0` | `validation_gate.rs` |
| Functional OS | Desktop + userland + network | `scripts/gate/run.py --gate functional` |
| CI matrix | Unified gate wired | `validation_matrix.py` |
| clan-rt no_std | `#![no_std]` on lib | `scripts/gate/clan_rt.py` |
| Production SMP | AP scheduler smoke | `scripts/gate/run.py --gate production` |
| Signed ELF corpus | Digest-verified user manifests | `production_gate` kernel smoke |
| External network | `has_external_network = false` until scope 475 gate | `architecture_state.toml` + [`GATE_AUDIT_401_500.md`](GATE_AUDIT_401_500.md) |
| Compat bridge | `ipc_bridge_compat_internal = 0` | kernel boot |
| Gap registry | 350 addressed (substance audit) | [`GAP_AUDIT.md`](GAP_AUDIT.md) |
| Full gate smoke | `ClanOS-Gate: ok=true` | QEMU serial |
| Host checks | gate host + compat | `scripts/gate/host.py` |

See [`VALIDATION_GATES.md`](VALIDATION_GATES.md) for subsystem inventory. **Fully operational OS** (scopes 401–500) is the roadmap target in [ROADMAP_401_500.md](ROADMAP_401_500.md); current STATUS headline is **Functional OS (scope 400)** — see [GATE_AUDIT.md](GATE_AUDIT.md).
