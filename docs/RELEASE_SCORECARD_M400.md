# Milestone 400 Release Scorecard — Functional OS

```yaml
status: authoritative
milestone: 400
semantics_version: 1.0.0
```

| Criterion | Target | Gate |
|-----------|--------|------|
| Phase catalog | `COMPLETED_PHASE = 400` | `phase_catalog.rs` |
| Desktop M375 | GUI + mouse + WM + shell | `phase375_milestone_check.py` |
| Native apps | `/bin/demo-hello` runs | `phase399_native_app_smoke` |
| Network | Loopback ping | `phase386_network_smoke` |
| Prior milestones | M350 regression-free | `phase350_milestone_check.py` |
| Boot smoke | `Phase400-Milestone: ok=true` | QEMU serial |
