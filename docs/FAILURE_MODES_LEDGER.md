# Failure Modes Ledger

```yaml
status: authoritative
semantics_version: 1.0.0
```

Pathologies, OOM, cap exhaustion — cross-ref `FAULT_ESCALATION.md`, `ERROR_TAXONOMY.md`.

---

## Entries

| Id | Mode | Tier | Doc |
|----|------|------|-----|
| FM-oom-stub | MEM_BUDGET_STUB saturation | 1 | scope 121 |
| FM-cap-quota | Cap quota exceeded | remediable structural | ERROR_TAXONOMY |
| FM-audit-flush | Audit flush timeout on suspend | 3 | FAULT_ESCALATION |
| FM-handler-exhaust | Fault handler MEM_BUDGET exhausted | 3 | FAULT_ESCALATION |
| FM-cap-cycle-timeout | Mutual cap cycle teardown exceeds 5s | 2 | KERNEL_OBJECT_MODEL |
| FM-suspend-flush | Audit flush timeout on suspend | 3 | FAULT_ESCALATION |
