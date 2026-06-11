# Milestone 350 Release Scorecard

```yaml
status: authoritative
milestone: 350
semantics_version: 1.0.0
```

| Criterion | Target | Gate |
|-----------|--------|------|
| Phase catalog | `COMPLETED_PHASE = 350` | `phase_catalog.rs` |
| Gap registry | 0 open | `release_scorecard_check.py` |
| Threat nodes | 0 open | `threat_node_lifecycle_check.py` |
| Covenant CI | green | `covenant_ci.py` |
| Milestone smokes | 175/200/250/300/350 | QEMU or `phase_smoke_host_check.py` |
| Never-stabilize | graduated | `never_stabilize_graduated.toml` |
| Public process | SECURITY + CONTRIBUTING | repo root docs |
| Reviewer registry | populated | `keys/reviewer-registry.toml` |
