# AresOS Documentation Index

Central index for validation gates, deep-dive guides, and historical scope checklists. The public roadmap lives in the root [README](../README.md).

## Validation (authoritative)

| Document | Role |
|----------|------|
| **[VALIDATION_GATES.md](VALIDATION_GATES.md)** | Boot + system gate serial lines, scripts, CI matrix |
| [RELEASE_SCORECARD_M500.md](RELEASE_SCORECARD_M500.md) | M500 release criteria |
| [RELEASE_SCORECARD_M400.md](RELEASE_SCORECARD_M400.md) | M400 functional OS criteria |
| [RELEASE_SCORECARD_M350.md](RELEASE_SCORECARD_M350.md) | Release 1.0 criteria |

Quick checks:

```bash
cargo check -p kernel
python scripts/gate/host.py
python scripts/gate/boot.py --gate boot --timeout 360
python scripts/gate/system.py --gate system --timeout 360
```

Full matrix:

```bash
python scripts/validation_matrix.py --smoke-timeout 180
```

Canonical scripts live under `scripts/gate/` and `scripts/preemption/`. Legacy shims at `scripts/gate/boot.py` forward to the gate package.

## Boot gate subsystems (phases 6–150)

| Gate | Scope | Check |
|------|-------|-------|
| `shell_storage` | 6–8 | `gate/boot.py --gate shell_storage` |
| `loader_security` | 9–13 | `--gate loader_security` |
| `memory_layout` | 14–16 | `--gate memory_layout` |
| `userspace_bootstrap` | 17–20 | `--gate userspace_bootstrap` |
| `hw_paging` | 21–30 | `--gate hw_paging` |
| `sched_userspace` | 31–40 | `--gate sched_userspace` |
| `dynamic_runtime` | 41–50 | `--gate dynamic_runtime` |
| `fd_mmap` | 51–60 | `--gate fd_mmap` |
| `vm_fork` | 61–70 | `--gate vm_fork` |
| `syscall_ring3` | 71–80 | `--gate syscall_ring3` |
| `path_exec` | 81–90 | `--gate path_exec` |
| `smp_depth` | 91–100 | `--gate smp_depth` |
| `constitutional` | 110 | `--gate constitutional` |
| `capabilities` | 120 | `--gate capabilities` |
| `service_loader` | 121 | `--gate service_loader` |
| `platform_brokers` | 130 | `--gate platform_brokers` |
| `build_endpoints` | 131–140 | `--gate build_endpoints` |
| `virtio_blk` | 201 | `--gate virtio_blk` |
| `network_compat` | 404 | `--gate network_compat` |
| `scheduler_epoch` | 149 | `--gate scheduler_epoch` |
| `boundary` | 150 | `--gate boundary` |

Module: `kernel/src/boot_gate.rs`

## System gate subsystems (epochs 7–20 / M500)

| Gate | Former milestone | Check |
|------|------------------|-------|
| `integrity` | Epoch 7 | `gate/system.py --gate integrity` |
| `scheduling` | M200 | `--gate scheduling` |
| `hardware` | M250 | `--gate hardware` |
| `federation` | M300 | `--gate federation` |
| `release` | M350 | `--gate release` |
| `desktop_preview` | Desktop preview | `--gate desktop_preview` |
| `desktop` | M375 | `--gate desktop` |
| `functional` | M400 | `--gate functional` |
| `ci` | M425 | `--gate ci` |
| `production` | M450 | `--gate production` |
| `network` | M475 | `--gate network` |
| `system` | M500 | `--gate system` |

Module: `kernel/src/system_gate.rs`

## Post-100 architecture guides

| Guide | Role |
|-------|------|
| [ROADMAP_POST100.md](ROADMAP_POST100.md) | Constitutional + capability foundation |
| [ROADMAP_151_350.md](ROADMAP_151_350.md) | Epochs 7–14 |
| [ROADMAP_351_400.md](ROADMAP_351_400.md) | Desktop + userland + network |
| [ROADMAP_401_500.md](ROADMAP_401_500.md) | Production SMP + signed ELF + M500 |
| [NATIVE_MODEL.md](NATIVE_MODEL.md) | Post-Unix native definition |
| [AXIOMS.md](AXIOMS.md) | Constitutional axioms A1–A10 |
| [KERNEL_OBJECT_MODEL.md](KERNEL_OBJECT_MODEL.md) | Universal objects; G1 |
| [RIGHTS_ALGEBRA.md](RIGHTS_ALGEBRA.md) | Authority calculus; G2 |
| [TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md) | Visibility; G5 |
| [SEMANTIC_SPECS.md](SEMANTIC_SPECS.md) | Architecture-preservation cases |
| [ABI_SYSCALL.md](ABI_SYSCALL.md) | Compat `ares-abi-v1` |
| [ABI_IPC.md](ABI_IPC.md) | Endpoint guarantees; G3 |

## Deep-dive guides

| Guide | Topics |
|-------|--------|
| [SCHEDULER.md](SCHEDULER.md) | Preemption, CR3 binding, runqueues |
| [STORAGE.md](STORAGE.md) | Filesystem, block devices |
| [DEVICES.md](DEVICES.md) | PCI, block manager |
| [PROGRAMS.md](PROGRAMS.md) | Loader, manifests, ELF |
| [SECURITY.md](SECURITY.md) | Credentials, trust |
| [EXECUTABLE_IMAGES.md](EXECUTABLE_IMAGES.md) | ELF validation |
| [USER_PAGE_TABLES.md](USER_PAGE_TABLES.md) | Paging, CR3, W^X |
| [USER_SYSCALLS.md](USER_SYSCALLS.md) | Syscall surface |
| [SMP.md](SMP.md) | APs, IPI, work-stealing |
| [context-lab.md](context-lab.md) | Cooperative context lab |

## Historical phase checklists

Per-phase checklists (`phase-*-checklist.md`) record **completed implementation scope**. They are not the runtime validation surface — use [VALIDATION_GATES.md](VALIDATION_GATES.md).

<details>
<summary>Phases 1–100 (click to expand file links)</summary>

| Phase | Topic | Checklist |
|------:|-------|-----------|
| 1 | Boot | [phase-1-checklist.md](phase-1-checklist.md) |
| 2 | Hardware / IRQ | [phase-2-checklist.md](phase-2-checklist.md) |
| 3 | Memory / paging | [phase-3-checklist.md](phase-3-checklist.md) |
| 4 | Processes | [phase-4-checklist.md](phase-4-checklist.md) |
| 5 | Preemptive scheduling | [phase-5-checklist.md](phase-5-checklist.md) |
| 6–100 | Userland through SMP integration | [phase-6-checklist.md](phase-6-checklist.md) … [phase-100-checklist.md](phase-100-checklist.md) |

</details>

<details>
<summary>Phases 101–500 (click to expand)</summary>

Constitutional (101–110), capabilities (111–120), platform brokers (121–130), build/IPC (131–140), scheduler epoch (141–149), boundary (150), post-150 roadmaps (151–500). Individual files: `phase-NNN-checklist.md`.

</details>

## Windows QEMU note

```powershell
taskkill /IM qemu-system-x86_64.exe /F
```

The validation matrix retries `llvm-objcopy` lock errors and pauses after `preemption-integration` on Windows.
