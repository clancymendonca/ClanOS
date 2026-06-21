> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 87 Checklist: `PipeLite` Anonymous Pipe

## Scope

- [x] `Pipe = 80` syscall; ring buffer; pipe fds via `/@pipe/{id}/r|w`.
- [x] `read`/`write` delegate to pipe backend; `smoke_pipe_lite`.
- [x] Covered by validation gate `path_exec` (`ClanOS-Gate: name=path_exec ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/run.py --gate path_exec --timeout 180`

## Deferred

- [ ] `poll`/`select`; socket pairs.
