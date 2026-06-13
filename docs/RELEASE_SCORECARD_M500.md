# Milestone 500 Release Scorecard — Fully Operational OS

```yaml
status: authoritative
milestone: 500
semantics_version: 1.0.0
```

| Criterion | Target | Gate |
|-----------|--------|------|
| System gate | `SYSTEM_GATE_VERSION = 1.0.0` | `system_gate.rs` |
| Functional OS | Desktop + userland + network | `scripts/gate/system.py --gate functional` |
| CI matrix | Unified gate wired | `validation_matrix.py` |
| ares-rt no_std | `#![no_std]` on lib | `scripts/gate/ares_rt.py` |
| Production SMP | AP scheduler smoke | `scripts/gate/system.py --gate production` |
| Signed ELF corpus | Digest-verified user manifests | `production_gate` kernel smoke |
| External network | `has_external_network = true` | `architecture_state.toml` |
| Compat bridge | `ipc_bridge_compat_internal = 0` | kernel boot |
| Boot gate | `BOOT_GATE_VERSION = 1.0.0` | `boot_gate.rs` |
| Boot smoke | `AresOS-BootGate: ok=true` | QEMU serial |
| System smoke | `AresOS-SystemGate: ok=true` | QEMU serial |
| Host checks | boot + system | `scripts/gate/host.py` |
