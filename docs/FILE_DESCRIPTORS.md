# File Descriptors and User Paths

Phases 44–46 expose bounded user/kernel data transfer for paths and file I/O through the hardware syscall table.

## Phase 44 — User Paths

`user_path::copy_path_from_user` validates a NUL-terminated path in user memory (length cap, no kernel pointers). Syscall `ReadPathProbe` (id 65) round-trips a probe string for smoke tests.

Boot smoke:

```text
Phase44-UserPath: probes=..., ok=true
```

## Phase 45 — FD Table

Bring-up uses a global `fd_table` (spin `Mutex`) mapping small integer FDs to storage file indices. Syscalls:

- `OpenFile` (66) — open by path with permission checks
- `CloseFile` (67)

Boot smoke:

```text
Phase45-FileFd: opens=..., closes=..., ok=true
```

## Phase 46 — FD I/O

- `ReadFd` (68) — read into user buffer with `copy_to_user`
- `WriteFd` (69) — write from user buffer with `copy_from_user`

Boot smoke:

```text
Phase46-FdIO: reads=..., writes=..., ok=true
```

Hardware syscall dispatch passes `arg1`/`arg2` from `rsi`/`rdx` for these calls.

## Validation

```bash
python scripts/phase44_user_path_check.py --timeout 180
python scripts/phase45_file_fd_check.py --timeout 180
python scripts/phase46_fd_io_check.py --timeout 180
```

## Deferred

- Per-process FD tables and `dup` / `fcntl`
- Pipes, sockets, and non-storage backends
- `select` / `poll` integration
