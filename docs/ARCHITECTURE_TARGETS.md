```yaml
status: superseded-by: docs/architecture/ARCHITECTURE_TARGETS.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/architecture/ARCHITECTURE_TARGETS.md`](architecture/ARCHITECTURE_TARGETS.md). This flat copy retained until migration squash reconciles any differences.

# Architecture Targets (Epoch 6)

```yaml
status: authoritative
semantics_version: 1.0.0
```

---

## Primary ISA

**x86_64** — QEMU first; real hardware epoch 6.

---

## QEMU

Versioned config script: [`scripts/qemu/x86_64-config.ps1`](../scripts/qemu/x86_64-config.ps1) (v1 — virtio-blk-pci, `-smp 2`). SMP enabled phases 141+.

---

## Deferred

| Target | Trigger |
|--------|---------|
| TPM / measured boot | `has_tpm_integration` |
| Bare metal NIC/AHCI | `has_real_hardware_target` |
| Hypervisor guest | `has_hypervisor_guest_target` |

---

## Power / thermal

Out of scope until epoch 6; benchmarks assume no throttling.
