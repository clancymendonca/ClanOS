# ADR-0003: Loader-Signed `/bin/*` Exec Manifests

```yaml
status: authoritative
adr_id: ADR-0003
decision_date: 2026-06-21
revision_date: 2026-06-21
depends_on: ADR-0002
blocks: program_loader signature path, loader_security gate reclassification
q5_status: pending-golden-bytes
```

## Context

[ADR-0002](ADR-0002-signed-elf-production-gate.md) scoped epoch-450 Ed25519 verification to the **production gate pinned corpus** (`config/signed_elf_test_corpus/`, wire format `clan-signed-manifest-v1`). `build_integrity::verify_signed_user_elf_corpus()` and `kernel/src/signed_elf.rs` are **done** at gate `2.2.0`.

ADR-0002 explicitly deferred:

> Wiring signature verification into `program_loader` / general `/bin/*` load paths.

Today, [`program_loader`](../../../kernel/src/task/program_loader.rs) enforces **`clan-exec-v1`** manifests with optional `digest=sha256:` integrity for `trust=system` programs ([`SECURITY.md`](../../SECURITY.md)). That path has **no public-key signature**. `ClanOS-Gate: name=production ok=true` therefore means **pinned gate corpus verified** — not that arbitrary `/bin/*` ELFs are signature-checked ([`GATE_AUDIT_401_500.md`](../../GATE_AUDIT_401_500.md)).

This ADR closes the deferred loader path. **Q1–Q4 locked below; Q5 field list remains blocked on golden-byte fixtures** (same discipline as ADR-0002 PR1).

---

## Locked decisions

### Q1 — Manifest wire format: **Option A**

Extend **`clan-exec-v1`** with an optional trailing `sig=ed25519:<128-hex>` line. Do **not** introduce a third manifest dialect or sidecar file.

**Critical nuance (locked):** shared **`sig=ed25519:`** line syntax with ADR-0002; **distinct canonical signed body** from `clan-signed-manifest-v1`. Gate corpus signs `clan-signed-manifest-v1\nname=…\ndigest=…\ntrust=system\n` over `payload.bin`. Loader signs a **`clan-exec-v1`** canonical body over selected exec-manifest fields and ELF digest (exact field set: Q5). Two verifiers may share Ed25519 implementation code; they **must not** share canonical-body construction.

**Rejected:** sidecar `clan-signed-manifest-v1` (B); merged header replacing `clan-exec-v1` (C).

---

### Q2 — Trust anchor: **Option B — separate loader anchor (epoch 460)**

| Artifact | Role |
|----------|------|
| `config/trust_anchor_epoch450.toml` | **Unchanged** — ADR-0002 gate corpus only |
| `config/trust_anchor_epoch460_loader.toml` | **New** — loader `/bin/*` exec signing root (public key only in-repo) |

Kernel embeds **both** public keys at build time (separate `include_bytes!` / const sites). Host dev signing for loader fixtures uses a **separate** documented dev seed — never the epoch-450 gate seed.

**Rejected:** reusing epoch-450 anchor (A) — epoch-450 private key is a deterministic public dev seed scoped gate-corpus-only; loader trust rooted on it would let any repo clone derive the "production" signing key. **Rejected:** runtime trust store (C) for this epoch.

---

### Q3 — Transition: **Option D with enforced sunset (concrete triggers)**

Two trust classes — no silent "production ⇒ all `/bin/*` signed" claim:

| `trust=` value | Policy |
|----------------|--------|
| `system` | Digest-only (`digest=sha256:` over ELF bytes). **After loader epoch lands:** permitted **only** for program names listed in `config/loader_digest_only_allowlist.toml`. |
| `system-signed` | Requires valid `sig=ed25519:` against epoch-460 loader anchor + digest verify. Fail closed on parse/verify error. |

**New programs** added after implementation scope **460** must use `trust=system-signed` (or `trust=user` where applicable). They may **not** be added to the digest-only allowlist.

#### Sunset mechanism (machine-enforced, not aspirational)

Same discipline as `architecture_state_check.py` hard-denying `has_external_network=true`:

| Control | Location | Rule |
|---------|----------|------|
| Allowlist | `config/loader_digest_only_allowlist.toml` | Explicit program `name=` values grandfathered digest-only. **Empty file = migration complete.** |
| Sunset scope | `config/loader_signing_policy.toml` → `sunset_scope = 465` | Locked constant; amendment required to extend. |
| Grace flag | `architecture_state.toml` → `loader_digest_only_grace = true` | While `true`, non-empty allowlist is permitted. Must flip to `false` before scope **465** close. |
| Enforced check | `scripts/gate/loader_signing_sunset_check.py` (validation matrix) | **Fail CI** if: (1) `current_scope >= sunset_scope` AND allowlist non-empty; OR (2) `loader_digest_only_grace = false` AND allowlist non-empty; OR (3) any seed manifest uses `trust=system` for a name **not** on allowlist after scope **460** lands. |
| Scope source | `epoch_checklist.toml` active scope or `loader_signing_policy.toml` → `implementation_scope` | Check script reads numeric scope — no comment-only sunset. |
| Gate reclassification guard | `scripts/gate/gate_honesty_check.py` extension | `loader_security` may not move to "Real (digest+sig)" in `GATE_AUDIT_401_500.md` until allowlist empty **and** `loader_digest_only_grace = false`. |

