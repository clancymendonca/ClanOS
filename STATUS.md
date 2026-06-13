# AresOS Project Status

## Snapshot (fully operational OS)

- **Boot gate:** phases 6–150 unified in `kernel/src/boot_gate.rs` (`BOOT_GATE_VERSION = 1.0.0`)
- **System gate:** post-150 integration in `kernel/src/system_gate.rs` (`SYSTEM_GATE_VERSION = 1.0.0`)
- **Desktop:** VGA 320×200, double-buffered compositor, PS/2 mouse, window manager, taskbar shell
- **Userland:** `/bin/demo-hello`, `/bin/ares-info` native packages (`ares-rt` `#![no_std]`)
- **Network:** virtio-net loopback + external route simulation
- gap_registry: 0 open, 350 addressed (350 total)
- threat nodes open: 0
- release_scorecard: [`RELEASE_SCORECARD_M500.md`](docs/RELEASE_SCORECARD_M500.md)
- Track 1 doc migration: **gated** (see `config/track1_scope_freeze.toml`)

## Validation gates

Boot and system validation emit two serial families at boot:

| Family | Final line | Module |
|--------|------------|--------|
| Boot (6–150) | `AresOS-BootGate: ok=true` | `boot_gate.rs` |
| System (151–500) | `AresOS-SystemGate: ok=true` | `system_gate.rs` |

Host checks (no QEMU):

```
python scripts/gate/host.py
```

QEMU checks:

```
python scripts/gate/boot.py --gate boot --timeout 360
python scripts/gate/system.py --gate system --timeout 360
```

Legacy phase numbers: `python scripts/gate/legacy.py --phase N`. Thin shims at `scripts/gate/boot.py` etc. remain for older references.

### Boot gate subsystems

| Gate | Covers |
|------|--------|
| shell_storage | Phases 6–8 |
| loader_security | Phases 9–13 |
| memory_layout | Phases 14–16 |
| userspace_bootstrap | Phases 17–20 |
| hw_paging | Phases 21–30 |
| sched_userspace | Phases 31–40 |
| dynamic_runtime | Phases 41–50 |
| fd_mmap | Phases 51–60 |
| vm_fork | Phases 61–70 |
| syscall_ring3 | Phases 71–80 |
| path_exec | Phases 81–90 |
| smp_depth | Phases 91–100 |
| constitutional | Phase 110 |
| capabilities | Phase 120 |
| service_loader | Phase 121 |
| platform_brokers | Phase 130 |
| build_endpoints | Phases 131–140 |
| virtio_blk | Phase 201 |
| network_compat | Phase 404 |
| scheduler_epoch | Phase 149 |
| boundary | Phase 150 |

## Running with GUI

```powershell
.\scripts\run_desktop.ps1
```

Shell commands: `help`, `run demo-hello`, `run ares-info`, `ls`, `ps`, `fsinfo`, `desktop`
