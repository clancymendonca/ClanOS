# AresOS Project Charter

```yaml
status: authoritative
semantics_version: 1.0.0
```

This document defines **charter-level** authority for AresOS governance. Process rules in epoch checklists, phase checklists, and `EPOCH_FAILURE_PROCEDURE.md` are subordinate to this charter unless explicitly elevated here.

Referenced by: scope-freeze exceptions, Verus N+2 escalation, emergency dependency bumps, C-ABI FFI gates, compound epoch failures, dissent override, emergency stabilization.

---

## Authority

**Charter decisions** may be invoked by:

1. **Unanimous domain quorum** (see below), or
2. **Charter session** after dissent escalation timeout (14 days, see Dissent), or
3. **Documented emergency** where process-level gates cannot complete in time and a named charter fallback exists in another spec (e.g. day-zero dependency soak exemption).

Only outcomes recorded in `DECISION_LOG.md` or an amended charter commit constitute binding charter decisions.

---

## Quorum

Epoch 0 requires **unanimous sign-off from three domains**:

| Domain | Scope |
|--------|-------|
| **Kernel / object model** | Cap table, lifecycle, scheduler, memory model |
| **Evidence / verification** | Kani, proptest, Verus, fuzz, proof coverage |
| **Process / compat** | Epoch gates, compat sunset, ABI, supply chain |

Each domain must have a **primary** and **backup** reviewer identified before the epoch 0 gate (`SECURITY.md`).

Sign-offs are recorded in `epoch_signoffs/epoch-N.toml` per `epoch_signoffs/schema.toml`. Dissent blocks squash until resolved.

---

## Charter vs process

| Level | Examples | Change mechanism |
|-------|----------|------------------|
| **Charter** | This document, scope-freeze exceptions, emergency stabilization, Verus N+2 acceptance, C-ABI FFI approval | Quorum + GPG-signed gate commit |
| **Process** | Phase checklist fields, benchmark thresholds, reviewer currency cadence, compat review checklist | Epoch gate or additive doc semver |
| **Implementation** | Phase commits, syscall behavior, driver code | Phase owner commit + pyramid gates |

When a process rule says "charter approval," it means quorum per this document.

---

## Amendments

Changes to `CHARTER.md` require:

1. Staging commit with explicit `charter-amendment` tag in message body
2. Unanimous 3/3 domain re-sign-off
3. GPG-signed squash commit
4. Entry in `DECISION_LOG.md` describing rationale and alternatives rejected

Additive clarifications that do not change authority or quorum may use protocol semver `clarification` with a single non-author reviewer acknowledgment.

---

## Dissent resolution

1. Dissent recorded in `epoch_signoffs/` blocks squash until addressed in a follow-up staging commit.
2. If unresolved after **14 days**, escalate to charter review session.
3. Charter may override with recorded justification in `DECISION_LOG.md`.
4. **Parallel dissent:** multiple concurrent dissents are escalated together in one charter session.

---

## Minimum viable team

If the project cannot field three distinct people for domain quorum:

- One person **may hold multiple domains** with documented justification in `epoch_signoffs/epoch-N.toml` (`multi_domain_roles`).
- Multi-domain assignment is **reviewed at every epoch gate**.
- Reviewer currency (`SECURITY.md`) still applies; the same person satisfies currency for held domains with documented roles.

This is an operational allowance, not a permanent reduction in review rigor.

---

## Epoch 0 time budget

- **90 days** maximum from the **`scope-freeze`** commit to epoch 0 gate squash.
- If exceeded: triage to **minimum viable Epoch 0** — `CHARTER.md`, `gap_registry.toml`, `KERNEL_OBJECT_MODEL.md`, `FAULT_ESCALATION.md`, `THREAT_NODES.toml`, `RIGHTS_ALGEBRA.md`.
- Remaining epoch 0 docs become Epoch 1 prereqs. **This is not project failure.**

---

## Emergency stabilization procedure

Interfaces on the **never-stabilize before 1.0** list (`DESIGN_NORTH_STAR.md`) cannot receive stability guarantees during phases 121–150.

If an **external dependency or integration partner** requires a stability commitment on such an interface before milestone 1.0:

1. **Default:** impossible before 1.0 — no in-tree freeze without charter action.
2. **Charter override path:**
   - Written justification in `DECISION_LOG.md` naming the partner requirement and risk accepted
   - Unanimous 3/3 quorum + charter session record
   - Creation of a **formally tracked divergent artifact** (e.g. `abi-fork/<name>/` with its own semver, changelog, and compatibility matrix) — **not** silent freezing of the in-tree unstable interface
   - The in-tree interface remains on the never-stabilize list until 1.0 graduation per `DESIGN_NORTH_STAR.md`
3. Divergent artifacts are listed in `DESIGN_NORTH_STAR.md` § Emergency forks and reviewed each epoch gate.

Granting stability without this procedure is out of charter authority.

---

## Scope freeze

After the **`scope-freeze`** commit:

- New epoch-0 foundational documents require **charter approval**.
- The 90-day epoch 0 clock starts.
- `gap_registry.toml` is the canonical gap lifecycle source (supersedes the planning document).
