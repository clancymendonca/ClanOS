> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 95 Checklist: `pipeprobe` Ring-3 HW ELF

## Scope

- [x] `/bin/pipeprobe` manifest + ELF seed; `MAX_FILES = 28`.
- [x] `HW_PIPE_PROBES` counter; kernel/HW pipe smoke.
- [x] Covered by boot gate `smp_depth` (`AresOS-BootGate: name=smp_depth ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 95 --timeout 180

## Deferred

- [ ] Default-on HW `sysret` on every CI target without feature gate.
