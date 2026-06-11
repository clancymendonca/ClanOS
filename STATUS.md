# AresOS Project Status

## Snapshot (post-150 roadmap through phase 350)

- **Phases 111-130:** platform brokers + interim IPC (epoch 1, commit `044d4ef`)
- **Phase 201:** virtio-blk hybrid stub (epoch 2)
- **Phases 131-140:** build integrity, native endpoints, audit wire, IPC integration (epoch 3)
- **Phase 404:** virtio-net, compat sockets, functional network broker (epoch 4)
- **Phase 149:** service scheduler, SMP readiness, compositor, OOM policy (epoch 5)
- **Phase 150:** four-layer boundary review (epoch 6)
- **Phases 151-350:** [`ROADMAP_151_350.md`](docs/ROADMAP_151_350.md); `COMPLETED_PHASE=350`; epochs 7-14 graduated
- **Userland:** `ares-rt` host-target demo + `install_userland.py`
- **Epoch 0 evidence tier:** `proof-rights` proptest + Kani harnesses; `kani_gate.py` in covenant CI
- gap_registry: 0 open, 350 addressed, 0 wontfix (350 total)
- threat nodes open: 0
- kani_harness_count: 3
- phase_checklists: 200 implemented (151-350)
- release_scorecard: [`RELEASE_SCORECARD_M350.md`](docs/RELEASE_SCORECARD_M350.md)
- ipc_bridge_compat_internal: 0 (retired phase 134)

## Threat coverage by goal

- `privilege_escalation`: 6/8 closed
- `information_disclosure`: 1/2 closed
- `denial_of_service`: 3/3 closed
- `integrity_violation`: 7/9 closed

## Integration milestones

| Milestone | Serial line | Script |
|-----------|-------------|--------|
| Epoch 7 | `Phase175-Epoch7` | `phase175_epoch7_check.py` |
| M200 | `Phase200-Milestone` | `phase200_milestone_check.py` |
| M250 | `Phase250-Milestone` | `phase250_milestone_check.py` |
| M300 | `Phase300-Milestone` | `phase300_milestone_check.py` |
| M350 | `Phase350-Milestone` | `phase350_milestone_check.py` |

## Boot smokes (QEMU)

Expected serial lines (all `ok=true`):

- `Phase201-VirtioBlk`
- `Phase140-IPC` (`bridge=0`)
- `Phase404-Network`
- `Phase149-Epoch5`
- `Phase150-Milestone`
- `Phase175-Epoch7` through `Phase350-Milestone`

Scripts: `phase201_virtio_blk_check.py`, `phase134_endpoint_check.py`, `phase404_network_check.py`, `phase149_epoch5_check.py`, `phase150_milestone_check.py`, `phase175_epoch7_check.py` … `phase350_milestone_check.py`