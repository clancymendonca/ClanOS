# ADR-0003: Loader-Signed `/bin/*` Exec Manifests

```yaml
status: authoritative
adr_id: ADR-0003
decision_date: 2026-06-21
revision_date: 2026-06-21
depends_on: ADR-0002
blocks: program_loader signature path, loader_security gate reclassification
q5_status: locked-pr1-host
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

### Q3 — Transition: **Option D with enforced sunset (concrete triggers locked)**

**Locked constants** (same weight as Q2 anchor separation — amendment required to change):

| Constant | Value | Enforced by |
|----------|-------|-------------|
| `implementation_scope` | **460** | `config/loader_signing_policy.toml` |
| `sunset_scope` | **465** | `config/loader_signing_policy.toml` |
| `loader_digest_only_grace` | **`true` until allowlist empty** | `architecture_state.toml` — must be `false` before scope 465 close |
| CI fail rule | **`current_scope >= 465` AND allowlist non-empty** | `scripts/gate/loader_signing_sunset_check.py` (hard exit 1) |

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

#### Seed migration workflow (locked — scopes 461–465)

Verification machinery (PR1–PR2, anchor guard) is proven on **pinned synthetic corpora**. Seed migration is the first touch of **real `/bin/*` binaries** the system depends on. **Not a batch cutover.**

| Rule | Decision |
|------|----------|
| Unit of work | **One program per commit/PR** — sign manifest, run gates, shrink allowlist by one `name=` |
| Rollback | **Not a one-way door.** Wrong sig, bad `kind`/`entry`, or typo → revert manifest to `trust=system` (digest-only) and **re-add** `name=` to `loader_digest_only_allowlist.toml` in the same revert commit. Digest-only path remains available while `loader_digest_only_grace = true`. |
| Allowlist role | **Staging safety net**, not permanent exception bucket. A name stays on the allowlist until its signed manifest is confirmed working; only then remove it. |
| Gate bar before shrink | Host: `test_loader_signed_exec.py` + `loader_signed_exec.py` on that manifest bytes. Matrix: `run.py --gate` smokes covering the program (e.g. `functional`, `desktop`, `dynamic_runtime` as applicable). QEMU integration already proves verifier; per-binary PR must prove **that binary** still runs. |
| Progress metric | **`len(allowlist)`** — sunset scope 465 is a countdown (16 → 0 today), not a single event. Track in commit messages / scope checklist. |
| Batch signing | **Rejected** — do not empty allowlist in one commit; defeats rollback and obscures which binary broke gates. |
| Re-add to allowlist | Permitted during scopes 461–464 for rollback only; new programs after 460 still may not join allowlist (Q3). |

Suggested order (lowest gate blast radius first): shell builtins (`echo`, `time`, …) → demos (`demo-hello`, `clan-info`) → probes → desktop paths (`mendo`, `ring3-io-demo`, …). Exact order is implementation choice; **one-at-a-time invariant is not.**

**Rejected:** hard cutover in one commit without allowlist (A) — breaks gates. **Rejected:** forward-only permanent digest tier (C) — calcifies exception class. **Rejected:** grace without enforced sunset (B alone).

---

### Q4 — Verification locus and gate semantics: **locked**

| Item | Decision |
|------|----------|
| Verify where | Kernel `program_loader` on exec path only |
| Fail mode | Reject load; increment `MANIFEST_ELF_REJECTED`; no audit-only warn path for `trust=system-signed` |
| Verify ordering | **Parse-tolerant, verify-authoritative:** unsigned metadata and non-signed fields are parsed without short-circuiting before Ed25519; tamper/forgery rejection is at signature verification (not parse-order accidents). Malformed input still fails closed. |
| Gate honesty | `loader_security` "Real (digest+sig)" only when pinned inventory in `GATE_AUDIT_401_500.md` lists signature-verified programs and Q3 sunset guards pass |
| Host role | Negative fixtures + `loader_signing_sunset_check.py`; host preflight does **not** substitute for kernel QEMU proof |

---

### Q5 — Canonical signed body for `clan-exec-v1`: **locked (PR1 golden bytes)**

**Status:** locked in [`config/loader_signed_exec/WIRE_FORMAT.txt`](../../../config/loader_signed_exec/WIRE_FORMAT.txt) and golden octets `canonical_body.utf8`.

Signed fields (fixed order, LF-only body, `sig=` excluded):

1. `clan-exec-v1`
2. `name=`
3. `kind=` (`builtin-alias` | `elf64-image`)
4. `entry=`
5. `image=` — **only** when `kind=elf64-image`
6. `requires=execute`
7. `digest=sha256:` — recomputed from ELF/`payload.bin` at verify
8. `trust=system-signed`

**Not signed:** `owner=`, `description=`, `sig=`

**Rejected:** reusing ADR-0002 `clan-signed-manifest-v1` canonical body for loader manifests.

Host reference: `scripts/gate/loader_signed_exec_lib.py` (must not import ADR-0002 canonicalization). Kernel hook remains blocked until QEMU gauntlet epoch (PR2).

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

## Implementation epoch

Build order — separate PRs:

1. ~~Lock Q1–Q5~~ **Done** (ADR revision + PR1 host)
2. ~~Q5 wire format + golden bytes + `trust_anchor_epoch460_loader.toml`~~ **Done (PR1 host)**
3. ~~Host sign/verify tooling + fixtures + `loader_signing_sunset_check.py`~~ **Done (PR1 host)**
4. ~~Kernel verifier hook in `program_loader` + negative QEMU gauntlet~~ **Done (PR2)** — `loader_signed_exec.rs`, `loader_signed_exec_integration.rs`, `VALIDATION_GATE_VERSION` 2.3.0
4b. ~~Anchor embed guard~~ **Done** — `test_anchor_embed_match.py`
5. **Seed migration (next)** — one program per PR; see § Seed migration workflow; shrink allowlist only after gate green
6. Scope **465**: allowlist empty, `loader_digest_only_grace = false`, sunset check green
