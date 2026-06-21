# ADR-0002: Production Signed ELF Gate

```yaml
status: authoritative
adr_id: ADR-0002
decision_date: 2026-06-20
revision_date: 2026-06-20
supersedes_smoke: build_integrity::verify_signed_user_elf_corpus (hardcoded digest)
```

## Context

[`GATE_AUDIT_401_500.md`](../../GATE_AUDIT_401_500.md) classifies `production` gate signed-ELF leaf as **Hardcoded**: `verify_signed_user_elf_corpus()` hashes `b"clan-rt demo:hello"` and verifies against a manifest it generates inline. The roadmap falsifier "Signed ELF" requires digest-verified **user manifests**, not self-referential stubs.

[`architecture_state.toml`](../../../architecture_state.toml) and [`RELEASE_SCORECARD.md`](../../RELEASE_SCORECARD.md) already list signed ELF under production gate — current proof overclaims depth.

## Decision

Replace the hardcoded corpus smoke with a **trust-anchor verification path** scoped to the **production gate pinned corpus only** (epoch 450).

### Trust anchor (locked)

| Artifact | Role |
|----------|------|
| [`config/trust_anchor_epoch450.toml`](../../../config/trust_anchor_epoch450.toml) | Canonical **epoch-450 code-signing** trust anchor (public key material only) |
| Kernel | Verifies using public key embedded at build time (`include_bytes!` / const from anchor file) |
| Host (`scripts/gate/`) | Signs **gate test corpus only** via public deterministic dev seed ([`SECURITY.md`](../../SECURITY.md)); private key material is derivable by anyone with the repo — must not be reused for production userland |

**Not** rooted in [`docs/CAP_REGISTRY.toml`](../../CAP_REGISTRY.toml): capability grants and code-signing trust are separate concerns with separate rotation and threat-modeling. Conflating them is rejected.

Verifier rules:

- No caller-supplied expected digest (fail closed on parse/verify error).
- Signature verified over canonical manifest bytes using the embedded epoch-450 public key only.
- **Normative byte layout:** [`config/signed_elf_test_corpus/WIRE_FORMAT.txt`](../../../config/signed_elf_test_corpus/WIRE_FORMAT.txt). Golden signed octets: `canonical_body.utf8` in the same directory. Kernel verifier **must** be tested against committed fixture bytes verbatim, not regenerated from prose alone.

### Pinned gate corpus (locked scope)

1. **Pinned test corpus** in-repo: `config/signed_elf_test_corpus/` with sidecar manifest (`digest=sha256:…`, `trust=system`, `sig=…` over canonical manifest bytes).
2. **`verify_signed_user_elf_corpus()`** loads that corpus + manifest, verifies digest, then verifies signature against the epoch-450 anchor — no inline expected digest generation.
3. **`smoke_signed_user_elf()`** succeeds only when verification passes and `SIGNED_USER_ELF_VERIFIED` increments once per boot (idempotent guard retained).

**Out of scope for this ADR (follow-on epoch / ADR):**

- Wiring signature verification into [`program_loader`](../../../kernel/src/task/program_loader.rs) / general `/bin/*` load paths.
- Production out-of-kernel signing pipeline, PKI, TPM trust store (deferred scope 475+).

### Two trust mechanisms (do not conflate)

Clan OS already has a **manifest integrity** path unrelated to this ADR:

| Mechanism | Where | What it proves |
|-----------|--------|----------------|
| **Manifest digest (`clan-exec-v1`)** | `program_loader` / `execute_trusted_manifest_elf` | `digest=sha256:<hex>` over ELF bytes; `trust=system` policy gate — **integrity only, no public-key signature** ([`SECURITY.md`](../../SECURITY.md)) |
| **Signed ELF gate (this ADR)** | `build_integrity::verify_signed_user_elf_corpus` | Pinned corpus + manifest **cryptographic signature** vs epoch-450 trust anchor |

These may share line-oriented fields (`digest=sha256:`, `trust=system`) in manifest text; they are **not** the same feature under two names. This ADR **does not supersede** the loader digest path; it adds a separate signature layer for the production gate smoke. Loader integration requires its own ADR amendment.

### Host and gate enforcement

1. **Negative cases** in host check: tampered corpus, bad digest, bad sig, unsigned manifest, wrong-key sig (`scripts/gate/fixtures/signed_elf/`).
2. **`scripts/gate/signed_elf.py`** must fail on negative fixtures before the gate is trusted to pass (same discipline as `gate_honesty_check` self-tests).

Production userland signing pipeline (out-of-kernel) remains deferred; this ADR scopes **gate semantics** and kernel verification API for the pinned corpus only.

## Alternatives considered

| Option | Outcome |
|--------|---------|
| Keep hardcoded digest | Rejected — falsifier unchanged |
| Trust anchor in `CAP_REGISTRY.toml` | Rejected — capability vs code-signing separation |
| Full PKI / TPM trust store | Deferred — scope 475+; epoch 450 uses pinned epoch key |
| Host-only verification | Rejected — production serial line must reflect kernel path |
| Gate + loader in one epoch | Rejected — scope creep; gate corpus first |

## Consequences

- Positive: `production` gate signed leaf moves from Hardcoded toward Real **for the pinned test corpus**
- Positive: Host `signed_elf.py` can negative-test tamper cases (unsigned, wrong sig, tampered payload)
- Negative: Requires `VALIDATION_GATE_VERSION` bump when implementation merges
- Negative: Trust anchor rotation needs ADR amendment
- **Scope honesty:** Do **not** infer "userland binaries are cryptographically signed" from `ClanOS-Gate: name=production ok=true` alone. That serial line, after this ADR lands, means the **pinned gate corpus** passed epoch-450 signature verification — not that arbitrary loadable ELFs or `/bin/*` demos are signature-checked. See [`GATE_AUDIT_401_500.md`](../../GATE_AUDIT_401_500.md) and [`GATE_DESIGN_401_500.md`](../../GATE_DESIGN_401_500.md) §1.

## Security implications

- Attack surface: signature verification parser — threat node required on implementation
- No ambient trust: verification uses explicit anchor bytes, not caller-supplied expected digest
- Failure mode: verification error → `production` gate false (fail closed)

## Verification approach

- Host: `scripts/gate/signed_elf.py` + fixtures (good corpus, bad sig, bad digest, unsigned, wrong key)
- Kernel: `cargo test -p kernel --test signed_elf_integration` in QEMU — **required** execution proof; host checks do not substitute (separate Ed25519/parser stack). Matrix entry: `signed-elf-kernel-integration`.
- QEMU: `scripts/gate/run.py --gate production`
- Tier A: proptest on manifest parser (bounded)
- Tier B: Kani on digest compare bounds if parser in shared crate

## Implementation epoch (after this amendment)

Build order — separate PRs, negative tests at each layer:

1. `config/trust_anchor_epoch450.toml` + manifest wire format (`WIRE_FORMAT.txt`, `canonical_body.utf8` golden bytes)
2. Host sign + verify tooling + fixtures (must fail on negatives)
3. Kernel verifier (embedded pubkey) + swap `verify_signed_user_elf_corpus`
4. Gate swap + `VALIDATION_GATE_VERSION` bump + threat node + GATE_AUDIT_401_500 reclassification

See [`GATE_DESIGN_401_500.md`](../../GATE_DESIGN_401_500.md) §1.
