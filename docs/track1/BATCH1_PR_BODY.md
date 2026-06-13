## Changes

**Reconciled from flat `docs/KERNEL_OBJECT_MODEL.md` into canonical `docs/architecture/KERNEL_OBJECT_MODEL.md` (semantics 1.3.0):**

- Scope-110 design decision (immutable identity + generation invalidation)
- Universal interface table (`ObjectId`, `Kind`, `Generation`, `Rights`, `Metadata`)
- G1 handle semantics numbered list (create/transfer/delegate/revoke/close) + scope-115 path broker note
- Generation invalidation section (R-03, triggers)
- Full mint vs delegation authority section (kernel root mint only)
- Cap kind schema version, cap send/confinement, kind semantics freeze
- Historical implementation-scope table (111–115)
- **BrokerSession** kind row (already in `docs/CAP_REGISTRY.toml` as `kernel.broker_session`)
- Orphan endpoint policy under Endpoint per-kind section
- Implementation-verified TOCTOU transfer state machine + property table

**Canonical updated for precision (wording merge, both had content):**

- Reference cycles / 5s teardown timeout — expanded from canonical stub to full DECISION_LOG cross-ref
- Bootstrap cap ceremony — aligned with flat threat-node wording
- R-destroy-notify delivery — simultaneous checkpoint semantics clarified
- Object kinds table — merged flat Examples column with canonical module column

**Infrastructure:**

- `config/migration_baselines.json` — KOM baseline (all four manifest fields + `semantics_version`)
- `docs/PROTOCOL_CHANGELOG.md` — **1.3.0.additive.0** (see classification below)
- `kernel_object::cap_transfer_move` — `STUB(track1b)` on alloc-failure rollback gap
- `docs/track1/BATCH2_PREP.md` — cross-ref inventory for FE/SM/RA

## Diagram verification

| Diagram | Property | Verification |
|---------|----------|--------------|
| Universal lifecycle | Created→Active→Teardown→Invalidated | Spec intent + flat KOM table; matches epoch-0 lifecycle docs |
| Endpoint per-kind | Owner death → Teardown → Invalidated | Spec intent; matches KOM flat orphan-endpoint section |
| IPC transfer sequence | Capability-secured IPC send/reply path | Spec intent; CAP_TRANSFER_PROTOCOL is Batch 2/5 (`CTP`) |
| **TOCTOU transfer** | **Close-before-alloc; never both tables; alloc-failure gap** | **Verified against `kernel_object::cap_transfer_move`** — no in-table Reserved state; alloc failure does not restore source (`STUB(track1b)`). **Orphaned state:** resource leak only — cap in kernel locals unreachable by any process; not privilege escalation or cap duplication; object generation unaffected. |

## Gate conditions met

| Condition | Status |
|-----------|--------|
| `flat_status_header_matches_canonical_dst` | PASS — flat `superseded-by: docs/architecture/KERNEL_OBJECT_MODEL.md` |
| `canonical_has_mermaid_state_machine` | PASS — 4 Mermaid diagrams (≥1 universal lifecycle) |
| `link_check_passes` | PASS — `doc_link_check.py` |
| `status_header_hash` recorded | PASS — `4d27d658f74177581b4432824c778c5760b1dc54dc0f70bbb5833573c1bbed92` (`status: authoritative`) |

## Threat nodes

No new threat nodes opened. `T-transfer-toctou` remains **closed** (tier B happy path). Alloc-failure **Orphaned** path is classified as **resource leak only** (cap unreachable by any process; source receives error; no escalation or duplication) — documented in `cap_transfer_move` STUB co-located with the gap, not as a new threat node. Fix deferred to track1b reserved-slot protocol.

## Not in this PR

- Batch 2 migrations (FE, SM, RA)
- `CAP_TRANSFER_PROTOCOL.md` canonical spec (Batch 5 `CTP`)
- Physical move of `THREAT_NODES.toml` / `CAP_REGISTRY.toml` to `config/`
- `clan-rt` no_std fix (`clan-rt-001`)
- Workspace restructure (`servers/` extraction)
- `#![deny(warnings)]` on `kernel` lib crate (deny on bin `main.rs` only per scope-freeze)
- Opening `T-transfer-toctou` for receiver_failed rollback (track1b)
