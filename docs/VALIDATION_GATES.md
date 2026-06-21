# Clan OS Validation Gates

```yaml
status: authoritative
validation_gate_version: "2.4.0"
kernel_module: kernel/src/validation_gate.rs
scripts_package: scripts/gate/
```

Runtime validation is **gate-based**. At boot the kernel runs all subsystem smokes once and emits unified serial lines:

```
ClanOS-Gate: name=<subsystem> ok=true|false
ClanOS-Gate: ok=true|false
```

During the legacy alias epoch, deprecated lines (`ClanOS-Gate:`, `ClanOS-Gate:`) are also emitted for backward compatibility.

## Quick validation

```bash
cargo check -p kernel
python scripts/gate/host.py
python scripts/gate/run.py --gate all --timeout 360
python scripts/validation_matrix.py --smoke-timeout 180
```

Use `--gate <subsystem>` for individual smokes. Historical scope indices map via `scripts/gate/map.py` (`gate_for_scope`).

Deprecated wrappers (`boot.py`, `system.py`) forward to `run.py` with stderr warnings.

## Subsystem gates

| Gate | Role | Serial line |
|------|------|-------------|
| `shell_storage` | CLANFS mount, seed corpus, shell dispatch | `ClanOS-Gate: name=shell_storage ok=true` |
| `loader_security` | Program discovery, credentials, ELF inventory, pinned loader signed corpus | `ClanOS-Gate: name=loader_security ok=true` |
| `memory_layout` | Frame ownership, backing, page tables | `ClanOS-Gate: name=memory_layout ok=true` |
| `userspace_bootstrap` | User context, ring-3 entry, syscalls | `ClanOS-Gate: name=userspace_bootstrap ok=true` |
| `hw_paging` | HW page tables, CR3, iretq, HW syscalls | `ClanOS-Gate: name=hw_paging ok=true` |
| `sched_userspace` | Scheduler CR3, user frames, demand paging | `ClanOS-Gate: name=sched_userspace ok=true` |
| `dynamic_runtime` | Shared libs, FD/path, SMP probes | `ClanOS-Gate: name=dynamic_runtime ok=true` |
| `fd_mmap` | FD table, mmap, mprotect, runqueue | `ClanOS-Gate: name=fd_mmap ok=true` |
| `vm_fork` | VMA, fork-lite, ring-3 syscalls | `ClanOS-Gate: name=vm_fork ok=true` |
| `syscall_ring3` | sysret, wait-lite, fcntl, IPI | `ClanOS-Gate: name=syscall_ring3 ok=true` |
| `path_exec` | HW sysret, getcwd, pipes, exec-lite | `ClanOS-Gate: name=path_exec ok=true` |
| `smp_depth` | CoW fork, poll, work-steal, LAPIC | `ClanOS-Gate: name=smp_depth ok=true` |
| `constitutional` | Governance ABI / semantics | `ClanOS-Gate: name=constitutional ok=true` |
| `capabilities` | Capability compat table | `ClanOS-Gate: name=capabilities ok=true` |
| `service_loader` | Service loader bootstrap | `ClanOS-Gate: name=service_loader ok=true` |
| `platform_brokers` | Storage/permission/device/network brokers | `ClanOS-Gate: name=platform_brokers ok=true` |
| `build_endpoints` | Build integrity, IPC, audit wire | `ClanOS-Gate: name=build_endpoints ok=true` |
| `virtio_blk` | Virtio block probe | `ClanOS-Gate: name=virtio_blk ok=true` |
| `network_compat` | Virtio net + compat socket epoch | `ClanOS-Gate: name=network_compat ok=true` |
| `scheduler_epoch` | Service scheduler integration | `ClanOS-Gate: name=scheduler_epoch ok=true` |
| `boundary` | Four-layer boundary review | `ClanOS-Gate: name=boundary ok=true` |
| `integrity` | Build integrity, audit, OOM, loom | `ClanOS-Gate: name=integrity ok=true` |
| `scheduling` | Unified service scheduling | `ClanOS-Gate: name=scheduling ok=true` |
| `hardware` | Virtio block/net, SDK path | `ClanOS-Gate: name=hardware ok=true` |
| `federation` | Federation + observability | `ClanOS-Gate: name=federation ok=true` |
| `release` | Checkpoint, scorecard, boot verify | `ClanOS-Gate: name=release ok=true` |
| `desktop_preview` | Compositor preview | `ClanOS-Gate: name=desktop_preview ok=true` |
| `desktop` | Full desktop stack | `ClanOS-Gate: name=desktop ok=true` |
| `compat_runtime` | Ring-3 clan-rt corpus | `ClanOS-Gate: name=compat_runtime ok=true` |
| `compat_fd_vm` | FD / mmap / CoW | `ClanOS-Gate: name=compat_fd_vm ok=true` |
| `compat_signal` | Signal skeleton + delivery | `ClanOS-Gate: name=compat_signal ok=true` |
| `storage_depth` | Buddy / VFS / ext2 | `ClanOS-Gate: name=storage_depth ok=true` |
| `posix_compat` | POSIX server skeleton | `ClanOS-Gate: name=posix_compat ok=true` |
| `functional` | Composite (includes compat) | `ClanOS-Gate: name=functional ok=true` |
| `ci` | Release CI hardening | `ClanOS-Gate: name=ci ok=true` |
| `production` | Production SMP + signed ELF | `ClanOS-Gate: name=production ok=true` |
| `network` | External network depth | `ClanOS-Gate: name=network ok=true` |
| **`all`** (summary) | all subsystems above | `ClanOS-Gate: ok=true` |

## Preemption validation

Not part of gate serial lines. Kernel emits:

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
| `gate-host-check` | `scripts/gate/gate_host.py` |
| `gate-check` | `scripts/gate/run.py --gate all` |
| `boot-gate-host-check` | alias → `gate-host-check` (deprecated name) |
| `boot-gate-check` | alias → `gate-check` (deprecated name) |
| `system-gate-host-check` | alias → `gate-host-check` (deprecated name) |
| `system-gate-check` | alias → `gate-check` (deprecated name) |
| `compat-subsystems-host-check` | `scripts/gate/compat_subsystems.py` |
| `preemption-soak-check` | `scripts/preemption/soak.py` |
| `preemption-latency-check` | `scripts/preemption/latency.py` |
| `gate-signed-elf-host-check` | `scripts/gate/signed_elf.py` |
| `gate-signed-elf-self-test` | `scripts/gate/test_signed_elf.py` |
| `trust-anchor-embed-match` | `scripts/gate/test_anchor_embed_match.py` — kernel `[u8; 32]` embed must match anchor TOML hex (catches transcription errors before QEMU) |
| `signed-elf-kernel-integration` | `cargo test -p kernel --test signed_elf_integration` in QEMU (**9 cases**). Run via `validation_matrix.py` (`ensure_qemu_on_path()`); bare `cargo test` fails if QEMU is not on PATH. Compile success alone is not a pass. |
| `gate-loader-signed-exec-host-check` | `scripts/gate/loader_signed_exec.py` |
| `gate-loader-signed-exec-self-test` | `scripts/gate/test_loader_signed_exec.py` |
| `loader-signing-sunset-check` | `scripts/gate/loader_signing_sunset_check.py` |
| `loader-signed-exec-kernel-integration` | `cargo test -p kernel --test loader_signed_exec_integration` in QEMU (**11 cases** incl. kind/entry tamper + ADR-0002 body distinctness). Matrix only — same QEMU PATH rule as signed-elf. |

See also [`RELEASE_SCORECARD.md`](RELEASE_SCORECARD.md).
