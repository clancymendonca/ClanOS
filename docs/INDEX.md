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
| 81 | Real HW `syscall`/`sysret` | [phase-81-checklist.md](phase-81-checklist.md) |
| 82 | `getcwd` syscall | [phase-82-checklist.md](phase-82-checklist.md) |
| 83 | `chdirprobe` ELF | [phase-83-checklist.md](phase-83-checklist.md) |
| 84 | VMA in-region split | [phase-84-checklist.md](phase-84-checklist.md) |
| 85 | Fork-lite CR3 duplicate | [phase-85-checklist.md](phase-85-checklist.md) |
| 86 | `ExecLite` + close-on-exec | [phase-86-checklist.md](phase-86-checklist.md) |
| 87 | `PipeLite` ring buffer | [phase-87-checklist.md](phase-87-checklist.md) |
| 88 | Ring 3 PLT `#PF` lazy bind | [phase-88-checklist.md](phase-88-checklist.md) |
| 89 | LAPIC IPI send stub | [phase-89-checklist.md](phase-89-checklist.md) |
| 90 | Integration (81–89) | [phase-90-checklist.md](phase-90-checklist.md) |
| 91 | Fork-lite COW break | [phase-91-checklist.md](phase-91-checklist.md) |
| 92 | `PollLite` syscall | [phase-92-checklist.md](phase-92-checklist.md) |
| 93 | Gap-aware `mmap` hint | [phase-93-checklist.md](phase-93-checklist.md) |
| 94 | `ExecLite` argv from user | [phase-94-checklist.md](phase-94-checklist.md) |
| 95 | `pipeprobe` HW pipe ELF | [phase-95-checklist.md](phase-95-checklist.md) |
| 96 | VMA adjacent coalesce | [phase-96-checklist.md](phase-96-checklist.md) |
| 97 | Work-stealing stub | [phase-97-checklist.md](phase-97-checklist.md) |
| 98 | AP runnable enqueue stub | [phase-98-checklist.md](phase-98-checklist.md) |
| 99 | LAPIC ICR write stub | [phase-99-checklist.md](phase-99-checklist.md) |
| 100 | Integration (91–99) | [phase-100-checklist.md](phase-100-checklist.md) |

## Post-100 Constitutional Architecture

Governance framework for semantic integrity — documentation pass phases **101–110** complete; implementation **111+** gated on phase 110 sign-off.

| Guide | Role |
|-------|------|
| [ROADMAP_POST100.md](ROADMAP_POST100.md) | Phases 101–150 + beyond 150 |
| [NATIVE_MODEL.md](NATIVE_MODEL.md) | Post-Unix native definition, hierarchy, layers |
| [AXIOMS.md](AXIOMS.md) | Constitutional axioms A1–A10, gates G1–G5 |
| [KERNEL_OBJECT_MODEL.md](KERNEL_OBJECT_MODEL.md) | Universal objects; immutable identity + generation (G1) |
| [RIGHTS_ALGEBRA.md](RIGHTS_ALGEBRA.md) | Authority calculus (G2) |
| [TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md) | Visibility; meta-semantics outline (G5) |
| [SEMANTIC_SPECS.md](SEMANTIC_SPECS.md) | Architecture-preservation cases R-/E-/T-/M-/S- |
| [SEMANTIC_JURISDICTION.md](SEMANTIC_JURISDICTION.md) | Who may define semantics |
| [SEMANTIC_LINT.md](SEMANTIC_LINT.md) | Semantic static analysis roadmap |
| [SEMANTIC_OBSERVABILITY.md](SEMANTIC_OBSERVABILITY.md) | Law-linked diagnostics (outline post-170) |
| [ABI_SYSCALL.md](ABI_SYSCALL.md) | Compat `ares-abi-v1` |
| [ABI_MEMORY.md](ABI_MEMORY.md) | VMA, mmap, COW (compat) |
| [ABI_IPC.md](ABI_IPC.md) | Endpoint guarantees (G3) |
| [ABI_ASYNC.md](ABI_ASYNC.md) | Async OS contract |
| [ABI_RUNTIME.md](ABI_RUNTIME.md) | Language-neutral native runtime |
| [ABI_DRIVER.md](ABI_DRIVER.md) | Distrustful drivers |
| [ABI_SECURITY.md](ABI_SECURITY.md) | No ambient authority |
| [ABI_STABILITY.md](ABI_STABILITY.md) | `ares-abi-v*` + `ares-semantics-v*` |
| [NATIVE_DEVELOPER_EXPERIENCE.md](NATIVE_DEVELOPER_EXPERIENCE.md) | UX vs compat retreat |

## Phase Checklists (101–150)

| Phase | Topic | Checklist |
|------:|-------|-----------|
| 101 | Compat syscall ABI freeze | [phase-101-checklist.md](phase-101-checklist.md) |
| 102 | Memory contract | [phase-102-checklist.md](phase-102-checklist.md) |
| 103 | IPC endpoint guarantees | [phase-103-checklist.md](phase-103-checklist.md) |
| 104 | Async OS contract | [phase-104-checklist.md](phase-104-checklist.md) |
| 105 | Security + axioms | [phase-105-checklist.md](phase-105-checklist.md) |
| 106 | Kernel object model | [phase-106-checklist.md](phase-106-checklist.md) |
| 107 | Rights algebra | [phase-107-checklist.md](phase-107-checklist.md) |
| 108 | Temporal semantics | [phase-108-checklist.md](phase-108-checklist.md) |
| 109 | Semantic index + lint + jurisdiction | [phase-109-checklist.md](phase-109-checklist.md) |
| 110 | Constitutional sign-off | [phase-110-checklist.md](phase-110-checklist.md) |
| 111–119 | Capabilities + compat bridge | [phase-111-checklist.md](phase-111-checklist.md) … [phase-119-checklist.md](phase-119-checklist.md) |
| 120 | Integration (111–119) | [phase-120-checklist.md](phase-120-checklist.md) |
| 121–129 | Platform brokers | [phase-121-checklist.md](phase-121-checklist.md) … [phase-129-checklist.md](phase-129-checklist.md) |
| 130 | Integration (121–129) | [phase-130-checklist.md](phase-130-checklist.md) |
| 131–139 | Immutable + async IPC | [phase-131-checklist.md](phase-131-checklist.md) … [phase-139-checklist.md](phase-139-checklist.md) |
| 140 | Integration (131–139) | [phase-140-checklist.md](phase-140-checklist.md) |
| 141–149 | Scheduler, drivers, QoS | [phase-141-checklist.md](phase-141-checklist.md) … [phase-149-checklist.md](phase-149-checklist.md) |
| 150 | Four-layer review | [phase-150-checklist.md](phase-150-checklist.md) |

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

Post-100 constitutional foundation (phases 101–110):

```bash
python scripts/semantic_lint.py
python scripts/phase110_constitutional_check.py --timeout 300
```

Matrix entries: `semantic-lint-check`, `phase110-constitutional-check`

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
