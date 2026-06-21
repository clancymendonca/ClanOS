# Release Scorecard — Fully Operational OS Era (Historical)

> **Historical artifact.** Superseded by [RELEASE_SCORECARD.md](RELEASE_SCORECARD.md). Milestone numbering retired; validation uses unified `validation_gate.rs` (ADR-0001).

```yaml
status: superseded-by
superseded-by: docs/RELEASE_SCORECARD.md
scope_index: 500
semantics_version: 1.0.0
```

Fully operational OS era criteria (now unified validation gate):

| Criterion | Subsystem gate |
|-----------|----------------|
| Functional OS regression | `functional` |
| Production SMP | `production` |
| Signed ELF corpus | `production_gate` kernel smoke |
| External network | `network` |
| CI hardening | `ci` |
| Full boot matrix | `all` → `ClanOS-Gate: ok=true` |

Host: `python scripts/gate/host.py` · QEMU: `python scripts/gate/run.py --gate all --timeout 360`

See [RELEASE_SCORECARD.md](RELEASE_SCORECARD.md) for current criteria.
