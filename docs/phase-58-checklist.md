> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 58 Checklist: Manifest Digest Trust

## Scope

- [x] `image_digest` SHA-256 module.
- [x] `digest=sha256:<hex>` manifest field; verify on trusted exec.
- [x] Covered by boot gate `fd_mmap` (`AresOS-BootGate: name=fd_mmap ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 58 --timeout 180

## Deferred

- [ ] Public-key signatures; TOFU policy.
