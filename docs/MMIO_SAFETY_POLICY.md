# MMIO Safety Policy (Epoch 2 planning)

```yaml
status: authoritative
semantics_version: 1.0.0
```

---

## Rules

- All MMIO mappings require `device.*` cap
- Kernel trampoline validates offset + length before userspace mapping
- No ambient PCI config space access

See VIRTIO_SAFETY.md and DRIVER_MODEL.md.
