# Gate Audit — Post-400 (Scopes 401–500)

```yaml
status: authoritative
validation_gate_version: "2.6.0"
roadmap: docs/ROADMAP_401_500.md
companion: docs/GATE_AUDIT.md
```

Read-only classification of gates that support **fully operational OS** claims. Does not implement new backends — see [`GATE_DESIGN_401_500.md`](GATE_DESIGN_401_500.md) for Phase 3 design backlog.

Legend matches [`GATE_AUDIT.md`](GATE_AUDIT.md): Real / Partial / Shallow / Hardcoded / Const / Counter / Circular / Stub.

## Post-400 serial gates

| Gate | Class | Current proof | Roadmap claims | Aspirational gap |
|------|-------|---------------|----------------|------------------|
| `ci` | Stub + composite | `validation_matrix_smoke()` → unconditional `true`; chains `functional_gate()` | Epoch 425: validation matrix wired | Host matrix not invoked from kernel; stub must be replaced or removed |
| `production` | Mixed | `ci_gate` + `smp::smoke_ap_scheduler()` + `build_integrity::smoke_signed_user_elf()` | Epoch 450: production SMP + signed pinned gate corpus | AP smoke is tick/enqueue counter; signed ELF is **Real for pinned corpus** (Ed25519 vs epoch-450 anchor) — not general `/bin/*` loader signing |
| `network` | Shallow | `network_stack::smoke_external_network()` → loopback `external-probe` | Epoch 475: `has_external_network` | No real NIC TX/RX; flag stays `false` until scope-475 gate passes |
| `hardware` | Counter + shallow | `HARDWARE_PATH_READY` increment + `build_integrity::boot_verified()` / digest stub | Epoch 500: hardware bring-up | No bare-metal procedure exercised in QEMU gate |
| `release` / `system_gate` | Composite + shallow | `network_gate` + `hardware_path_smoke` + `release_compat_smoke` | Full `ClanOS-Gate: ok=true` | Chains shallow network/hardware smokes |

## ROADMAP falsifier mapping

| Falsifier ([ROADMAP_401_500.md](ROADMAP_401_500.md)) | Audit status | Notes |
|------------------------------------------------------|--------------|-------|
| Functional OS regression | **Audited** | `functional_gate()` — see GATE_AUDIT.md |
| Production SMP | **Gap** | `smoke_ap_scheduler` Real-ish counter; not multi-AP load/fairness |
| Signed ELF (gate corpus) | **Real (pinned corpus)** | Kernel `signed_elf.rs` Ed25519 verify vs epoch-450 anchor; **execution proof:** `signed-elf-kernel-integration` (9 QEMU cases). |
| Loader signed exec (seed `/bin/*`) | **Real (digest+sig)** | 16/16 seed manifests `trust=system-signed`; verify on all enumerated exec paths (`test_loader_signed_exec_path_audit.py`); sunset closed scope 465 (`loader_digest_only_grace=false`, empty allowlist). **Exempt:** `/bin/hello` remains `trust=user` (ADR-0002 gate-corpus fixture, not a system seed) — see § Scope honesty below. |
| External network | **Gap** | Loopback simulation; flag `false` until Real gate (fixed 2026-06-20) |
| Release gate | **Partial** | Serial `ok=true` composes gaps above |

## Architecture flags vs gate proof

| Flag (`architecture_state.toml`) | Value | Gate alignment |
|-----------------------------------|-------|----------------|
| `has_no_std_enforcement` | `true` | Host `scripts/gate/clan_rt.py` — aligned |
| `has_external_network` | `false` | Aligned — flip to `true` only when scope-475 external NIC gate passes (`architecture_state_check.py`) |
| `loader_digest_only_grace` | `false` | Aligned — scope **465** closed; empty `loader_digest_only_allowlist.toml`; seed migration complete |
| `has_real_hardware_target` | `false` | Aligned with shallow hardware smoke |

## Scope honesty — loader signing inventory

| Inventory | Signed verify at exec | Notes |
|-----------|----------------------|-------|
| 16 seed `/bin/*` programs (ADR-0003 migration) | **Yes** — `trust=system-signed`, epoch-460 anchor | Allowlist empty; grace `false` |
| Pinned loader gate corpus | **Yes** — `loader_signed_exec_integration` | Synthetic fixtures only |
| `/bin/hello` (`trust=user`) | **No** — intentional exempt | User-trust HW ELF validation fixture; not a system seed. Uses name allowlist + `execute_allowlisted_user_elf`; **not** in scope of ADR-0003 seed migration. Revisit only if `hello` is promoted to `trust=system-signed` (would require ADR amendment + `execute_minimal_user_elf_descriptor` row in Q4 table). |
| ADR-0002 gate corpus ELFs | **Yes** — separate `signed_elf.rs` / epoch-450 anchor | Distinct from loader exec manifests |

## No gate yet (design backlog)

These require **new gate design** against real backends, not wiring existing orphans:

1. ~~**Signed ELF (gate corpus)** — ADR-0002~~ **Done**
2. ~~**Loader signed exec (kernel verifier + seed migration)** — ADR-0003~~ **Done** (scope 465 closed)
3. **External network** — virtio (or NIC) TX/RX to non-loopback peer or test harness
4. **Production SMP** — AP scheduler under load (fairness/latency thresholds; see preemption soak patterns)
5. **CI gate** — kernel `ci_gate` invokes host `validation_matrix.py` subset or drops stub

See [`GATE_DESIGN_401_500.md`](GATE_DESIGN_401_500.md) for proposed serial semantics and ADR links. Priority implementation: [`architecture/ADR/ADR-0002-signed-elf-production-gate.md`](architecture/ADR/ADR-0002-signed-elf-production-gate.md).

## Distinction from v2.1.0 remediation

| Epoch | Question |
|-------|----------|
| v2.1.0 gate honesty | Do existing serial lines exercise wired code? |
| 401–500 audit (this doc) | Do post-400 lines prove what the roadmap/scorecard claims? |
| 401–500 design (next) | Build the backend the claim describes |

No `VALIDATION_GATE_VERSION` bump for this document.
