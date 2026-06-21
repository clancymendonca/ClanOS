> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 95 Checklist: `pipeprobe` Ring-3 HW ELF

## Scope

- [x] `/bin/pipeprobe` manifest + ELF seed; `MAX_FILES = 28`.
- [x] `HW_PIPE_PROBES` counter; kernel/HW pipe smoke.
- [x] Covered by validation gate `smp_depth` (`ClanOS-Gate: name=smp_depth ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate smp_depth --timeout 180`

## Deferred

- [ ] Default-on HW `sysret` on every CI target without feature gate.
