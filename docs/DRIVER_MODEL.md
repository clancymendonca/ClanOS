```yaml
status: superseded-by: docs/architecture/DRIVER_MODEL.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/architecture/DRIVER_MODEL.md`](architecture/DRIVER_MODEL.md). This flat copy retained until migration squash reconciles any differences.

# Driver Model

```yaml
status: authoritative
semantics_version: 1.1.0
```

See `DECISION_LOG.md#driver_isolation_model`, [`VIRTIO_SAFETY.md`](VIRTIO_SAFETY.md) (epoch 2).

---

## Adopted model: hybrid

| Layer | Responsibility |
|-------|----------------|
| **Kernel trampoline** | MMIO map/unmap gates, IRQ delivery, DMA buffer pinning, IOMMU stub (QEMU) |
| **Userspace driver host** | Virtio protocol, queue processing, error recovery |
| **Device caps** | `device.block`, `device.net` attenuated per device instance |

Drivers run in a **privileged service process**, not in the general app sandbox. Kernel does not parse virtio rings in TCB beyond validation hooks.

---

## Epoch 2 deliverables

- virtio-blk via hybrid model before userland epoch 2 bootstrap
- Threat nodes for DMA confusion and MMIO escape closed or deferred with trigger
