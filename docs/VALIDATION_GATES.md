# Clan OS Validation Gates



```yaml

status: authoritative

boot_gate_version: "1.0.0"

system_gate_version: "1.0.0"

kernel_modules:

  boot: kernel/src/boot_gate.rs

  system: kernel/src/system_gate.rs

scripts_package: scripts/gate/

```



Runtime validation is **gate-based**. Boot no longer emits `numbered boot serial` serial lines. Two gate families run sequentially at boot:



1. **Boot gate** (boot subsystems scope) → `ClanOS-BootGate: …`

2. **System gate** (epochs 7–20 / M500 scope) → `ClanOS-Gate: …` + `ClanOS-SystemGate: …`



## Quick validation



```bash

cargo check -p kernel

python scripts/gate/host.py

python scripts/gate/boot.py --gate boot --timeout 360

python scripts/gate/system.py --gate system --timeout 360

python scripts/validation_matrix.py --smoke-timeout 180

```



Legacy shims at `scripts/gate/boot.py` etc. forward to `scripts/gate/` and remain for older docs.



## Boot gate subsystems



| Gate | Legacy milestone (docs) | Serial line |

|------|-------------|-------------|

| `shell_storage` | 6–8 | `ClanOS-BootGate: name=shell_storage ok=true` |

| `loader_security` | 9–13 | `ClanOS-BootGate: name=loader_security ok=true` |

| `memory_layout` | 14–16 | `ClanOS-BootGate: name=memory_layout ok=true` |

| `userspace_bootstrap` | 17–20 | `ClanOS-BootGate: name=userspace_bootstrap ok=true` |

| `hw_paging` | 21–30 | `ClanOS-BootGate: name=hw_paging ok=true` |

| `sched_userspace` | 31–40 | `ClanOS-BootGate: name=sched_userspace ok=true` |

| `dynamic_runtime` | 41–50 | `ClanOS-BootGate: name=dynamic_runtime ok=true` |

| `fd_mmap` | 51–60 | `ClanOS-BootGate: name=fd_mmap ok=true` |

| `vm_fork` | 61–70 | `ClanOS-BootGate: name=vm_fork ok=true` |

| `syscall_ring3` | 71–80 | `ClanOS-BootGate: name=syscall_ring3 ok=true` |

| `path_exec` | 81–90 | `ClanOS-BootGate: name=path_exec ok=true` |

| `smp_depth` | 91–100 | `ClanOS-BootGate: name=smp_depth ok=true` |

| `constitutional` | 110 | `ClanOS-BootGate: name=constitutional ok=true` |

| `capabilities` | 120 | `ClanOS-BootGate: name=capabilities ok=true` |

| `service_loader` | 121 | `ClanOS-BootGate: name=service_loader ok=true` |

| `platform_brokers` | 130 | `ClanOS-BootGate: name=platform_brokers ok=true` |

| `build_endpoints` | 131–140 | `ClanOS-BootGate: name=build_endpoints ok=true` |

| `virtio_blk` | 201 | `ClanOS-BootGate: name=virtio_blk ok=true` |

| `network_compat` | 404 | `ClanOS-BootGate: name=network_compat ok=true` |

| `scheduler_epoch` | 149 | `ClanOS-BootGate: name=scheduler_epoch ok=true` |

| `boundary` | 150 | `ClanOS-BootGate: name=boundary ok=true` |

| **boot** (summary) | all above | `ClanOS-BootGate: ok=true` |



Historical scope indices map to gates via `scripts/gate/map.py` (`gate_for_scope`). Prefer `python scripts/gate/boot.py --gate <name>` or `python scripts/gate/system.py --gate <name>`.



## System gate subsystems



| Gate | Role | Serial line |

|------|------|-------------|

| `integrity` | Build integrity, audit, OOM | `ClanOS-Gate: name=integrity ok=true` |

| `scheduling` | Unified service scheduling | `ClanOS-Gate: name=scheduling ok=true` |

| `hardware` | Virtio + SDK path | `ClanOS-Gate: name=hardware ok=true` |

| `federation` | Federation + observability | `ClanOS-Gate: name=federation ok=true` |

| `release` | Checkpoint, scorecard | `ClanOS-Gate: name=release ok=true` |

| `desktop_preview` | Compositor preview | `ClanOS-Gate: name=desktop_preview ok=true` |

| `desktop` | Full desktop stack | `ClanOS-Gate: name=desktop ok=true` |










| `compat_runtime` | Ring-3 clan-rt corpus | `ClanOS-Gate: name=compat_runtime ok=true` |
| `compat_fd_vm` | FD / mmap / CoW | `ClanOS-Gate: name=compat_fd_vm ok=true` |
| `compat_signal` | Signal skeleton + delivery | `ClanOS-Gate: name=compat_signal ok=true` |
| `storage_depth` | Buddy / VFS / ext2 | `ClanOS-Gate: name=storage_depth ok=true` |
| `posix_compat` | POSIX server skeleton | `ClanOS-Gate: name=posix_compat ok=true` |






| **system** (summary) | all above | `ClanOS-SystemGate: ok=true` |



Use `scripts/gate/system.py --gate <name>` for individual subsystems.



## Preemption validation



Not part of boot/system gate serial lines. Kernel emits:



- `ClanOS-Preemption: name=fairness T1=… T2=… T3=… T4=… score=…`

- `ClanOS-Preemption: name=latency ticks=… req=… est_ms=…`



Requires `cargo run -p kernel --features preemption` (context lab; CI only).



```bash

python scripts/preemption/soak.py --boot-wait 90 --duration 30

python scripts/preemption/latency.py --boot-wait 90 --duration 30

```



Or `scripts/validation_matrix.py` (`preemption-soak-check`, `preemption-latency-check`).



## Scope checklists (historical)



Per-scope checklists under `docs/scope-*-checklist.md` record **implementation scope** for completed work. They are not the runtime validation surface. Use this document and `scripts/gate/` for CI and QEMU smokes.



## CI matrix entries



| Check | Script |

|-------|--------|

| `gate-host-check` | `scripts/gate/host.py` |

| `boot-gate-host-check` | `scripts/gate/boot_host.py` |

| `boot-gate-check` | `scripts/gate/boot.py --gate boot` |

| `system-gate-host-check` | `scripts/gate/system_host.py` |

| `system-gate-check` | `scripts/gate/system.py --gate system` |

| `compat-subsystems-host-check` | `scripts/gate/compat_subsystems.py` |

| `preemption-soak-check` | `scripts/preemption/soak.py` |

| `preemption-latency-check` | `scripts/preemption/latency.py` |



See also [`RELEASE_SCORECARD_M500.md`](RELEASE_SCORECARD_M500.md).

