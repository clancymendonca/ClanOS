# ADR-0001: Unified Validation Gate

```yaml
status: authoritative
adr_id: ADR-0001
decision_date: 2026-06-20
```

## Context

Clan OS runtime validation uses named subsystem gates (`shell_storage`, `functional`, `network`, …). Historically these were split across two kernel modules (`boot_gate.rs`, `system_gate.rs`), two serial families (`ClanOS-BootGate:`, `ClanOS-SystemGate:`), and milestone labels (M350, M400, M500). Both gate modules run sequentially at boot in `main.rs` — there is no separate runtime phase.

Milestone numbers and boot/system phase vocabulary duplicate the same subsystem model and force maintainers to choose "boot or system?" for every new smoke.

## Decision

**Option A — full unify:** merge `boot_gate.rs` and `system_gate.rs` into `validation_gate.rs` with:

- Single serial family: `ClanOS-Gate: name=<subsystem> ok=<bool>` and summary `ClanOS-Gate: ok=true`
- Single version constant: `VALIDATION_GATE_VERSION = "2.0.0"`
- One CI QEMU entrypoint: `scripts/gate/run.py --gate all`
- One-epoch legacy alias emission (`ClanOS-BootGate:` / `ClanOS-SystemGate:`) and Python wrapper shims for backward compatibility

## Alternatives considered

| Option | Outcome |
|--------|---------|
| B — Keep two phases, drop M-numbers only | Rejected: structural duplication and two QEMU boots remain |
| C — Docs-only cleanup | Rejected: dual runtime modules unchanged |

## Consequences

- Positive: one module, one version, one scorecard, faster CI (single QEMU boot for full gate)
- Positive: subsystem names unchanged — bisect via `--gate <name>` preserved
- Negative: breaking change to serial protocol (mitigated by legacy alias epoch)
- Negative: `scripts/gate/boot.py` / `system.py` deprecated then removed

**Revision (2026-06-20):** honesty remediation bumped `VALIDATION_GATE_VERSION` to `2.1.0` (wired compat smokes, `smoke_ok` accumulation, `gate_honesty_check.py`). ADR decision body above records the initial ADR-0001 merge at `2.0.0`.

## Security implications

No new attack surface. Smoke inventory and graduation checks unchanged; only packaging and serial formatting consolidate.

## Verification approach

- Host: `scripts/gate/gate_host.py` (cargo check + function inventory)
- QEMU: `scripts/gate/run.py --gate all`
- Matrix: `scripts/validation_matrix.py`
- Alias parsing: `scripts/gate/map.py` `LEGACY_PATTERNS`

Tier A: proptest on map.py gate routing (bounded). Tier B: Kani not required — no capability logic changed.
