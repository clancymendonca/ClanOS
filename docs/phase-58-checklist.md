# Phase 58 Checklist: Manifest Digest Trust

## Scope

- [x] `image_digest` SHA-256 module.
- [x] `digest=sha256:<hex>` manifest field; verify on trusted exec.
- [x] Emit `Phase58-DigestTrust` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase58_digest_trust_check.py --timeout 180`

## Deferred

- [ ] Public-key signatures; TOFU policy.
