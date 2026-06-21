# Clan OS Gate Audit

```yaml
status: authoritative
validation_gate_version: "2.1.0"
kernel_module: kernel/src/validation_gate.rs
```

Machine-verifiable inventory of serial-emitted validation gates. Regenerate checks:

```bash
rg 'pub fn smoke_' kernel/src/
python scripts/gate/gate_honesty_check.py
python scripts/gate/run.py --gate all --timeout 360
```

## Classification legend

| Class | Meaning |
|-------|---------|
| **Real** | Exercises kernel behavior; checks counters/state from operations |
| **Partial** | Runner accumulates leaf smokes via `smoke_ok &=` (fixed v2.1.0) |
| **Shallow** | Real code path with simulated/stub backend (in-kernel broker, loopback) |
| **Hardcoded** | Fixed tag/corpus/digest verified against itself |
| **Const** | Compile-time constants + light runtime checks |
| **Counter** | Pass certified by atomic increment |
| **Circular** | Pass certified inside same gate call chain |
| **Stub** | Unconditional `true` (none in v2.1.0 serial path) |

CI enforcement: `scripts/gate/gate_honesty_check.py` (parts A: trivial stubs, B: `smoke_ok` shadowing), `scripts/gate/test_gate_honesty_check.py` (negative fixtures), `scripts/gate/module_wiring_check.py` (orphan `.rs` inventory; path-keyed allow-list), `scripts/gate/test_module_wiring_check.py` (inventory count + docs parity).

## Serial-emitted gates (`run_validation_gate`)

| Gate | Class | Notes |
|------|-------|-------|
| `shell_storage` | Real | CLANFS mount, README, echo, persistence |
| `loader_security` | Real | Discovery, policy, creds, ELF inventory |
| `memory_layout` | Real | Frame registry, backing, HW page tables |
| `userspace_bootstrap` | Real | User context, ring-3 trampoline, minimal ELF |
| `hw_paging` | Partial | Leaf smokes accumulated (v2.1.0) |
| `sched_userspace` | Partial | Integration + `restore2` |
| `dynamic_runtime` | Partial | Integration smoke |
| `fd_mmap` | Partial | Integration smoke |
| `vm_fork` | Partial | Integration smoke |
| `syscall_ring3` | Partial | Integration smoke |
| `path_exec` | Partial | Integration smoke |
| `smp_depth` | Partial | Integration smoke |
| `constitutional` | Const + shallow | `CONSTITUTIONAL_*` + syscall allowlist length |
| `capabilities` | Real | Cap lifecycle, rights, grants, brokers |
| `service_loader` | Real | Bootstrap caps, quota/budget rejection |
| `platform_brokers` | Mixed | Cap minting real; clipboard/network brokers shallow |
| `build_endpoints` | Mixed | IPC FIFO burst real; digest smokes hardcoded |
| `virtio_blk` | Real | Sector R/W on active backend |
| `network_compat` | Shallow | Loopback ping + compat socket stubs |
| `scheduler_epoch` | Real + shallow | Scheduler + compositor + OOM |
| `boundary` | Shallow | `LAYER_*` consts + boot_verified + counters |
| `integrity` | Mixed | OOM/audit real; repro build hardcoded |
| `scheduling` | Real (chained) | `integrity_gate` + epoch8 scheduler |
| `hardware` | Counter + shallow | Readiness atomic; virtio probe counts |
| `federation` | Shallow | Token forward + driver chain |
| `release` | Circular + shallow | `mark_release_scorecard` self-check |
| `desktop_preview` | Real | Mode 13h compositor frame |
| `desktop` | Real | Mouse, compositor, shell, font |
| `compat_runtime` | Real | `demo-hello` userland run |
| `compat_fd_vm` | Real | FD open/io + anon mmap |
| `compat_signal` | Real | `signal::smoke_signal_register/delivery` (wired v2.1.0) |
| `storage_depth` | Shallow | Persistence + mount subset |
| `posix_compat` | Real | `posix_server::smoke_posix_server` (wired v2.1.0) |
| `functional` | Composite | Desktop + loopback network + compat subsystems |
| `ci` | Stub + composite | `validation_matrix_smoke` = counter + `true` |
| `production` | Mixed | AP scheduler real; signed ELF hardcoded corpus |
| `network` | Shallow | Loopback `external-probe` simulation |
| `ClanOS-Gate: ok` | Composite | `boot_ok && system_gate()` |

