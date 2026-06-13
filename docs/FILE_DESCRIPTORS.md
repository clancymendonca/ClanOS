# File Descriptors and User Paths

Phases 44–46 expose bounded user/kernel data transfer for paths and file I/O through the hardware syscall table.

## Phase 44 — User Paths

`user_path::copy_path_from_user` validates a NUL-terminated path in user memory (length cap, no kernel pointers). Syscall `ReadPathProbe` (id 65) round-trips a probe string for smoke tests.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## Phase 45 — FD Table

Bring-up uses a global `fd_table` (spin `Mutex`) mapping small integer FDs to storage file indices. Syscalls:

- `OpenFile` (66) — open by path with permission checks
- `CloseFile` (67)

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## Phase 46 — FD I/O

- `ReadFd` (68) — read into user buffer with `copy_to_user`
- `WriteFd` (69) — write from user buffer with `copy_from_user`

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

Hardware syscall dispatch passes `arg1`/`arg2` from `rsi`/`rdx` for these calls.

## Validation

```bash
python scripts/gate/legacy.py --phase 44 --timeout 180
python scripts/gate/legacy.py --phase 45 --timeout 180
python scripts/gate/legacy.py --phase 46 --timeout 180
```

## Deferred

- Per-process FD tables and `dup` / `fcntl`
- Pipes, sockets, and non-storage backends
- `select` / `poll` integration
