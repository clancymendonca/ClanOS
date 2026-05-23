# AresOS Documentation Index

Central index for phase checklists, deep-dive guides, and validation commands. The public roadmap lives in the root [README](../README.md).

## Phase Checklists (1–70)

| Phase | Topic | Checklist |
|------:|-------|-----------|
| 1 | Boot | [phase-1-checklist.md](phase-1-checklist.md) |
| 2 | Hardware / IRQ | [phase-2-checklist.md](phase-2-checklist.md) |
| 3 | Memory / paging | [phase-3-checklist.md](phase-3-checklist.md) |
| 4 | Processes / cooperative tasks | [phase-4-checklist.md](phase-4-checklist.md) |
| 5 | Preemptive scheduling | [phase-5-checklist.md](phase-5-checklist.md) |
| 6 | User space shell | [phase-6-checklist.md](phase-6-checklist.md) |
| 7 | Persistent storage | [phase-7-checklist.md](phase-7-checklist.md) |
| 8 | Device / block drivers | [phase-8-checklist.md](phase-8-checklist.md) |
| 9 | Stored program loader | [phase-9-checklist.md](phase-9-checklist.md) |
| 10 | Permissions | [phase-10-checklist.md](phase-10-checklist.md) |
| 11 | Executable images | [phase-11-checklist.md](phase-11-checklist.md) |
| 12 | Load plans | [phase-12-checklist.md](phase-12-checklist.md) |
| 13 | Mapping stubs | [phase-13-checklist.md](phase-13-checklist.md) |
| 14 | Frame ownership | [phase-14-checklist.md](phase-14-checklist.md) |
| 15 | Frame-backed images | [phase-15-checklist.md](phase-15-checklist.md) |
| 16 | Inactive user page tables | [phase-16-checklist.md](phase-16-checklist.md) |
| 17 | User context | [phase-17-checklist.md](phase-17-checklist.md) |
| 18 | Ring 3 trampoline | [phase-18-checklist.md](phase-18-checklist.md) |
| 19 | Syscall return ABI | [phase-19-checklist.md](phase-19-checklist.md) |
| 20 | Minimal ELF MVP | [phase-20-checklist.md](phase-20-checklist.md) |
| 21 | Hardware page tables | [phase-21-checklist.md](phase-21-checklist.md) |
| 22 | CR3 activation | [phase-22-checklist.md](phase-22-checklist.md) |
| 23 | `iretq` user entry | [phase-23-checklist.md](phase-23-checklist.md) |
| 24 | Hardware user trap | [phase-24-checklist.md](phase-24-checklist.md) |
| 25 | `syscall` / `sysret` | [phase-25-checklist.md](phase-25-checklist.md) |
| 26 | User copyin | [phase-26-checklist.md](phase-26-checklist.md) |
| 27 | Static ELF relocations | [phase-27-checklist.md](phase-27-checklist.md) |
| 28 | Hardware hello | [phase-28-checklist.md](phase-28-checklist.md) |
| 29 | Allowlisted ELFs | [phase-29-checklist.md](phase-29-checklist.md) |
| 30 | Per-process CR3 | [phase-30-checklist.md](phase-30-checklist.md) |
| 31 | Scheduler CR3 binding | [phase-31-checklist.md](phase-31-checklist.md) |
| 32 | User trap frame persistence | [phase-32-checklist.md](phase-32-checklist.md) |
| 33 | Concurrent allowlisted ELFs | [phase-33-checklist.md](phase-33-checklist.md) |
| 34 | Exit / wait syscalls | [phase-34-checklist.md](phase-34-checklist.md) |
| 35 | HW syscall dispatch table | [phase-35-checklist.md](phase-35-checklist.md) |
| 36 | Storage syscalls + copyin | [phase-36-checklist.md](phase-36-checklist.md) |
| 37 | Manifest ELF load | [phase-37-checklist.md](phase-37-checklist.md) |
| 38 | Demand-zero paging | [phase-38-checklist.md](phase-38-checklist.md) |
| 39 | Dynamic linking groundwork | [phase-39-checklist.md](phase-39-checklist.md) |
| 40 | Integration (31–39) | [phase-40-checklist.md](phase-40-checklist.md) |
| 41 | Shared library mapping | [phase-41-checklist.md](phase-41-checklist.md) |
| 42 | Dynamic import relocs | [phase-42-checklist.md](phase-42-checklist.md) |
| 43 | Trust-gated execution | [phase-43-checklist.md](phase-43-checklist.md) |
| 44 | User path copyin | [phase-44-checklist.md](phase-44-checklist.md) |
| 45 | File descriptor table | [phase-45-checklist.md](phase-45-checklist.md) |
| 46 | FD read / write | [phase-46-checklist.md](phase-46-checklist.md) |
| 47 | File-backed demand paging | [phase-47-checklist.md](phase-47-checklist.md) |
| 48 | W^X mapping policy | [phase-48-checklist.md](phase-48-checklist.md) |
| 49 | SMP groundwork | [phase-49-checklist.md](phase-49-checklist.md) |
| 50 | Integration (41–49) | [phase-50-checklist.md](phase-50-checklist.md) |
| 51 | Per-process FD tables | [phase-51-checklist.md](phase-51-checklist.md) |
| 52 | Dup FD and CWD-relative open | [phase-52-checklist.md](phase-52-checklist.md) |
| 53 | mprotect and guard pages | [phase-53-checklist.md](phase-53-checklist.md) |
| 54 | mmap bring-up | [phase-54-checklist.md](phase-54-checklist.md) |
| 55 | User write path | [phase-55-checklist.md](phase-55-checklist.md) |
| 56 | Multiple shared libraries | [phase-56-checklist.md](phase-56-checklist.md) |
| 57 | PLT JUMP_SLOT relocs | [phase-57-checklist.md](phase-57-checklist.md) |
| 58 | Manifest digest trust | [phase-58-checklist.md](phase-58-checklist.md) |
| 59 | Per-CPU runqueue skeleton | [phase-59-checklist.md](phase-59-checklist.md) |
| 60 | Integration (51–59) | [phase-60-checklist.md](phase-60-checklist.md) |
| 61 | chdir and path normalization | [phase-61-checklist.md](phase-61-checklist.md) |
| 62 | munmap | [phase-62-checklist.md](phase-62-checklist.md) |
| 63 | Per-process VMA registry | [phase-63-checklist.md](phase-63-checklist.md) |
| 64 | Fork-lite FD inheritance | [phase-64-checklist.md](phase-64-checklist.md) |
| 65 | Ring 3 HW syscall probes | [phase-65-checklist.md](phase-65-checklist.md) |
| 66 | Minimal fcntl stub | [phase-66-checklist.md](phase-66-checklist.md) |
| 67 | Lazy PLT resolution | [phase-67-checklist.md](phase-67-checklist.md) |
| 68 | TLB shootdown accounting | [phase-68-checklist.md](phase-68-checklist.md) |
| 69 | AP idle accounting | [phase-69-checklist.md](phase-69-checklist.md) |
| 70 | Integration (61–69) | [phase-70-checklist.md](phase-70-checklist.md) |
| 71 | HW syscall/sysret return | [phase-71-checklist.md](phase-71-checklist.md) |
| 72 | Ring 3 chdir from user | [phase-72-checklist.md](phase-72-checklist.md) |
| 73 | munmap with length | [phase-73-checklist.md](phase-73-checklist.md) |
| 74 | WaitLite for fork-lite child | [phase-74-checklist.md](phase-74-checklist.md) |
| 75 | syscallprobe ELF manifest | [phase-75-checklist.md](phase-75-checklist.md) |
| 76 | fcntl F_SETFD / close-on-exec | [phase-76-checklist.md](phase-76-checklist.md) |
| 77 | Ring 3 lazy PLT first call | [phase-77-checklist.md](phase-77-checklist.md) |
| 78 | IPI TLB shootdown stub | [phase-78-checklist.md](phase-78-checklist.md) |
| 79 | AP idle trampoline entry | [phase-79-checklist.md](phase-79-checklist.md) |
| 80 | Integration (71–79) | [phase-80-checklist.md](phase-80-checklist.md) |

