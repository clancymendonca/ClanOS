# Phase 95 Checklist: `pipeprobe` Ring-3 HW ELF

## Scope

- [x] `/bin/pipeprobe` manifest + ELF seed; `MAX_FILES = 28`.
- [x] `HW_PIPE_PROBES` counter; kernel/HW pipe smoke.
- [x] `Phase95-PipeProbe` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase95_pipe_probe_check.py --timeout 180` (with `hw-sysret-probe`)

## Deferred

- [ ] Default-on HW `sysret` on every CI target without feature gate.
