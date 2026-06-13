# Clan OS Project Status

## Snapshot (fully operational OS)

- **Boot gate:** `kernel/src/boot_gate.rs` (`BOOT_GATE_VERSION = 1.0.0`)
- **System gate:** `kernel/src/system_gate.rs` (`SYSTEM_GATE_VERSION = 1.0.0`)
- **Desktop:** VGA 320×200, double-buffered compositor, PS/2 mouse, window manager, taskbar shell
- **Userland:** `/bin/demo-hello`, `/bin/clan-info`, `/bin/mendo`, `/bin/ring3-io-demo`, `/bin/ring3-io-demo-ext2`, `/bin/hello-alloc` (Clan OS runtime: `clan-rt` with optional `ring3-heap` bump allocator)
- **Network:** virtio-net loopback + external route simulation
- gap_registry: 0 open, 350 addressed (350 total)
- threat nodes open: 0
- release_scorecard: [`RELEASE_SCORECARD_M500.md`](docs/RELEASE_SCORECARD_M500.md)
- Track 1 doc migration: **gated** (see `config/track1_scope_freeze.toml`)

## Validation gates

Boot and system validation emit two serial families at boot:

| Family | Final line | Module |
|--------|------------|--------|
| Boot | `ClanOS-BootGate: ok=true` | `boot_gate.rs` |
| System | `ClanOS-SystemGate: ok=true` | `system_gate.rs` |

Host checks (no QEMU):

```
python scripts/gate/host.py
python scripts/gate/compat_subsystems.py
```

QEMU checks:

```
python scripts/gate/boot.py --gate boot --timeout 360
python scripts/gate/system.py --gate system --timeout 360
```

Use `--gate <subsystem>` for individual boot/system subsystem smokes (see `scripts/gate/map.py`).

### Boot gate subsystems

| Gate | Role |
|------|------|
| shell_storage | CLANFS mount, seed corpus, shell dispatch |
| loader_security | Program discovery, credentials, ELF inventory |
| memory_layout | Frame ownership, backing, page tables |
| userspace_bootstrap | User context, ring-3 entry, syscalls |
| hw_paging | HW page tables, CR3, iretq, HW syscalls |
| sched_userspace | Scheduler CR3, user frames, demand paging |
| dynamic_runtime | Shared libs, FD/path, SMP probes |
| fd_mmap | FD table, mmap, mprotect, runqueue |
| vm_fork | VMA, fork-lite, ring-3 syscalls |
| syscall_ring3 | sysret, wait-lite, fcntl, IPI |
| path_exec | HW sysret, getcwd, pipes, exec-lite |
| smp_depth | CoW fork, poll, work-steal, LAPIC |
| constitutional | Governance ABI / semantics |
| capabilities | Capability compat table |
| service_loader | Service loader bootstrap |
| platform_brokers | Storage/permission/device/network brokers |
| build_endpoints | Build integrity, IPC, audit wire |
| virtio_blk | Virtio block probe |
| network_compat | Virtio net + compat socket epoch |
| scheduler_epoch | Service scheduler integration |
| boundary | Milestone boundary smoke |

### System gate subsystems

| Gate | Role |
|------|------|
| integrity | Build integrity, audit, OOM, loom |
| scheduling | Unified service scheduling |
| hardware | Virtio block/net, SDK path |
| federation | Federation + observability |
| release | Checkpoint, scorecard, boot verify |
| desktop_preview / desktop | Compositor, shell, mouse |
| compat_runtime | Userland demo + packages |
| compat_fd_vm | FD, mmap, CoW |
| compat_signal | Signal register + delivery |
| storage_depth | Buddy, cache, VFS, ext2 |
| posix_compat | POSIX server skeleton |
| functional | Composite (includes compat) |
| ci / production / network | Release hardening gates |

## Running with GUI

```powershell
.\scripts\run_desktop.ps1
```

Shell commands: `help`, `run demo-hello`, `run clan-info`, `run mendo`, `run ring3-io-demo`, `run ring3-io-demo-ext2`, `run hello-alloc`, `fork-run mendo`, `fork-run ring3-io-demo`, `fork-run hello-alloc`, `cat /ext2/smoke.txt`, `ls`, `ps`, `fsinfo`, `desktop`