**Migration target for scope 460–464:** shrink allowlist by signing seed programs (`trust=system-signed`); desktop/functional gates must pass with mixed inventory during grace. **Scope 465:** allowlist must be empty or CI fails — same shape as gap-registry `milestone-150-stub` prevention, but with a numeric gate trigger.

**Rejected:** hard cutover in one commit without allowlist (A) — breaks gates. **Rejected:** forward-only permanent digest tier (C) — calcifies exception class. **Rejected:** grace without enforced sunset (B alone).

---

### Q4 — Verification locus and gate semantics: **locked**

| Item | Decision |
|------|----------|
| Verify where | Kernel `program_loader` on exec path only |
| Fail mode | Reject load; increment `MANIFEST_ELF_REJECTED`; no audit-only warn path for `trust=system-signed` |
| Gate honesty | `loader_security` "Real (digest+sig)" only when pinned inventory in `GATE_AUDIT_401_500.md` lists signature-verified programs and Q3 sunset guards pass |
| Host role | Negative fixtures + `loader_signing_sunset_check.py`; host preflight does **not** substitute for kernel QEMU proof |

---

### Q5 — Canonical signed body for `clan-exec-v1`: **pending golden bytes**

**Status:** not lockable from prose. Implementation PR1 for this epoch must add:

- `config/loader_signed_exec/WIRE_FORMAT.txt` (normative)
- Golden signed octets + fixtures under `config/loader_signed_exec/` and `scripts/gate/fixtures/loader_signed/`
- Open field questions to resolve in PR1 before kernel hook merges:
  - Minimum signed fields: magic, `name`, `digest=sha256:` (recomputed from ELF), `trust=` value — **confirm** whether `kind`, `entry`, `image`, `owner` bind to signature or remain integrity-only metadata
  - LF rules, `sig=` exclusion — mirror ADR-0002

**Blocked:** kernel `program_loader` signature hook until Q5 golden bytes land.

---

## Out of scope (this ADR)

- PKI, TPM, user-provided trust stores (scope 475+)
- Signing pipeline outside repo (document interface only)
- CAP_REGISTRY integration for code-signing roots
- Replacing digest-only path for `trust=user` allowlisted ELFs

---

## Alternatives considered (epoch level)

| Option | Outcome |
|--------|---------|
| Amend ADR-0002 in place | Rejected — gate corpus scope shipped |
| Reuse epoch-450 anchor for loader | Rejected — Q2 |
| Implement loader sig without ADR | Rejected — scope-honesty regression |
| Host-only loader verification | Rejected — ADR-0002 kernel-path precedent |

---

## Consequences

- Positive: closes ADR-0002 deferral with honest trust boundaries
- Positive: epoch-450 gate anchor stays gate-corpus-only; loader trust independently rotatable
- Negative: `VALIDATION_GATE_VERSION` bump, threat node, seed signing work, sunset check in matrix
- Negative: two trust classes during scopes 460–464 — audit docs must not overclaim until allowlist empty

---

## Security implications

- New attack surface: `clan-exec-v1` signature parser in `program_loader` — threat node required on implementation
- Epoch-450 dev seed must never verify loader exec manifests (separate anchor enforced at compile time)
- Fail closed on parse/verify error for `trust=system-signed`
- Allowlist shrink enforced by CI — prevents digest-only exception class becoming permanent

---

## Verification approach

- Host: `loader_signing_sunset_check.py` + signed-elf-style negative fixtures for loader manifests
- Kernel: QEMU integration — signed exec succeeds, bad sig / wrong trust class rejected
- Tier A: proptest on `clan-exec-v1` canonical body (bounded)
- Tier B: Kani on parser bounds if shared with `signed_elf.rs`
- Matrix: new entries; `loader_security` reclassification in `GATE_AUDIT_401_500.md`

---

## Implementation epoch (Q5 unblocks kernel PR)

Build order — separate PRs:

1. ~~Lock Q1–Q4~~ **Done** (this revision)
2. Q5 wire format + golden bytes + `trust_anchor_epoch460_loader.toml`
3. Host sign/verify tooling + fixtures + `loader_signing_sunset_check.py`
4. Kernel verifier hook in `program_loader` + negative QEMU gauntlet
5. Seed corpus migration (shrink allowlist) + gate/version bump + threat node + audit update
6. Scope **465**: allowlist empty, `loader_digest_only_grace = false`, sunset check green
