# Track 1 scope-freeze commit template

Use this body verbatim (fill reviewer identity and commit hash after commit). Prefix: `scope-freeze(track1):` — not `feat`.

---

```
scope-freeze(track1): doc migration manifest — 30-day budget from this commit

Scope: migrate flat docs/FILENAME.md to canonical layers under docs/architecture/,
docs/specs/, docs/process/, and docs/proofs/.
Implementation work: zero. Config changes: track1_scope_freeze.toml + migration_baselines.json stub only.
Code changes: #![deny(warnings)] on kernel only.

Subdirectory policy (explicit decision):
  Track 1 CREATES docs/specs/, docs/process/, and docs/proofs/ alongside docs/architecture/.
  Interface specs (ABI, wire, IPC) → docs/specs/. Governance/audit policy → docs/process/.
  Proof ledgers → docs/proofs/. Architecture models → docs/architecture/.
  Rationale: avoids wrong grouping and a second migration epoch.

Resolved open questions:

  1. Batch 5 inventory: 31 [[docs]] entries (see config/track1_scope_freeze.toml).
     RA moved to Batch 2 per prereq_graph PROOF_COVERAGE → RIGHTS_ALGEBRA edge.
     Excluded: scope-*.md, ROADMAP_*.md, RELEASE_SCORECARD_*.md, epoch graduation docs,
     PLAN_SUPERSESSION.md (archived/superseded), docs/architecture/SECURITY_MODEL.md (already canonical).

  2. THREAT_NODES.toml (TN): canonical location is docs/THREAT_NODES.toml at M400 — NOT config/.
     config/README.md maps config/THREAT_NODES.toml → docs/THREAT_NODES.toml (pointer layer).
     Batch 3 TN is cross-reference parity verification only; zero open nodes (STATUS.md);
     physical move to config/ explicitly deferred (exclusions.threat_nodes_physical_move).

  3. THREAT_MODEL.md (TM) gate adds: every_threat_node_id_in_tm_exists_in_tn,
     attacker_goal_taxonomy_present (privilege_escalation, information_disclosure,
     denial_of_service, integrity_violation), tn_depends_on_all_resolve.

  4. Domain reviewers: solo-dev satisfies all three domains per CHARTER.md §Minimum viable team.
     Audit trail: epoch_signoffs/track1.toml with multi_domain_justification recorded.

Migration batches (machine-readable: config/track1_scope_freeze.toml):

Batch 1 (unblocked):
  KOM  — docs/KERNEL_OBJECT_MODEL.md → docs/architecture/ (reconcile flat vs existing canonical)

Batch 2 (blocks on Batch 1; mutual deps via CROSS-REF stubs until squash):
  FE   — docs/FAULT_ESCALATION.md → docs/architecture/FAULT_ESCALATION.md
  SM   — docs/SCHEDULER_MODEL.md → docs/architecture/SCHEDULER_MODEL.md
  RA   — docs/RIGHTS_ALGEBRA.md → docs/architecture/RIGHTS_ALGEBRA.md
         (prereq_graph: PROOF_COVERAGE depends on RA — must precede Batch 4 PC)

Batch 3 (blocks on Batch 1; atomic pair):
  TM   — docs/THREAT_MODEL.md → docs/architecture/THREAT_MODEL.md
  TN   — docs/THREAT_NODES.toml → docs/THREAT_NODES.toml (verify parity; no file move)

Batch 4 (blocks on Batch 2 + Batch 3):
  ET   — docs/ERROR_TAXONOMY.md → docs/architecture/ERROR_TAXONOMY.md
  PC   — docs/PROOF_COVERAGE.md → docs/architecture/PROOF_COVERAGE.md
  KS   — docs/KANI_SCOPE.md → docs/architecture/KANI_SCOPE.md

Batch 5 (blocks on Batch 4; 31 entries — prereq_graph.toml order):

  Architecture (11):
    TS   TEMPORAL_SEMANTICS      → docs/architecture/
    GC   GENERATION_COUNTER      → docs/architecture/
    MSB  MEMORY_SAFETY_BOUNDARY  → docs/architecture/
    AUD  AUDIT_SUBSYSTEM         → docs/architecture/
    CIS  COMPAT_ISOLATION        → docs/architecture/
    ATP  ARCHITECTURE_TARGETS    → docs/architecture/
    VSA  VIRTIO_SAFETY           → docs/architecture/
    DNS  DESIGN_NORTH_STAR       → docs/architecture/ (reconcile scorecard rows)
    DRM  DRIVER_MODEL            → docs/architecture/
    SUN  SCHEDULING_UNIFIED      → docs/architecture/
    FML  FORMAL_MODEL            → docs/architecture/

  Specs (7):
    CTP  CAP_TRANSFER_PROTOCOL   → docs/specs/
    WSR  WIRE_SCHEMA_REGISTRY    → docs/specs/
    IVN  IPC_VERSION_NEGOTIATION → docs/specs/
    ANS  ABI_NATIVE_SYSCALL      → docs/specs/
    AAR  ABI_CLAN_RT             → docs/specs/
    ACP  ABI_COMPOSITOR_IPC      → docs/specs/
    PCH  PROTOCOL_CHANGELOG       → docs/specs/

  Process (9):
    UAD  UNSAFE_AUDIT            → docs/process/
    EFP  EPOCH_FAILURE_PROCEDURE → docs/process/
    SCP  SUPPLY_CHAIN_POLICY     → docs/process/
    DPP  DEPENDENCY_POLICY       → docs/process/
    BLD  BUILD_INTEGRITY         → docs/process/
    CSN  COMPAT_SUNSET           → docs/process/
    DLD  DECISION_LOG            → docs/process/ (root DECISION_LOG.md preserved)
    CTR  CHARTER.md              → root (pointer stub in docs/process/)
    SEC  SECURITY.md             → root (pointer stub in docs/process/)

  Proofs (2):
    FZT  FUZZ_TARGETS            → docs/proofs/
    LVP  LIVENESS_PROPERTIES      → docs/proofs/

  Config verify (1):
    CRG  CAP_REGISTRY.toml       → docs/ (sync verify only; no move)

Kernel warning resolution:
  #![deny(warnings)] added to kernel/src/main.rs in this commit or immediate fixup.
  Pre-existing warning count: 3. Required count at gate: 0.

Semantic diff baseline:
  config/migration_baselines.json populated at each migration PR merge.
  CI flags canonical doc edits that reduce mermaid_diagram_count or cross_ref_count below baseline.

Exclusions (charter-approved deferrals):
  - No workspace restructure (servers/ extraction)
  - No clan-rt no_std fix (tracked: clan-rt-001)
  - No new docs added to canonical layers beyond [[docs]] manifest
  - No physical THREAT_NODES.toml or CAP_REGISTRY.toml move to config/
  - docs/architecture/SECURITY_MODEL.md already canonical — reference update only at gate

Triage fallback (if 30-day budget exceeded):
  Gate on KOM + FE + SM only. Remainder becomes track1b.

Domain sign-offs required before squash gate:
  security:    solo-dev (multi-domain per CHARTER.md §Minimum viable team)
  kernel_abi:  solo-dev (multi-domain per CHARTER.md §Minimum viable team)
  process:     solo-dev (multi-domain per CHARTER.md §Minimum viable team)

Sign-off manifest: epoch_signoffs/track1.toml

scope_freeze_commit: <filled after commit>
```

---

## Post-commit checklist

1. Record commit hash in `config/track1_scope_freeze.toml` → `scope_freeze_commit`
2. Record same hash in `epoch_signoffs/track1.toml` → `scope_freeze_commit`
3. Open Batch 1 staging PR for KOM reconciliation (does not block on scope-freeze review)
4. 30-day clock starts at scope-freeze commit timestamp
