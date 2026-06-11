# AresOS Security Governance

```yaml
status: authoritative
semantics_version: 1.0.0
```

Epoch-0 **governance** security policy. Kernel implementation security (credentials, file policy) remains in [`docs/SECURITY.md`](docs/SECURITY.md).

Full disclosure playbooks and key-compromise runbooks are required before **milestone 150** public release (`DESIGN_NORTH_STAR.md` scorecard row).

---

## Responsible disclosure

Report vulnerabilities to **security@aresos.dev** (placeholder inbox until public release).
Include reproduction steps, affected commit, and threat-node id if known.

---

## Epoch gate commit signing

Squash and epoch gate commits are **GPG-signed**. CI verifies signatures against the known-good key registry in `keys/reviewer-registry.toml` (epoch 0 stub).

Forged `epoch_signoffs/` manifests are rejected at gate.

---

## Key rotation

Named ceremony for reviewer key expiry, compromise, or departure:

1. New key added to registry with effective date
2. Historical commits remain verifiable via retired keys section
3. Charter acknowledgment for compromise-driven rotation

Procedure details deferred to milestone 150; attacker class acknowledged in `THREAT_MODEL.md`.

---

## Post-quantum migration

Re-evaluate signing algorithms against NIST PQC at milestone 150. Migration path noted now; no PQC requirement before 1.0.

---

## External breaking-change communications

None before milestone 150. Policy defined before public release.

---

## Domain reviewer qualification

Before epoch 0 gate:

- Each domain (**kernel**, **evidence**, **process**) has **primary + backup** reviewer
- New reviewers co-sign the first **2** epoch gates
- Qualification criteria: demonstrated review of one foundational doc cross-ref cycle

---

## Reviewer currency

Reviewers must participate in **≥1 epoch gate review per 2 epochs** (or 6 months, whichever is shorter).

Stale reviewers re-qualify via co-sign with an active reviewer.

**Solo / MV team:** same person may satisfy currency for multiple domains with documented `multi_domain_roles` in sign-offs (`CHARTER.md`).

---

## Insider threat

Formal proofs and tests verify **code as written**, not author intent.

Position: malicious or coerced contributors are a **residual risk** mitigated by:

- Second-reviewer requirement on TCB and `unsafe` changes (`UNSAFE_AUDIT.md`)
- GPG-signed gate commits and sign-off manifests
- Split domain review at epoch gates

Full insider-threat modeling in `THREAT_MODEL.md`.

---

## Dev signing key compromise

Rotation, artifact re-sign, revocation of prior images, and runtime response to revoked keys — **procedure deferred** to BUILD_INTEGRITY epoch 2; attacker class in `THREAT_MODEL.md`.

---

## Boot attestation

Deferred post-150 **or** epoch-6 hardware measurement chain per `ARCHITECTURE_TARGETS.md`.

---

## AI-assisted code review

AI-generated or AI-assisted code in **kernel TCB paths** requires the **same second-reviewer treatment** as human-authored `unsafe` code, with explicit reviewer attestation that the output was **understood and verified against the relevant `KERNEL_OBJECT_MODEL` invariants** — not merely that it compiles and passes tests.

See also [`docs/UNSAFE_AUDIT.md`](docs/UNSAFE_AUDIT.md).
