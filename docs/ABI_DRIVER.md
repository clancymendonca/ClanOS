# Driver ABI — Distrustful Entities

Drivers (including GPU/vendor code) are **untrusted by default**. Phase **104** boundary doc; userspace driver host phase **144+**.

See: [DEVICES.md](DEVICES.md), [AXIOMS.md](AXIOMS.md), [SEMANTIC_JURISDICTION.md](SEMANTIC_JURISDICTION.md).

---

## Principles

| Rule | Detail |
|------|--------|
| No arbitrary kernel memory | MMIO and DMA only via declared caps |
| No unrestricted DMA | IOMMU / DMA cap limits (phase 146 narrative) |
| Userspace first | Drivers run as services with Device caps |
| Restartable stacks | GPU fault → compositor restart → apps survive (phase 145) |

---

## Device cap model (future)

Native **Device** kernel object — rights subset: `map_mmio`, `submit_dma`, `irq_bind` — attenuated per device node from device broker.

Compat kernel drivers from phases 1–100 remain for QEMU bring-up; native path migrates to broker + userspace host without expanding TCB.

---

## Compositor / GPU (phase 145 sketch)

| Event | Behavior |
|-------|----------|
| GPU driver crash | Compositor service restarts; GpuContext generation bump |
| Apps | Hold display caps via broker; survive compositor restart per E-03 |

---

## Phase 100 compat note

Block manager and PCI skeleton ([DEVICES.md](DEVICES.md)) are in-kernel for validation — documented as **compat-era** machinery, not native trust model target.
