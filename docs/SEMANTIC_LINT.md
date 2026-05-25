# Semantic Lint (Architecture Preservation)

Outline at phase **109**; automation scheduled post-110; **CI gate** at phases **159–160** before `ares-semantics-v*` bumps ([ABI_STABILITY.md](ABI_STABILITY.md)).

Not full theorem proving — **semantic static analysis** assisting human review.

See: [SEMANTIC_SPECS.md](SEMANTIC_SPECS.md), [AXIOMS.md](AXIOMS.md) A10.

---

## Lint classes

| Class | Detects |
|-------|---------|
| **Spec reference integrity** | Orphan case IDs; laws without ≥1 linked case |
| **Law linkage** | Broken cross-doc links; undefined case references |
| **Hierarchy violations** | Lower layer contradicting [NATIVE_MODEL.md](NATIVE_MODEL.md) hierarchy or [AXIOMS.md](AXIOMS.md) |
| **Temporal contradictions** | Conflicting visibility statements (heuristic) |
| **Minimization warnings** | Duplicate law candidates; subsystem-local doctrine posing as global law |
| **Jurisdiction violations** | Compat doc defining native behavior; runtime defining IPC delivery |

---

## Phase 110 (manual)

Until `scripts/semantic_lint.py` exists:

- [ ] Every R-/E-/T- case referenced from a law doc
- [ ] No native law defined only in compat checklists
- [ ] Minimization audit: law count per hierarchy layer recorded in phase-110 checklist

---

## Future tooling

| Tool | Purpose |
|------|---------|
| `scripts/semantic_lint.py` | Parse docs + optional kernel annotations |
| CI job `semantic-lint` | Required before `ares-semantics-v*` version bump |
| Doc check in PR template | A10 questions for new law prose |

Optional: Rust proc-macro or comment convention `// semantic_spec: R-01` for linkage to tests.

---

## Non-goals (initial)

- Automated Coq/Lean proof
- Full happens-before model checker
- POSIX compat lint (separate compat matrix)
