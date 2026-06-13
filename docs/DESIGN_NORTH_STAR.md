# Clan OS Design North Star

```yaml
status: superseded-by: docs/architecture/DESIGN_NORTH_STAR.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/architecture/DESIGN_NORTH_STAR.md`](architecture/DESIGN_NORTH_STAR.md). This flat copy retains epoch scorecard rows until migration squash reconciles content.

Falsifiable targets for scopes 121–150. See [`CHARTER.md`](../CHARTER.md), [`THREAT_MODEL.md`](THREAT_MODEL.md).

---

## Falsifiable scorecard

| Row | Claim | Falsifier |
|-----|-------|-----------|
| Capability security | No ambient authority in native path | Compat-internal counter > 0 at scope 134 gate |
| Evidence pyramid | Every closed critical threat node has tier A/B/C proof | `threat_node_proof_coverage_ratio` < 1.0 |
| Compat sunset | Fixed corpus % native-only E2E | COMPAT_SUNSET metric regression without review |
| Reproducible build | Dual-build hash match | BUILD_INTEGRITY CI failure |
| Public security process | Disclosure + key compromise playbooks | Missing before M150 release |
| Windows comparison | Named rows or explicit non-commitment | See § Comparative position |
| **M200** scheduling | `SCHEDULING_UNIFIED` S-* cases executable; semantic lint on ABI bumps | ClanOS-Gate: name=scheduling ok=true smoke false |
| **M250** hardware/SDK | QEMU→HW procedure + signed images; native SDK path | ClanOS-Gate: name=hardware ok=true smoke false |
| **M300** federation | Distributed endpoint protocol + observability tooling | ClanOS-Gate: name=federation ok=true smoke false |
| **M350** release 1.0 | Zero open threats; never-stabilize graduated; dual-build + compat threshold | ClanOS-Gate: name=release ok=true smoke false |

---

## Comparative position

**Linux:** capability-native IPC and generation-based revocation as structural differentiators.

**seL4:** formal proof depth; Clan OS targets tiered evidence (proptest + Kani + selective Verus) with Rust implementation velocity.

**Hyper-V / VM isolation:** not primary QEMU-era target; note in ARCHITECTURE_TARGETS.

**Guix/Nix reproducibility:** BUILD_INTEGRITY dual-build aspiration.

**Windows:** falsifiable comparison rows deferred to epoch 3 planning commit **or** explicit **non-commitment** statement — no implied parity claims until rows exist.

---

## Never stabilize before 1.0

These artifacts may change with semver but **cannot be declared stable** until 1.0 graduation:

- Interim IPC bridge (`compat-internal`)
- MEM_BUDGET_STUB / OOM shed stub wire formats
- Compat-internal channels
- Debug introspection TBD interfaces
- `health_timeseries.json` schema
- Benchmark baseline JSON schema
- `epoch_signoffs/` manifest schema
- `epoch_checklist.toml` schema
- `gap_registry.toml` schema
- `GLOSSARY.toml` schema
- `THREAT_NODES.toml` schema
- `kani_harness_registry` schema
- `architecture_state.toml` schema
- Test environment manifest

**Graduation:** full spec + zero compat callers + compat review. Review list each epoch gate; item on list **3 epochs** without plan → deferral decision recorded here.

**Cap kind semantics:** once a kind exits this list, semantics are **frozen**; reinterpretation requires a **new kind**.

**Emergency forks:** divergent ABI artifacts from charter emergency stabilization (`CHARTER.md`) listed here with semver.

---

## Semver vs never-stabilize

Protocol doc semver tracks all changes. Never-stabilize means **cannot declare stable** until 1.0 — accumulating breaking bumps does not imply approaching stability.

---

## Post-150 scope inventory

Flat list of deferred work. Reviewed at **milestone 150** gate: each item requires **named owner** and **rough timeline** (visibility, not scope commitment).

| Item | Trigger (`architecture_state.toml`) | Owner | Timeline |
|------|-----------------------------------|-------|----------|
| Checkpoint / restore security domain | `has_persisted_cap_state` | TBD | post-150 |
| Distributed cap revocation | N/A (structural) | TBD | post-150 |
| Physical attacker mitigations | `has_real_hardware_target` | TBD | epoch 6 |
| TPM / measured boot | `has_tpm_integration` | TBD | epoch 6 |
| Side-channel re-evaluation | `has_speculative_execution_unit` | TBD | post-150 |
| Hypervisor guest hardening | `has_hypervisor_guest_target` | TBD | post-150 |
| External network threat expansion | `has_external_network` | TBD | epoch 4+ |
| Formal Tier D model (`FORMAL_MODEL.md`) | framework decision in DECISION_LOG | TBD | post-150 |
| Accessibility platform obligation | M150 decision | TBD | post-150 or out-of-scope |
| Boot attestation chain | epoch 6 | TBD | post-150 |
| Cap table sharding SMP proof | epoch 5 note | TBD | post-150 |
| Full disclosure / key compromise playbooks | M150 release | TBD | M150 |
| Capability transfer walkthrough artifact | M150 | TBD | M150 |

---

## Internationalization

i18n for compositor and system UI: deferred post-150; ABI extension point reserved in ABI_COMPOSITOR_IPC planning.

---

## Accessibility

Platform a11y obligation: **deferred post-150** or explicit out-of-scope row at M150 gate.

---

## Health dashboard

Epoch 3: full health dashboard. Epoch 0 gate: static gap/doc visualization from `project_health.py`.
