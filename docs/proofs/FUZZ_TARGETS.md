# Fuzz Targets

```yaml
status: authoritative
semantics_version: 1.0.0
epoch: 0
authored_by: migration
```

```yaml
status: authoritative
semantics_version: 1.0.0
```

Stub ≠ gate. Each target lists boundary conditions before pyramid credit.

---

## Targets (epoch 0 stubs)

| Id | Boundary conditions required |
|----|------------------------------|
| fuzz-ipc-negotiation | Random version pairs across spread; downgrade edges |
| fuzz-cap-wire | Invalid kind version → structural error |
| fuzz-rights-mask | Empty-rights cap policies |

Committed corpus hash at graduation; verified each CI run.