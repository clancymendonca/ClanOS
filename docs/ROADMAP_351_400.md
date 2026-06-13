# Post-350 Roadmap (Phases 351–400)

```yaml
status: authoritative
semantics_version: 1.0.0
```

Continues from [ROADMAP_151_350.md](ROADMAP_151_350.md) milestone 350. Goal: **functional desktop OS** — interactive GUI, native userland, working network, and installable apps.

**Pace:** 1–3 phases/month.

---

## Epoch map

| Epoch | Phases | Milestone | Theme |
|-------|--------|-----------|-------|
| 15 | 351–375 | **375** | Graphics desktop: framebuffer, compositor, shell UI |
| 16 | 376–400 | **400** | Native userland production, functional network, app ecosystem |

---

## Phases 351–375 (Epoch 15 → Desktop)

| Phase | Title | Layer |
|------:|-------|-------|
| 351 | VGA mode 13h framebuffer + desktop shell frame | kernel |
| 352 | Mouse input + window focus model | kernel |
| 353 | Compositor damage regions + double buffer | kernel |
| 354 | Desktop shell service (taskbar, launcher) | platform |
| 355 | Font renderer + text labels in GUI | kernel |
| 356–374 | Window manager, a11y hooks, GPU broker path | mixed |
| 375 | **Milestone 375** desktop integration gate | governance |

---

## Phases 376–400 (Epoch 16 → Userland + Network)

| Phase | Title |
|------:|-------|
| 376–385 | ares-rt ring-3 ELF install + syscall surface |
| 386–395 | Virtio-net TX/RX path; compat socket depth |
| 396–399 | Package install hook; `/bin` native programs |
| 400 | **Milestone 400** functional OS gate | **complete** |

---

## Milestone 400 falsifiers

| Criterion | Falsifier |
|-----------|-----------|
| Desktop | AresOS-Gate: name=desktop ok=true smoke false |
| Native apps | No ring-3 ELF runs from `/bin` manifest |
| Network | Loopback ping smoke false |
| Boot | AresOS-Gate: name=release ok=true regression |
