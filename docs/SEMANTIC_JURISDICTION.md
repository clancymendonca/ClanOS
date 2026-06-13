# Semantic Jurisdiction

Who may define new semantics in Clan OS — **ownership boundaries**, not bureaucracy.

Ratified at scope **110** with [AXIOMS.md](AXIOMS.md) and [NATIVE_MODEL.md](NATIVE_MODEL.md).

---

## Jurisdiction table

| Layer | May define | May NOT |
|-------|------------|---------|
| **Kernel / constitutional** | Object ontology, rights algebra core, temporal visibility base, syscall dispatch law, cap handle semantics | Runtime policy; compat-native exceptions |
| **Platform (brokers / services)** | Broker contracts, service restart policy **within** kernel laws | Native axioms; compat semantics; IPC delivery class |
| **Runtime** | Language adapters, executor conventions **within** [ABI_RUNTIME.md](ABI_RUNTIME.md) | Authority amplification; ambient namespace; revoke visibility |
| **Compat substrate** | ELF, FD, path, POSIX shim behavior | **Native semantics** (axiom A5) |
| **Scheduler (post-150)** | Wake / fairness **within** `SCHEDULING_UNIFIED` (future) | Endpoint ordering; rights monotonicity; temporal visibility |

---

## Rules

1. **New cross-cutting law** → kernel constitutional review + A10 minimization + `clan-semantics-v*` if guarantees change ([ABI_STABILITY.md](ABI_STABILITY.md)).
2. **Runtime** cannot introduce implicit amplification (A1) or ambient paths (A2).
3. **Compat** cannot define native behavior; compat-only features must be tagged `compat-scope` in docs and checklists.
4. **Scheduler** cannot override IPC ([ABI_IPC.md](ABI_IPC.md)) or temporal ([TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md)) guarantees; it only refines wake precedence.
5. **Platform brokers** resolve paths **inside** the broker; native processes receive `FsNode` caps, not ambient `/` (see [ABI_SECURITY.md](ABI_SECURITY.md)).

---

## FAQ

| Question | Answer |
|----------|--------|
| Can runtime introduce semantic laws? | Only executor conventions bounded by `ABI_RUNTIME`; not cross-cutting authority law |
| Can compat define exceptions for native? | No — compat defines compat behavior only (A5) |
| Can scheduler override runtime assumptions? | Only within scheduler jurisdiction (wake/fairness), not IPC delivery |
| Who owns temporal guarantees? | Kernel / constitutional (`TEMPORAL_SEMANTICS.md`) |

---

## Violations (anti-patterns)

- “Runtime invented a law” — e.g. implicit FD authority for native apps
- “Compat silently redefined native” — e.g. path `open` becoming native default
- “Scheduler broke IPC semantics” — e.g. reordering endpoint delivery for convenience

Report violations as semantic lint / charter issues ([SEMANTIC_LINT.md](SEMANTIC_LINT.md)).
