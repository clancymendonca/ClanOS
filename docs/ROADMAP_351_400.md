# Post-350 Roadmap (Scopes 351–400)

```yaml
status: authoritative
semantics_version: 1.0.0
```

Continues from [ROADMAP_151_350.md](ROADMAP_151_350.md) (release 1.0 epoch, scope 350). Goal: **functional desktop OS** — interactive GUI, native userland, working network, and installable apps.

**Pace:** 1–3 scopes/month.

> **Note:** Scope-index labels (375, 400, …) name integration checkpoints in this roadmap. Runtime validation uses unified subsystem gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md).

---

## Epoch map

| Epoch | Scopes | Scope index | Theme |
|-------|--------|-------------|-------|
| 15 | 351–375 | **375** | Graphics desktop: framebuffer, compositor, shell UI |
| 16 | 376–400 | **400** | Native userland production, functional network, app ecosystem |

---

## Scopes 351–375 (Epoch 15 → Desktop)

| Scope | Title | Layer |
|------:|-------|-------|
| 351 | VGA mode 13h framebuffer + desktop shell frame | kernel |
| 352 | Mouse input + window focus model | kernel |
| 353 | Compositor damage regions + double buffer | kernel |
| 354 | Desktop shell service (taskbar, launcher) | platform |
| 355 | Font renderer + text labels in GUI | kernel |
| 356–374 | Window manager, a11y hooks, GPU broker path | mixed |
| 375 | **Scope 375** desktop integration gate | governance |

---

## Scopes 376–400 (Epoch 16 → Userland + Network)

| Scope | Title |
|------:|-------|
| 376–385 | Clan OS runtime (`clan-rt`) ring-3 ELF install + syscall surface |
| 386–395 | Virtio-net TX/RX path; compat socket depth |
| 396–399 | Package install hook; `/bin` native programs |
| 400 | **Scope 400** functional OS gate | **complete** |

---

## Functional OS falsifiers

| Criterion | Falsifier |
|-----------|-----------|
| Desktop | ClanOS-Gate: name=desktop ok=true smoke false |
| Native apps | No ring-3 ELF runs from `/bin` manifest |
| Network | Loopback ping smoke false |
| Boot | ClanOS-Gate: name=release ok=true regression |

See [RELEASE_SCORECARD.md](RELEASE_SCORECARD.md).
