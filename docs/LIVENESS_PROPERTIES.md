# Liveness Properties

```yaml
status: authoritative
semantics_version: 1.0.0
```

Safety vs liveness split. Tier D formal models post-150.

---

## Documented liveness obligations

- IPC cancel must not block on saturated queue
- Cap quota release-retry terminates
- Suspend flush timeout bounded

---

## Out of scope pre-150

Full temporal logic proofs — pointer to `FORMAL_MODEL.md` when framework decision recorded.