## Deep-Dive Guides

| Guide | Phases |
|-------|--------|
| [SCHEDULER.md](SCHEDULER.md) | 4–5, 31, 49 |
| [STORAGE.md](STORAGE.md) | 7–8, 36, 45–47 |
| [DEVICES.md](DEVICES.md) | 8 |
| [PROGRAMS.md](PROGRAMS.md) | 9–11, 37, 41–43 |
| [SECURITY.md](SECURITY.md) | 10, 29, 43 |
| [EXECUTABLE_IMAGES.md](EXECUTABLE_IMAGES.md) | 11 |
| [LOAD_PLANS.md](LOAD_PLANS.md) | 12 |
| [MAPPING_STUBS.md](MAPPING_STUBS.md) | 13 |
| [FRAME_OWNERSHIP.md](FRAME_OWNERSHIP.md) | 14 |
| [FRAME_BACKED_IMAGES.md](FRAME_BACKED_IMAGES.md) | 15 |
| [USER_PAGE_TABLES.md](USER_PAGE_TABLES.md) | 16, 21–22, 30–31, 48, 62–63 |
| [USER_CONTEXT.md](USER_CONTEXT.md) | 17 |
| [RING3_TRAMPOLINE.md](RING3_TRAMPOLINE.md) | 18, 23–24 |
| [USER_SYSCALLS.md](USER_SYSCALLS.md) | 19–20, 25–26, 34–36, 44–46 |
| [USER_ELF_MVP.md](USER_ELF_MVP.md) | 20, 28–29, 37, 43 |
| [SHARED_LIBRARIES.md](SHARED_LIBRARIES.md) | 39, 41–42 |
| [DEMAND_PAGING.md](DEMAND_PAGING.md) | 38, 47 |
| [FILE_DESCRIPTORS.md](FILE_DESCRIPTORS.md) | 45–46 |
| [SMP.md](SMP.md) | 49, 59, 68–69 |
| [context-lab.md](context-lab.md) | 4 (isolated lab) |

## Validation

Quick checks:

```bash
cargo check -p kernel
cargo test -p kernel --features preemption --test preemption_integration
```

Full QEMU matrix (serial; allow ~2+ hours on Windows):

```bash
python scripts/validation_matrix.py --soak-duration 30 --latency-duration 30 --boot-wait 90 --smoke-timeout 180
```

Resume from a named check:

```bash
python scripts/validation_matrix.py --from-check phase41-shared-lib-check --smoke-timeout 180
```

On Windows, stop stray QEMU before a matrix run:

```powershell
taskkill /IM qemu-system-x86_64.exe /F
```

The matrix retries `llvm-objcopy` lock errors and pauses after `preemption-integration` to reduce bootimage contention.
