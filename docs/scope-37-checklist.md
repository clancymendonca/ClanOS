> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 37 Checklist: Manifest-Discovered ELF Load

## Scope

- [x] Discover `elf64-image` manifests from storage.
- [x] Gated execution via `EXECUTION_ALLOWLIST` and `execute_manifest_elf_gated`.
- [x] Seed `/bin/tickprobe` fixture.
- [x] Covered by boot gate `sched_userspace` (`ClanOS-BootGate: name=sched_userspace ok=true`)

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --gate sched_userspace --timeout 180`

## Deferred

- [ ] Unsigned arbitrary ELF execution.
