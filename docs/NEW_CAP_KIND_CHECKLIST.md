# New Cap Kind Checklist (post-150)

```yaml
status: authoritative
epoch: 7
```

Placeholder for gap #100 — required before adding `ObjectKind` variants post-150:

1. Update [`CAP_REGISTRY.toml`](CAP_REGISTRY.toml) and run `cap_registry_sync.py`
2. Threat node in [`THREAT_NODES.toml`](THREAT_NODES.toml)
3. Proof harness tier A/B in [`kani_harness_registry.toml`](../kani_harness_registry.toml)
4. Native error code in ERROR_TAXONOMY
5. Phase owner sign-off in epoch gate manifest
