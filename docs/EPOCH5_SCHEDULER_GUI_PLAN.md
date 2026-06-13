# Epoch 5 Planning — Scheduler, GUI, SMP (141–150)

```yaml
status: epoch-scoped: 5
```

Pre-epoch planning commit:

- SMP loom test plan per shared structure
- ABI_COMPOSITOR_IPC before scope 145
- Scope 147 full OOM — suspend frozen-in-memory; MEM_BUDGET_STUB enforcement
- AP bring-up only after loom + SMP review

Gate: compositor crash smoke; full matrix; benchmarks.
