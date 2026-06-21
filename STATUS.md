# Clan OS Project Status

## CI and local verification

**GitHub Actions is not currently substantiating pushes.** Run [27895509720](https://github.com/clancymendonca/ClanOS/actions/runs/27895509720) (tip `e4939f4`) failed before any job steps: account billing lock — workflow did not execute (`cargo fmt`, `validation_matrix.py`, QEMU). Re-run CI after billing is restored; do not treat a green local matrix as CI proof until then.

**Last verified locally at `e4939f4` (2026-06-21):** per-commit `scripts/verify_commit_clean.py` on all four pushed SHAs; tip additionally host `test_signed_elf.py` and QEMU `signed_elf_integration` (9/9) with `qemu-system-x86_64` on PATH.

## Working tree triage (~152 paths, not on `origin/main`)

Doc-migration catch-up from unified-gate epoch — **no hidden gate/kernel engineering** in this pile (that shipped in `ed56489`…`e4939f4`).

| Bucket | Paths | Action |
|--------|-------|--------|
| **Commit** (Track 1 batch) | ~141 — `docs/scope-*-checklist.md` (117), README, ROADMAP/INDEX, architecture redirects, RELEASE_SCORECARD_M*, generator scripts | Single `fixup(docs): Track 1 gate terminology migration` when doc epoch opens; aligns stale `boot.py` / `BootGate` refs with committed `validation_gate.rs` |
| **Investigate before that commit** | 3 — `config/track1_scope_freeze.toml` (`clan_rt_no_std` → resolved), `DECISION_LOG.md` (M400 wording), scorecard superseded-by headers | Wording looks correct vs `origin/main` (`has_no_std_enforcement = true`, `RELEASE_SCORECARD.md` exists); confirm in batch commit message |
| **Optional / local** | `.cursor/rules/clanos-principal-engineer.mdc`, `scripts/run_desktop.cmd` (untracked) | Agent guidance + desktop wrapper; commit with doc batch or leave local |
| **Discard** | 0 | No superseded scratch or duplicate engineering found |

## Snapshot (Functional OS — scope 400, QEMU gate v2.2.0)

- **Validation gate:** `kernel/src/validation_gate.rs` (`VALIDATION_GATE_VERSION = 2.2.0`)
- **ADR-0002 signed ELF (epoch 450):** kernel verifier + in-QEMU negative gauntlet (`cargo test -p kernel --test signed_elf_integration` via `validation_matrix.py`; requires `qemu-system-x86_64` — compile-only is not substantiation)
- **Gate audit:** [`docs/GATE_AUDIT.md`](docs/GATE_AUDIT.md) — per-gate substance classification
- **Gap audit:** [`docs/GAP_AUDIT.md`](docs/GAP_AUDIT.md) — `addressed` ≠ Implemented (204 overclaimed baseline)
- **Desktop:** VGA 320×200, double-buffered compositor, PS/2 mouse, window manager, taskbar shell
- **Userland:** `/bin/demo-hello`, `/bin/clan-info`, `/bin/mendo`, `/bin/ring3-io-demo`, `/bin/ring3-io-demo-ext2`, `/bin/hello-alloc` (Clan OS runtime: `clan-rt` with optional `ring3-heap` bump allocator)
- **Network:** virtio-net loopback + external route simulation
- gap_registry: 0 open, 350 addressed — see [`docs/GAP_AUDIT.md`](docs/GAP_AUDIT.md) (58% overclaimed baseline; audit OK = baseline held, not fully substantiated)
- threat nodes open: 0
- release_scorecard: [`RELEASE_SCORECARD.md`](docs/RELEASE_SCORECARD.md)
- Track 1 doc migration: **gated** — ~152 paths triaged above; batch commit pending (see `config/track1_scope_freeze.toml`)
- **Next epoch:** loader signing for `/bin/*` — [`ADR-0003`](docs/architecture/ADR/ADR-0003-loader-signed-exec-manifests.md) Q1–Q4 locked; Q5 golden bytes block kernel PR; implementation scope 460, allowlist sunset scope 465
- **Sequencing:** Track 1 doc batch (~141 paths) ready independent of CI billing; re-run Actions when billing restored — do not block doc migration on GitHub

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
