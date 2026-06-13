# Architecture-Preservation Semantic Spec Cases

Normative scenarios that **must** hold. Not implementation unit tests in this documentation pass — same IDs become executable semantic tests when subsystems exist (QEMU / Rust `#[semantic_spec(...)]`).

**Gate G5** — scope 111+ behavior must not contradict these cases without `clan-semantics-v*` bump.

See: [AXIOMS.md](AXIOMS.md) A8, [SEMANTIC_LINT.md](SEMANTIC_LINT.md).

---

## Shared semantic namespace

| Prefix | Domain |
|--------|--------|
| `R-*` | Rights / authority |
| `E-*` | Endpoints / IPC |
| `T-*` | Temporal visibility |
| `M-*` | Meta-semantics / precedence |
| `S-*` | Scheduler (outline post-150) |

Use the same ID in docs, code comments, tests, and diagnostics ([SEMANTIC_OBSERVABILITY.md](SEMANTIC_OBSERVABILITY.md)).

---

## Rights cases (R-*)

| ID | Scenario | Expected outcome |
|----|----------|------------------|
| **R-01** | Delegate cap to child | Child rights ⊆ parent; no new rights |
| **R-02** | Borrowed cap | Cannot outlive lender; cannot delegate; fails after lender revoke |
| **R-03** | Generation revoke on object | All caps at old generation invalid at checkpoint |
| **R-04** | Proxy revoke (broker session end) | All caps minted via session invalid |
| **R-05** | Move transfer | Sender cap slot empty after success; receiver holds authority |
| **R-06** | Amplification attempt without broker authority | Operation fails; audit counter if applicable |

**Laws:** [RIGHTS_ALGEBRA.md](RIGHTS_ALGEBRA.md)

---

## Endpoint cases (E-*)

| ID | Scenario | Expected outcome |
|----|----------|------------------|
| **E-01** | Bounded queue full | Producer blocks or errors per [ABI_IPC.md](ABI_IPC.md); no silent drop |
| **E-02** | Cancel while messages in flight | Ordering vs cancel documented; no duplicate consume |
| **E-03** | Peer service death | Generation / teardown per [TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md); waiters wake with error |
| **E-04** | Wait-set timeout | Deterministic wake ordering among ready endpoints |
| **E-05** | Sustained backpressure | No unbounded kernel queue growth |

**Laws:** [ABI_IPC.md](ABI_IPC.md) — frozen before scope 134 implementation.

---

## Temporal cases (T-*)

| ID | Scenario | Expected outcome |
|----|----------|------------------|
| **T-01** | Lazy revoke | Cap fails at documented checkpoint, not arbitrarily late |
| **T-02** | Cancel + revoke race | Single documented outcome; no authority double-free |
| **T-03** | SMP delegate | No CPU observes amplified rights vs delegate contract |
| **T-04** | Cancel + revoke + teardown + restart overlap | Single outcome per meta-semantics outline (M-01) |

**Laws:** [TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md)

---

## Meta-semantics cases (M-* — outline)

| ID | Scenario | Expected outcome |
|----|----------|------------------|
| **M-01** | Same as T-04 | Precedence sketch in TEMPORAL_SEMANTICS; full table scopes 156–158 |

---

## Scheduler cases (S-* — outline post-150)

| ID | Scenario | Expected outcome |
|----|----------|------------------|
| **S-01** | Mailbox wake vs compat idle | Native endpoint waiter runs before best-effort compat idle work (service-centric) |
| **S-02** | Service starvation | Documented fairness bound **or** explicit non-guarantee in SCHEDULING_UNIFIED |

---

## Law ↔ case linkage matrix (scope 110 audit)

Every major law in RIGHTS_ALGEBRA and ABI_IPC must link ≥1 case ID above.

| Law area | Cases |
|----------|-------|
| Monotonicity / delegate | R-01, R-06 |
| Borrow / move | R-02, R-05 |
| Generation / proxy revoke | R-03, R-04 |
| Endpoint bounds / backpressure | E-01, E-05 |
| Cancel / peer death | E-02, E-03, T-02 |
| Visibility | T-01, T-03, T-04 |

---

## Executable tests (future)

| Scope | Work |
|------:|------|
| 112+ | Rust / QEMU tests tagged with R-* |
| 134+ | E-* endpoint tests |
| 159–160 | CI requires semantic lint + spec coverage before `clan-semantics-v*` bump |
