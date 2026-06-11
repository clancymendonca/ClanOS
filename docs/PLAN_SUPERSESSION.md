# Planning Document Supersession

```yaml
status: authoritative
```

## Policy (Upgrade R)

After `gap_registry.toml` import is reviewed and **scope-freeze** is recorded, the multi-round Cursor planning document is **no longer authoritative**.

| Superseded | Living reference |
|------------|------------------|
| `aresos_full_os_build_b960e4a0.plan.md` | `gap_registry.toml` + `CHARTER.md` + `DESIGN_NORTH_STAR.md` + individual spec docs |

Do not maintain parallel sources of truth. New gaps are tracked only in `gap_registry.toml`.

## Archive action

Add to planning document frontmatter when archiving (human step at scope-freeze commit):

```yaml
status: superseded-by: gap_registry.toml
```

The planning file may remain in `.cursor/plans/` for history but must not receive new binding requirements.
