# AresOS Project Status

## Snapshot (functional OS — milestone 400)

- **Phases 111-130:** platform brokers + interim IPC (epoch 1)
- **Phases 131-140:** build integrity, native endpoints, audit wire, IPC integration (epoch 3)
- **Phases 151-350:** [`ROADMAP_151_350.md`](docs/ROADMAP_151_350.md); epochs 7-14 graduated
- **Phases 351-400:** [`ROADMAP_351_400.md`](docs/ROADMAP_351_400.md); desktop + userland + network
- **COMPLETED_PHASE:** 400
- **Desktop:** VGA 320×200, double-buffered compositor, PS/2 mouse, window manager, taskbar shell
- **Userland:** `/bin/demo-hello`, `/bin/ares-info` native packages (ares-rt ABI)
- **Network:** virtio-net loopback ping + compat sockets
- **Userland runtime:** `ares-rt` + `install_userland.py`
- gap_registry: 0 open, 350 addressed (350 total)
- threat nodes open: 0
- phase_checklists: 250 implemented (151-400)
- release_scorecard: [`RELEASE_SCORECARD_M400.md`](docs/RELEASE_SCORECARD_M400.md)

## Threat coverage by goal

- `privilege_escalation`: 6/8 closed
- `information_disclosure`: 1/2 closed
- `denial_of_service`: 3/3 closed
- `integrity_violation`: 7/9 closed

## Integration milestones

| Milestone | Serial line | Script |
|-----------|-------------|--------|
| M350 | `Phase350-Milestone` | `phase350_milestone_check.py` |
| M375 | `Phase375-Milestone` | `phase375_milestone_check.py` |
| **M400** | `Phase400-Milestone` | `phase400_milestone_check.py` |
| Desktop | `Phase351-Desktop` | `phase351_desktop_check.py` |

## Boot smokes (QEMU)

Expected serial lines (all `ok=true`):

- `Phase350-Milestone`
- `Phase351-Desktop`
- `Phase375-Milestone`
- `Phase400-Milestone`

## Running with GUI

```powershell
.\scripts\run_desktop.ps1
```

Or manually:

```powershell
$env:Path = "C:\Program Files\qemu;" + $env:Path
cargo bootimage -p kernel
qemu-system-x86_64 -drive format=raw,file=target\x86_64-unknown-none\debug\bootimage-kernel.bin -serial stdio -display default -vga std -no-reboot
```

- **Terminal** (this window): type shell commands after `AresOS shell ready`
- **QEMU window**: 320×200 desktop (auto-refreshes; click to focus windows)

Shell commands: `help`, `run demo-hello`, `run ares-info`, `ls`, `ps`, `fsinfo`, `desktop`
