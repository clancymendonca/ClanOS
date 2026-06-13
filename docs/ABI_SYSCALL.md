# Compat Syscall ABI (`clan-abi-v1`)

Frozen snapshot of scopes 1–100 **compat** syscall surface. Native syscall IDs are reserved separately (scope 107+); they must not redefine compat numbers.

See: [AXIOMS.md](AXIOMS.md), [USER_SYSCALLS.md](USER_SYSCALLS.md), [ABI_STABILITY.md](ABI_STABILITY.md), [ABI_SECURITY.md](ABI_SECURITY.md).

---

## Register ABI (x86_64 hardware path)

| Register | Role |
|----------|------|
| `rax` | Syscall ID |
| `rdi` | arg0 |
| `rsi` | arg1 |
| `rdx` | arg2 |

Entry: `syscall` instruction → kernel trampoline → `sysret` return.

Scope 81 validates real `sysret` path (`HW_SYSRET_REAL` counter).

---

## Allowlisted hardware syscalls (scope 100)

From `kernel/src/user_syscall_hw.rs` `ALLOWED_HW_SYSCALLS`:

| ID | Name | Notes |
|---:|------|-------|
| 1 | `GetTickCount` | |
| 60 | `UserCopyProbe` | copyin validation |
| 61 | `ExitProcess` | |
| 62 | `WaitProcess` | |
| 63 | `ReadFileProbe` | |
| 64 | `WriteFileProbe` | |
| 65 | `ReadPathProbe` | path string — **compat** |
| 66 | `OpenFile` | path — **compat** |
| 67 | `CloseFile` | |
| 68 | `ReadFd` | |
| 69 | `WriteFd` | |
| 70 | `DupFd` | |
| 71 | `Mprotect` | |
| 72 | `Mmap` | |
| 73 | `WritePathProbe` | |
| 74 | `Chdir` | |
| 75 | `Munmap` | |
| 76 | `ForkLite` | |
| 77 | `Fcntl` | |
| 78 | `WaitLite` | |
| 79 | `GetCwd` | |
| 80 | `Pipe` | PipeLite — **compat** IPC |
| 81 | `ExecLite` | |
| 82 | `Poll` | PollLite — **compat** |

Signal delivery syscalls (`83`–`86`) are documented in `docs/architecture/` when the signal subsystem is linked; they are not yet in the HW dispatch table.

Kernel-internal IDs 2–59 (counters, telemetry) exist in `SyscallId` but are not in the Ring 3 hardware allowlist unless invoked from kernel/shell paths.

---

## Error encoding (compat)

`SyscallError`: `InvalidSyscall`, `InvalidArgument`, `Storage` — returned via dispatcher; user copy paths use probe counters.

Native errors (future) will use a separate enum bound to `clan-semantics-v*`.

---

## Native ID space (kernel smokes; ring-3 allowlist deferred to scope 128 G4)

| Range | Owner |
|-------|--------|
| 1–82 | **Compat** (frozen `clan-abi-v1`) |
| 256+ | **Native** caps, endpoints, async wait |

Native syscalls must not expose path-string `open` for native processes ([ABI_SECURITY.md](ABI_SECURITY.md)).

## Native syscall IDs (scope 112)

Kernel-only dispatch via [`kernel/src/native_syscall.rs`](../kernel/src/native_syscall.rs) — not in `ALLOWED_HW_SYSCALLS` until scope 128.

| ID | Name | Notes |
|----|------|-------|
| 256 | `CapCreate` | Mint cap `(arg0=ObjectId, arg1=rights mask)` |
| 257 | `CapClose` | Close cap slot `arg0` |
| 258 | `CapTransfer` | Move cap `arg0` to process `arg1` |

---

## PipeLite compat bridge

`Pipe = 80` creates `/@pipe/{id}/r` and `/@pipe/{id}/w` FD paths ([kernel/src/pipe.rs](../kernel/src/pipe.rs)). Native **Endpoint** objects supersede this for native apps (scope 134+); compat unchanged (spec **E-*** compat preservation).
