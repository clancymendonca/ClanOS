# Phase 28 Checklist: Hardware Hello Execution

## Scope

- [x] Run `hello` through hardware syscall path.
- [x] Preserve `hello: exit=0 tick=...` output format.
- [x] Add blocked `UserHwElfExited` process metadata.
- [x] Emit `Phase28-HwHello` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --test preemption_integration`
- [x] `python scripts/phase28_hw_hello_check.py --timeout 120`

## Deferred

- [ ] Multiple allowlisted programs.
