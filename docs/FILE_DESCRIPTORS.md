# File Descriptors and User Paths

Scopes 44–46 expose bounded user/kernel data transfer for paths and file I/O through the hardware syscall table.

## Scope 44 — User Paths

`user_path::copy_path_from_user` validates a NUL-terminated path in user memory (length cap, no kernel pointers). Syscall `ReadPathProbe` (id 65) round-trips a probe string for smoke tests.

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## Scope 45 — FD Table

Per-process tables hold up to **64** open descriptors (`kernel/src/fd_table.rs::MAX_FDS`). Syscalls:

- `OpenFile` (66) — open by path with permission checks
- `CloseFile` (67)

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## Scope 46 — FD I/O

- `ReadFd` (68) — read into user buffer with `copy_to_user`
- `WriteFd` (69) — write from user buffer with `copy_from_user`

Boot smoke:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

Hardware syscall dispatch passes `arg1`/`arg2` from `rsi`/`rdx` for these calls.

## Validation

```bash
python scripts/gate/boot.py --gate dynamic_runtime --timeout 180
python scripts/gate/boot.py --gate dynamic_runtime --timeout 180
python scripts/gate/boot.py --gate dynamic_runtime --timeout 180
```

## Deferred

- Pipes, sockets, and non-storage backends beyond current pipe/serial paths
- `select` / `poll` integration
