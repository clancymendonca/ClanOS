# Configuration layout

```yaml
status: authoritative
version: 0.1.0
epoch: 14
```

The repository structure spec places machine-readable configuration under `config/`. Canonical files remain at repository root and `docs/` for CI compatibility. This directory documents the mapping. New config files should be created here; migrations happen at epoch gates.

## Path mapping

| Spec path | Current canonical path |
|-----------|---------------------|
| `config/CAP_REGISTRY.toml` | [`docs/CAP_REGISTRY.toml`](../docs/CAP_REGISTRY.toml) |
| `config/THREAT_NODES.toml` | [`docs/THREAT_NODES.toml`](../docs/THREAT_NODES.toml) |
| `config/gap_registry.toml` | [`gap_registry.toml`](../gap_registry.toml) |
| `config/architecture_state.toml` | [`architecture_state.toml`](../architecture_state.toml) |
| `config/trust_anchor_epoch450.toml` | [`config/trust_anchor_epoch450.toml`](trust_anchor_epoch450.toml) |
| `config/signed_elf_test_corpus/` | [`config/signed_elf_test_corpus/`](signed_elf_test_corpus/) (ADR-0002 gate corpus) |
| `config/prereq_graph.toml` | [`prereq_graph.toml`](../prereq_graph.toml) |
| `config/epoch_checklist.toml` | [`epoch_checklist.toml`](../epoch_checklist.toml) |
| `config/GLOSSARY.toml` | [`GLOSSARY.toml`](../GLOSSARY.toml) |
| `config/epoch_signoffs/` | [`epoch_signoffs/`](../epoch_signoffs/) |
| `config/benchmark_baseline.json` | (create at next benchmark gate) |

## Charter

Governance charter: [`CHARTER.md`](../CHARTER.md) (repository root).

## CI enforcement

`scripts/config_readme_check.py` verifies every mapping-table link target exists on disk (wired in `covenant_ci.py`). Update this table when moving canonical files.

Scripts under `scripts/` otherwise reference root and `docs/` paths. Physical relocation requires coordinated script changes and an epoch gate commit.
