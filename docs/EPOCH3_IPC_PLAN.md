# Epoch 3 Planning — IPC + Build Integrity (131–140)

```yaml
status: epoch-scoped: 3
```

Pre-epoch planning commit deliverables (before implementation):

- Expand phase 131–140 checklists to epoch-1 detail level
- Signed system images + reproducible build CI (BUILD_INTEGRITY)
- Phase **134**: remove interim IPC bridge; CI `ipc_bridge_compat_internal` counter → 0
- P-134 semantic ordering smoke corpus populated at phase 133
- Audit IPC correlation on wire (ERROR_TAXONOMY + WIRE_SCHEMA_REGISTRY)

Gate: 1M message soak; benchmark vs prior epoch; compat review.
