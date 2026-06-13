# Epoch 0 Scope Freeze Marker

```yaml
status: authoritative
```

**Scope-freeze commit:** `5961eb7` — epoch 0 artifacts authored per execution plan.

## Clock

90-day epoch 0 budget starts at the git commit that adds this file (per `CHARTER.md`).

## Living authority

After human review of `gap_registry.toml` import:

- **`gap_registry.toml`** is the canonical gap lifecycle source
- The Cursor planning document `clanos_full_os_build_b960e4a0.plan.md` is **superseded** — see [`docs/PLAN_SUPERSESSION.md`](docs/PLAN_SUPERSESSION.md)

## Gate remaining

Before epoch 0 squash:

1. Resolve all `[CROSS-REF — TBD]` stubs
2. Populate `epoch_signoffs/epoch-0.toml` (unanimous 3/3)
3. GPG-signed gate commit
4. `epoch_checklist.toml` all items green
