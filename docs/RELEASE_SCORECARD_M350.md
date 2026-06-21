# Release Scorecard — Scope 350 Era (Historical)

> **Historical artifact.** Superseded by [RELEASE_SCORECARD.md](RELEASE_SCORECARD.md). Milestone numbering retired; validation uses unified `validation_gate.rs` (ADR-0001).

```yaml
status: superseded-by
superseded-by: docs/RELEASE_SCORECARD.md
scope_index: 350
semantics_version: 1.0.0
```

Release 1.0 era criteria (now subsystem gates):

| Criterion | Subsystem gate |
|-----------|----------------|
| Gap registry closed | `release_scorecard_check.py` |
| Threat nodes closed | `threat_node_lifecycle_check.py` |
| Covenant CI | `covenant_ci.py` |
| Integrity → release smokes | `integrity`, `scheduling`, `hardware`, `federation`, `release` |
| Never-stabilize graduated | `never_stabilize_graduated.toml` |
| Public process docs | `SECURITY.md`, `CONTRIBUTING.md` |

Full matrix: `python scripts/gate/run.py --gate all` · Host: `python scripts/gate/host.py`
