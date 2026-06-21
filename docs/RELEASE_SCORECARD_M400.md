# Release Scorecard — Scope 400 Era (Historical)

> **Historical artifact.** Superseded by [RELEASE_SCORECARD.md](RELEASE_SCORECARD.md). Milestone numbering retired; validation uses unified `validation_gate.rs` (ADR-0001).

```yaml
status: superseded-by
superseded-by: docs/RELEASE_SCORECARD.md
scope_index: 400
semantics_version: 1.0.0
```

Functional OS era criteria (now subsystem gates):

| Criterion | Subsystem gate |
|-----------|----------------|
| Desktop stack | `desktop`, `desktop_preview` |
| Native userland | `functional`, `compat_runtime` |
| Network | `network_compat`, `network` |
| Release regression | `release` |

Full matrix: `python scripts/gate/run.py --gate functional` · Summary: `ClanOS-Gate: ok=true`

See [RELEASE_SCORECARD.md](RELEASE_SCORECARD.md) for current criteria.
