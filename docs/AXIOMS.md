# Constitutional Axioms (Clan OS Post-100)

Clan OS scopes 1–100 built kernel mechanics. Post-100 work defines **constitutional law** for semantic integrity: authority, IPC, temporal visibility, and native identity.

Axioms are the highest normative layer. They change only by explicit **charter revision** (rare). All documents in [INDEX.md](INDEX.md) § Post-100 must remain consistent with these axioms.

See also: [NATIVE_MODEL.md](NATIVE_MODEL.md), [SEMANTIC_JURISDICTION.md](SEMANTIC_JURISDICTION.md), [RIGHTS_ALGEBRA.md](RIGHTS_ALGEBRA.md).

---

## Anti-entropy pair (most strategically important)

| Axiom | Statement | Role |
|-------|-----------|------|
| **A7** | **Semantic laws override implementation convenience** | Prevents convenience-driven corruption of the model over years |
| **A10** | **Semantic minimization** — no new law without justification (see below) | Prevents doctrinal sprawl and unmaintainable semantic surface |

Together: **controlled semantic evolution**. Without A7, implementation erodes architecture. Without A10, architecture becomes too heavy to reason about.

---

## Full axiom table

| ID | Axiom | Role |
|----|--------|------|
| **A1** | Authority **never amplifies implicitly** | Prevents hidden privilege inflation |
| **A2** | Native code has **no ambient authority** (no unrestricted global namespace) | Foundational post-Unix principle |
| **A3** | All **cross-domain authority transfer** is explicit (move / borrow / delegate per [RIGHTS_ALGEBRA.md](RIGHTS_ALGEBRA.md)) | Formalizes propagation |
| **A4** | **Endpoint semantics** are deterministic within documented bounds | Prevents IPC drift |
| **A5** | **Compat never defines native semantics** | Preserves architectural identity |
| **A6** | **Revocation visibility** is always documentable ([TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md)) | Prevents temporal ambiguity |
| **A7** | Semantic laws override implementation convenience | Anti-entropy (see above) |
| **A8** | [Architecture-preservation spec cases](SEMANTIC_SPECS.md) are authoritative for behavior claims | Invariant preservation |
| **A9** | **Semantic law versioning** is explicit (`clan-semantics-v*` in [ABI_STABILITY.md](ABI_STABILITY.md)) | Semantics are platform ABI |
| **A10** | Semantic minimization — every new law must pass review (below) | Occam’s Razor for OS semantics |

---

## A10 — minimization review (required for new laws after scope 110)

| Question | Must answer |
|----------|-------------|
| Can this derive from existing axioms or laws? | Yes → do not add a new law |
| Is it **fundamental** (cross-cutting), not subsystem-local? | No → keep in subsystem notes only |
| Is it the **smallest** statement that carries the guarantee? | No → rewrite smaller |
| Should this be a [spec case ID](SEMANTIC_SPECS.md) instead of prose? | Prefer spec ID |

Scope 110 performs a **minimization audit** (law count per hierarchy layer).

---

## Governance gates (implementation blocked until scope 110 sign-off)

| Gate | Blocks | Requires |
|------|--------|----------|
| G1 | Scope 115+ new handle semantics | [KERNEL_OBJECT_MODEL.md](KERNEL_OBJECT_MODEL.md) |
| G2 | Scope 112–113 cap lifecycle code | [RIGHTS_ALGEBRA.md](RIGHTS_ALGEBRA.md) |
| G3 | Scope 134+ native endpoint code | [ABI_IPC.md](ABI_IPC.md) |
| G4 | Scope 128+ native-only enforcement | [NATIVE_MODEL.md](NATIVE_MODEL.md) |
| G5 | Scope 111+ authority/IPC contradicting specs | [SEMANTIC_SPECS.md](SEMANTIC_SPECS.md), [TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md) |

Scopes **111+** must not ship kernel behavior that violates ratified axioms or signed gates.

---

## Charter revision

To amend an axiom:

1. Document conflict with existing laws and spec cases
2. Propose `clan-semantics-v*` bump if guarantees change
3. Update [SEMANTIC_SPECS.md](SEMANTIC_SPECS.md) and run minimization audit
4. Record decision in [ROADMAP_POST100.md](ROADMAP_POST100.md) scope 110+ notes

Default: **reject** charter changes that weaken A1, A2, A5, A7, or A10 without extraordinary rationale.
