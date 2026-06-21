# Clan OS Project Status

## CI and local verification

**GitHub Actions is not currently substantiating pushes.** Run [27895509720](https://github.com/clancymendonca/ClanOS/actions/runs/27895509720) (tip `e4939f4`) failed before any job steps: account billing lock — workflow did not execute (`cargo fmt`, `validation_matrix.py`, QEMU). Re-run CI after billing is restored; do not treat a green local matrix as CI proof until then.

**Last verified locally (2026-06-21):** seed migration **complete** — 16/16 signed; allowlist empty; gate `2.6.0`.

## Snapshot (Functional OS — scope 400, QEMU gate v2.6.0)

- **Validation gate:** `kernel/src/validation_gate.rs` (`VALIDATION_GATE_VERSION = 2.6.0`)
- **ADR-0002 signed ELF (epoch 450):** kernel verifier + in-QEMU negative gauntlet (`signed_elf_integration`; 9 cases)
- **ADR-0003 loader signed exec (epoch 460):** seed migration **8 signed / 8 digest-only remaining** of 16; execution-path verify audit in ADR-0003 § Q4 (all `trust=system-signed` entry points); `/ext2/` uses `vfs::read_bytes` + same SHA256(ELF) digest
- **Gate audit:** [`docs/GATE_AUDIT.md`](docs/GATE_AUDIT.md) — per-gate substance classification
- **Gap audit:** [`docs/GAP_AUDIT.md`](docs/GAP_AUDIT.md) — `addressed` ≠ Implemented (204 overclaimed baseline)
- **Desktop:** VGA 320×200, double-buffered compositor, PS/2 mouse, window manager, taskbar shell
- **Userland:** `/bin/demo-hello`, `/bin/clan-info`, `/bin/mendo`, `/bin/ring3-io-demo`, `/bin/ring3-io-demo-ext2`, `/bin/hello-alloc` (Clan OS runtime: `clan-rt` with optional `ring3-heap` bump allocator)
- **Network:** virtio-net loopback + external route simulation
- gap_registry: 0 open, 350 addressed — see [`docs/GAP_AUDIT.md`](docs/GAP_AUDIT.md) (58% overclaimed baseline; audit OK = baseline held, not fully substantiated)
- threat nodes open: 0
- release_scorecard: [`RELEASE_SCORECARD.md`](docs/RELEASE_SCORECARD.md)
- **ADR-0003 (epoch 460):** verification epoch **done** (PR1 host, PR2 kernel, anchor guard). **Next:** seed `/bin/*` migration — one binary per PR, allowlist as rollback staging (**digest-only remaining 16 → 0** by scope 465); see ADR-0003 § Seed migration workflow
- Track 1 doc migration: **landed** (`8579e17`)
- **Q3 sunset (locked):** `sunset_scope=465`, `implementation_scope=460`; CI fails if `current_scope>=465` with non-empty allowlist

## Validation gates

Unified validation emits one serial family at boot:

| Line | Module |
|------|--------|
| `ClanOS-Gate: name=<subsystem> ok=true` | `validation_gate.rs` |
| `ClanOS-Gate: ok=true` | summary |

Host checks (no QEMU):

```
python scripts/gate/host.py
python scripts/gate/compat_subsystems.py
```

QEMU checks:

```
python scripts/gate/run.py --gate all --timeout 360
```

Use `--gate <subsystem>` for individual smokes (see `scripts/gate/map.py` and [`VALIDATION_GATES.md`](docs/VALIDATION_GATES.md)).

### Subsystem gates

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
| boundary | Four-layer boundary review |
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
