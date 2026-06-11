# Track 1 Batch 2 preparation (FE + SM + RA)

Prepared while Batch 1 KOM is in review. Batch 2 blocks on KOM merge; this inventory is ready for anchor updates.

**Canonical status:** `docs/architecture/FAULT_ESCALATION.md` and `docs/architecture/SCHEDULER_MODEL.md` do **not** exist — Batch 2 is **full migration**, not reconciliation (same as KOM was).

---

## FAULT_ESCALATION.md (`FE`) — KOM cross-references to update

| Flat reference | Target after KOM merge |
|----------------|------------------------|
| `KERNEL_OBJECT_MODEL.md` (see line) | `architecture/KERNEL_OBJECT_MODEL.md` |
| Tier 2 generation bump / R-cascade-revoke | KOM § Revocation models, § Universal lifecycle (Teardown, Invalidated) |
| R-cascade-revoke vs R-destroy-notify | KOM § Revocation models |

**Mutual dependency (CROSS-REF stub until squash):**

- FE → SM: **R-revoke-blocked** — `[CROSS-REF: docs/architecture/SCHEDULER_MODEL.md §R-revoke-blocked — resolves at squash]`
- SM → FE: **tier-3 fault escalation path** — `[CROSS-REF: docs/architecture/FAULT_ESCALATION.md §Tier 3 — resolves at squash]`

**GENERATION_COUNTER.md:** FE tier-2 references generation bump — indirect KOM dependency via GENERATION_COUNTER (Batch 5 `GC`, blocks on KOM).

---

## SCHEDULER_MODEL.md (`SM`) — KOM cross-references to update

| Flat reference | Target after KOM merge |
|----------------|------------------------|
| R-cascade-revoke section | KOM § Revocation models (delegation chain) |
| Cap handles (scheduler operates on caps) | KOM § Overview, § Handle semantics |
| R-revoke-blocked | Defines here; FE references this section |

**Mutual dependency:** see FE above.

---

## RIGHTS_ALGEBRA.md (`RA`) — KOM cross-references to update

| Flat reference | Target after KOM merge |
|----------------|------------------------|
| Cap kind definitions (implicit via move/delegate) | KOM § Object kinds, § Handle semantics |
| IPC reply amplification | KOM § IPC and cap transfer; specs `CAP_TRANSFER_PROTOCOL` (Batch 5 `CTP`) |
| No amplification (A1) | KOM § Invariants item 3 |

**Prereq:** `PROOF_COVERAGE` depends on RA — RA must gate before Batch 4 `PC`.

---

## Implementation verification (required before writing canonical docs)

Unlike KOM (definitional), FE and SM document **runtime behavior** that must match kernel code.

**Before migrating FAULT_ESCALATION:** read the actual tier-1/2/3 fault paths in kernel (`fault/`, service restart, audit flush-before-halt). Any flat doc claim not enforced in code → `STUB(track1b)` in implementation + `not-yet-enforced` note in canonical doc (same discipline as KOM TOCTOU diagram).

**Before migrating SCHEDULER_MODEL:** verify `R-revoke-blocked` against scheduler wait/revoke paths. Gaps → STUB + explicit canonical status, not silent omission.

Budget this verification before prose migration — it will take longer than writing.

---

## Batch 2 staging checklist (on KOM merge)

1. Create `docs/architecture/FAULT_ESCALATION.md`, `SCHEDULER_MODEL.md`, `RIGHTS_ALGEBRA.md`
2. Add `superseded-by` stubs to flat copies
3. Insert mutual CROSS-REF stubs; resolve both at Batch 2 squash
4. Record baselines in `config/migration_baselines.json`
