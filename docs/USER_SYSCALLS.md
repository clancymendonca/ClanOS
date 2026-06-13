# User Syscall Return ABI

Scope 19 adds a user-facing syscall register-frame ABI. It preserves the existing `invoke_raw` dispatcher and wraps it with user entry and return metadata. Scope 20 uses this controlled ABI as part of the guarded `/bin/hello` ELF MVP.

## ABI Records

A `UserRegisterFrame` records:

- syscall id
- first argument
- return value
- optional error

A `UserSyscallReturn` records the same fields plus whether control returned to the user context.

## Loader Flow

```mermaid
flowchart TD
Ring3Trampoline[Ring3Trampoline] --> UserRegisterFrame[UserRegisterFrame]
UserRegisterFrame --> SyscallDispatch[SyscallDispatch]
SyscallDispatch --> UserReturn[UserSyscallReturn]
UserReturn --> ProcessMetadata[Blocked UserSyscallReturned Process]
```

The loader exposes `run_user_syscall_probe(credentials, name)`. It prepares the controlled user path and dispatches a tick-count syscall probe through the user ABI.

## Shell And Smoke

The shell exposes:

- `bin usyscall <program>`
- `bin plans`

Boot emits:

```text
See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
```

## Safety Boundary

Scope 19 validates syscall entry/return metadata. It does not yet execute CPU `syscall`/`sysret` instructions or run arbitrary ELF syscall instructions. Scope 20 runs the seeded hello path through the guarded pipeline only.

## Hardware Syscall Table (Scopes 25–46)

Scope 25 enables real `syscall`/`sysret`. Scope 35 registers an allowlist in `user_syscall_hw::ALLOWED_HW_SYSCALLS`. Later scopes add:

| ID | Name | Scope |
|---:|------|-------|
| 1 | `GetTickCount` | 25 |
| 60 | `UserCopyProbe` | 26 |
| 61 | `ExitProcess` | 34 |
| 62 | `WaitProcess` | 34 |
| 63 | `ReadFileProbe` | 36 |
| 64 | `WriteFileProbe` | 36 |
| 65 | `ReadPathProbe` | 44 |
| 66 | `OpenFile` | 45 |
| 67 | `CloseFile` | 45 |
| 68 | `ReadFd` | 46 |
| 69 | `WriteFd` | 46 |

Arguments for FD and path syscalls use `rsi` / `rdx` in the hardware entry stub. See [FILE_DESCRIPTORS.md](FILE_DESCRIPTORS.md).

Boot smokes include `See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.
