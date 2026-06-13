# Semantic Observability (Outline)

**Documentation outline only** in scopes 101–110. Implementation targeted scopes **181–190** ([ROADMAP_POST100.md](ROADMAP_POST100.md)).

Without observability, debugging authority at scale becomes impractical.

See: [AXIOMS.md](AXIOMS.md), [SEMANTIC_SPECS.md](SEMANTIC_SPECS.md) shared IDs, [TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md).

---

## Questions tooling must answer

| Question | Artifact |
|----------|----------|
| Why was this authority transfer valid? | Law name + spec case ID (e.g. R-05) |
| Which law caused this revoke propagation? | RIGHTS_ALGEBRA rule + generation chain |
| What is this cap’s lineage? | Mint path: broker session → object → delegate chain |
| What did this thread observe when? | Temporal visibility timeline |

---

## Planned capabilities (post-170)

| Capability | Description |
|------------|-------------|
| **Semantic tracing** | Kernel/broker events tagged with law + case ID |
| **Law-linked diagnostics** | User-visible errors cite `clan-semantics-v*` clause |
| **Capability lineage graph** | ObjectId, generation, parent cap |
| **Temporal reconstruction** | Checkpoint-ordered revoke/cancel/restart log per process |

---

## Integration points

- Boot smoke may expose `semantic_trace=off|summary` (future)
- Permission broker logs manifest grant with case ID
- Native SDK ([NATIVE_DEVELOPER_EXPERIENCE.md](NATIVE_DEVELOPER_EXPERIENCE.md)) surfaces grant labels, not raw ObjectIds, in debug builds

---

## Non-goals (initial ship)

- Full distributed trace across machines (federation deferred)
- GUI lineage visualizer (SDK scope 161–170 may prototype)

---

## Dependencies

Stable before heavy observability investment:

- G1–G5 signed (scope 110)
- Immutable identity + generation ([KERNEL_OBJECT_MODEL.md](KERNEL_OBJECT_MODEL.md))
- Executable semantic tests for R-* / E-* (scopes 112+, 134+)
