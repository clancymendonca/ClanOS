# AresOS Validation Gates



```yaml

status: authoritative

boot_gate_version: "1.0.0"

system_gate_version: "1.0.0"

kernel_modules:

  boot: kernel/src/boot_gate.rs

  system: kernel/src/system_gate.rs

scripts_package: scripts/gate/

```



Runtime validation is **gate-based**. Boot no longer emits `PhaseN-*` serial lines. Two gate families run sequentially at boot:



1. **Boot gate** (phases 6–150 scope) → `AresOS-BootGate: …`

2. **System gate** (epochs 7–20 / M500 scope) → `AresOS-Gate: …` + `AresOS-SystemGate: …`



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



| Gate | Phase scope | Serial line |

|------|-------------|-------------|

| `shell_storage` | 6–8 | `AresOS-BootGate: name=shell_storage ok=true` |

| `loader_security` | 9–13 | `AresOS-BootGate: name=loader_security ok=true` |

| `memory_layout` | 14–16 | `AresOS-BootGate: name=memory_layout ok=true` |

| `userspace_bootstrap` | 17–20 | `AresOS-BootGate: name=userspace_bootstrap ok=true` |

| `hw_paging` | 21–30 | `AresOS-BootGate: name=hw_paging ok=true` |

| `sched_userspace` | 31–40 | `AresOS-BootGate: name=sched_userspace ok=true` |

| `dynamic_runtime` | 41–50 | `AresOS-BootGate: name=dynamic_runtime ok=true` |

| `fd_mmap` | 51–60 | `AresOS-BootGate: name=fd_mmap ok=true` |

| `vm_fork` | 61–70 | `AresOS-BootGate: name=vm_fork ok=true` |

| `syscall_ring3` | 71–80 | `AresOS-BootGate: name=syscall_ring3 ok=true` |

| `path_exec` | 81–90 | `AresOS-BootGate: name=path_exec ok=true` |

| `smp_depth` | 91–100 | `AresOS-BootGate: name=smp_depth ok=true` |

| `constitutional` | 110 | `AresOS-BootGate: name=constitutional ok=true` |

| `capabilities` | 120 | `AresOS-BootGate: name=capabilities ok=true` |

| `service_loader` | 121 | `AresOS-BootGate: name=service_loader ok=true` |

| `platform_brokers` | 130 | `AresOS-BootGate: name=platform_brokers ok=true` |

| `build_endpoints` | 131–140 | `AresOS-BootGate: name=build_endpoints ok=true` |

| `virtio_blk` | 201 | `AresOS-BootGate: name=virtio_blk ok=true` |

| `network_compat` | 404 | `AresOS-BootGate: name=network_compat ok=true` |

| `scheduler_epoch` | 149 | `AresOS-BootGate: name=scheduler_epoch ok=true` |

| `boundary` | 150 | `AresOS-BootGate: name=boundary ok=true` |

| **boot** (summary) | all above | `AresOS-BootGate: ok=true` |



Legacy phase numbers: `python scripts/gate/legacy.py --phase N` or `scripts/gate/boot.py --phase N`.



## System gate subsystems



| Gate | Former milestone | Serial line |

|------|------------------|-------------|

| `integrity` | Phase 175 / epoch 7 | `AresOS-Gate: name=integrity ok=true` |

| `scheduling` | Phase 200 | `AresOS-Gate: name=scheduling ok=true` |

| `hardware` | Phase 250 | `AresOS-Gate: name=hardware ok=true` |

| `federation` | Phase 300 | `AresOS-Gate: name=federation ok=true` |

| `release` | Phase 350 | `AresOS-Gate: name=release ok=true` |

| `desktop_preview` | Phase 351 | `AresOS-Gate: name=desktop_preview ok=true` |

| `desktop` | Phase 375 | `AresOS-Gate: name=desktop ok=true` |

| `functional` | Phase 400 | `AresOS-Gate: name=functional ok=true` |

| `ci` | Phase 425 | `AresOS-Gate: name=ci ok=true` |

| `production` | Phase 450 | `AresOS-Gate: name=production ok=true` |

| `network` | Phase 475 | `AresOS-Gate: name=network ok=true` |

| **system** (summary) | Phase 500 | `AresOS-SystemGate: ok=true` |



Milestone phases: `scripts/gate/system.py --phase N` or `--gate <name>`.



## Phase 5 (preemption)



Not part of either gate serial surface. Validated via:



```bash

python scripts/preemption/soak.py --boot-wait 90 --duration 30

python scripts/preemption/latency.py --boot-wait 90 --duration 30

```



Or `scripts/validation_matrix.py` (includes soak + latency checks).



## Phase checklists (historical)



Per-phase checklists under `docs/phase-*-checklist.md` record **implementation scope** for completed work. They are not the runtime validation surface. Use this document and `scripts/gate/` for CI and QEMU smokes.



Phases 1–4 fold into boot gate indirectly once phase 6+ subsystems run.



## CI matrix entries



| Check | Script |

|-------|--------|

| `gate-host-check` | `scripts/gate/host.py` |

| `boot-gate-host-check` | `scripts/gate/boot_host.py` |

| `boot-gate-check` | `scripts/gate/boot.py --gate boot` |

| `system-gate-host-check` | `scripts/gate/system_host.py` |

| `system-gate-check` | `scripts/gate/system.py --gate system` |

| `phase401-ares-rt-check` | `scripts/gate/ares_rt.py` |

| `phase5-soak-check` | `scripts/preemption/soak.py` |

| `phase5-latency-check` | `scripts/preemption/latency.py` |



See also [`RELEASE_SCORECARD_M500.md`](RELEASE_SCORECARD_M500.md).

