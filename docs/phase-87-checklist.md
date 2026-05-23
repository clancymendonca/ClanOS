# Phase 87 Checklist: `PipeLite` Anonymous Pipe

## Scope

- [x] `Pipe = 80` syscall; ring buffer; pipe fds via `/@pipe/{id}/r|w`.
- [x] `read`/`write` delegate to pipe backend; `phase87_smoke`.
- [x] `Phase87-PipeLite` boot output.

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase87_pipe_lite_check.py --timeout 180`

## Deferred

- [ ] `poll`/`select`; socket pairs.