## Remaining honesty gaps (documented, not fixed here)

- `validation_matrix_smoke()` and `release_scorecard_ok()` are circular/counter-only.
- `ci` gate does not execute host `validation_matrix.py` from kernel.
- External network and hardware path gates use loopback/digest stubs.
- Brokers run in-kernel; workspace `servers/` restructure deferred.

## Roadmap language

| Term | Meaning in this repo |
|------|---------------------|
| **Functional OS (scope 400)** | Desktop + userland + loopback network; QEMU `functional` gate |
| **Fully operational OS (scopes 401–500)** | Roadmap target: production SMP, signed ELF depth, external network, hardware path — see [ROADMAP_401_500.md](ROADMAP_401_500.md) |

`STATUS.md` uses **Functional OS** for current runtime posture; reserve **fully operational** for roadmap/scorecard epoch completion.

## v2.1.0 remediation (this epoch)

1. Wired `posix_server` and `signal` modules; replaced `smoke_posix_compat` / `smoke_compat_signal` stubs.
2. Fixed `smoke_ok` shadowing in eight `run_*_smokes` runners.
3. Fixed `COMPAT_SUBSYSTEMS_OK` cache: set after individual compat emits succeed (avoids non-idempotent double-run in `functional_gate`).
4. Fixed `posix_server` smoke: cap slot 0 and FD 0 are valid; compat-mode cap mint for native server bootstrap.

## Dead source inventory

Host check: `python scripts/gate/module_wiring_check.py` (also `module-wiring-check` in `validation_matrix.py`).

`.rs` files on disk not reachable from `lib.rs` `pub mod` tree are never compiled. After v2.1.0 wiring, `posix_server` and `signal` are no longer in this set. Remaining known-dead sources:

| File | Status | Notes |
|------|--------|-------|
| `kernel/src/cow_fork.rs` | Superseded | Live CoW in `user_paging.rs` / `task/process.rs`; `scripts/gate/cow_fork.py` expects `demand_paging` wiring that does not exist |
| `kernel/src/buddy.rs` | Unwired | `smoke_buddy_allocator` never referenced from `validation_gate.rs` |
| `kernel/src/block_cache.rs` | Unwired | `smoke_block_cache` never referenced from `validation_gate.rs` |

New orphan `.rs` files outside this inventory fail CI until wired or added to the documented inventory. Allow-list entries are **full paths** under `kernel/src/` only (`scripts/gate/module_wiring_check.py`); `EXPECTED_KNOWN_DEAD_COUNT = 3` is self-tested — a fourth entry requires updating this table and the test.

## Compat wiring blast radius (v2.1.0)

**Point-in-time claim (not a permanent guarantee):** as of v2.1.0, static analysis shows no syscall or userland callers of `signal` or `posix_server` outside the validation-gate smoke path. This can change when ring-3 dispatch or userland brokers wire `kill_syscall`, `invoke_compat`, or equivalent — re-audit at that scope.

Before PR2, `signal` and `posix_server` were not in the binary — zero production behavior. After wiring, the only kernel callers are `smoke_compat_signal()` and `smoke_posix_compat()` in `validation_gate.rs`. `init_process(victim)` runs only inside signal smokes; cap-slot-0 and FD-0 fixes apply only on the posix smoke path. `user_syscall_hw.rs` does not currently dispatch `kill_syscall` or `invoke_compat` to ring-3.
