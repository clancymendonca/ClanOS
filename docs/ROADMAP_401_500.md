# Post-400 Roadmap (Scopes 401–500)

```yaml
status: authoritative
semantics_version: 1.0.0
```

Continues from [ROADMAP_351_400.md](ROADMAP_351_400.md) (functional OS epoch). Goal: **fully operational OS** — production SMP, signed userland, external network, hardware bring-up path, and unified release gate.

**Pace:** 1–3 scopes/month.

---

## Epoch map

| Epoch | Scopes | Scope index | Theme |
|-------|--------|-------------|-------|
| 17 | 401–425 | **425** | CI hardening, validation matrix, `clan-rt` no_std |
| 18 | 426–450 | **450** | Production SMP AP scheduler, signed arbitrary ELF corpus |
| 19 | 451–475 | **475** | External network depth (`has_external_network`) |
| 20 | 476–500 | **500** | QEMU→hardware procedure, release scorecard |

---

## Architecture state flags

| Flag | Scope index | Trigger |
|------|-------------|---------|
| `has_no_std_enforcement` | 401 | `clan-rt` `#![no_std]` + host `cargo check` |
| `has_external_network` | 475 | External route smoke + threat re-eval |
| `has_real_hardware_target` | 500+ | Bare-metal procedure documented in [ARCHITECTURE_TARGETS.md](architecture/ARCHITECTURE_TARGETS.md) |

---

## Validation smokes (post-400)

| Line | Scope index | Script |
|------|-------------|--------|
| `ClanOS-Gate: name=ci ok=true` | 425 | `python scripts/gate/run.py --gate ci --timeout 180` |
| `ClanOS-Gate: name=production ok=true` | 450 | `python scripts/gate/run.py --gate production --timeout 180` |
| `ClanOS-Gate: name=network ok=true` | 475 | `python scripts/gate/run.py --gate network --timeout 180` |
| `ClanOS-Gate: ok=true` | 500 | `python scripts/gate/run.py --gate all --timeout 360` |

Prior functional-OS gates remain regression checks: `ClanOS-Gate: name=desktop_preview ok=true`, `ClanOS-Gate: name=desktop ok=true`, `ClanOS-Gate: name=functional ok=true`.

---

## Release falsifiers (fully operational OS)

| Criterion | Falsifier |
|-----------|-----------|
| Functional OS regression | `ClanOS-Gate: name=functional ok=true` smoke false |
| Production SMP | AP scheduler smoke false |
| Signed ELF | `smoke_signed_user_elf` false |
| External network | `ClanOS-Gate: name=network ok=true` smoke false |
| Release gate | `ClanOS-Gate: ok=true` smoke false |

See [RELEASE_SCORECARD.md](RELEASE_SCORECARD.md).
