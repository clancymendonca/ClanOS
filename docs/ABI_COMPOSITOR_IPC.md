```yaml
status: superseded-by: docs/specs/ABI_COMPOSITOR_IPC.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/specs/ABI_COMPOSITOR_IPC.md`](specs/ABI_COMPOSITOR_IPC.md). This flat copy retained until migration squash reconciles any differences.

# Compositor IPC ABI (Epoch 5 prereq)

```yaml
status: authoritative
semantics_version: 1.0.0
```

Required before phase **145**.

---

## a11y extension point

Versioned optional fields; unknown capability flags **must not error** (minimum contract).

Full a11y platform obligation deferred post-150 per DESIGN_NORTH_STAR.

---

## Epoch 5 deliverables

Compositor isolation smoke; GPU session caps per KERNEL_OBJECT_MODEL `GpuContext`.
