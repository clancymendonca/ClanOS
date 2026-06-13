# Post-400 Roadmap (Scopes 401–500)

```yaml
status: authoritative
semantics_version: 1.0.0
```

Continues from [ROADMAP_351_400.md](ROADMAP_351_400.md) milestone 400. Goal: **fully operational OS** — production SMP, signed userland, external network, hardware bring-up path, and M500 release gate.

**Pace:** 1–3 scopes/month.

---

## Epoch map

| Epoch | Scopes | Milestone | Theme |
|-------|--------|-----------|-------|
| 17 | 401–425 | **425** | CI hardening, validation matrix M400+, `clan-rt` no_std |
| 18 | 426–450 | **450** | Production SMP AP scheduler, signed arbitrary ELF corpus |
| 19 | 451–475 | **475** | External network depth (`has_external_network`) |
| 20 | 476–500 | **500** | QEMU→hardware procedure, M500 release scorecard |

---

## Architecture state flags

| Flag | Milestone | Trigger |
|------|-----------|---------|
| `has_no_std_enforcement` | 401 | `clan-rt` `#![no_std]` + host `cargo check` |
| `has_external_network` | 475 | External route smoke + threat re-eval |
| `has_real_hardware_target` | 500+ | Bare-metal procedure documented in [ARCHITECTURE_TARGETS.md](architecture/ARCHITECTURE_TARGETS.md) |

---

## Boot smokes (post-400)

| Line | Milestone | Script |
|------|-----------|--------|
| `ClanOS-Gate: name=ci ok=true` | 425 | `python scripts/gate/system.py --gate ci --timeout 180` |
| `ClanOS-Gate: name=production ok=true` | 450 | `python scripts/gate/system.py --gate production --timeout 180` |
| `ClanOS-Gate: name=network ok=true` | 475 | `python scripts/gate/system.py --gate network --timeout 180` |
| `ClanOS-SystemGate: ok=true` | 500 | `python scripts/gate/system.py --gate system --timeout 180` |

Prior M400 lines remain regression gates: `ClanOS-Gate: name=desktop_preview ok=true`, `ClanOS-Gate: name=desktop ok=true`, `ClanOS-Gate: name=functional ok=true`.

---

## Milestone 500 falsifiers

| Criterion | Falsifier |
|-----------|-----------|
| M400 regression | `ClanOS-Gate: name=functional ok=true` smoke false |
| Production SMP | AP scheduler smoke false |
| Signed ELF | `smoke_signed_user_elf` false |
| External network | `ClanOS-Gate: name=network ok=true` smoke false |
| Release gate | `ClanOS-SystemGate: ok=true` smoke false |

See [RELEASE_SCORECARD_M500.md](RELEASE_SCORECARD_M500.md).
