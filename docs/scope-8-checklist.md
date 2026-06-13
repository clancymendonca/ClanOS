> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 8 Checklist (Device & Block Driver Bring-Up)

**Date**: 2026-05-13  
**Status**: Complete

## 1. Device Layer Foundation

- [x] `DeviceId`, `DeviceKind`, `DeviceState`, `DeviceInfo`, and `DeviceError`
- [x] Global device registry
- [x] Device register/list/query helpers
- [x] Device summary counts for runtime diagnostics

## 2. PCI Discovery Skeleton

- [x] QEMU-safe PCI config-space scanner
- [x] Vendor/device/class/subclass discovery
- [x] PCI devices registered into the device registry
- [x] Empty-scan fallback represented without panic

## 3. Block Device Manager

- [x] `BlockDeviceId`, backend metadata, and block registry
- [x] Active block-device selection
- [x] Sector read/write through active backend
- [x] Storage reports active backend and driver-backed status

## 4. QEMU-Friendly Backend

- [x] Simulated QEMU-style driver-backed block backend
- [x] Scope 7 `SimpleFs` mounted through managed block-device backend
- [x] Read/write/remount smoke check through driver-backed path

## 5. Shell, Syscalls, and Observability

- [x] Shell commands: `devices`, `blk list`, `blk info <id>`, `mount <block-id>`
- [x] Device/block count syscalls
- [x] `fsinfo` reports block-device count
- [x] Covered by boot gate `shell_storage` (`ClanOS-BootGate: name=shell_storage ok=true`)

## 6. Validation

- [x] `python scripts/gate/boot.py --gate shell_storage --timeout 180` for QEMU-backed validation
- [x] `scripts/validation_matrix.py` includes `boot-gate-check`
- [x] Integration tests cover device registry, block registry, and storage-through-manager behavior

## Validation

```bash
cargo check -p kernel
python scripts/gate/boot.py --gate shell_storage --timeout 180
python scripts/validation_matrix.py --smoke-timeout 180
```

See [VALIDATION_GATES.md](VALIDATION_GATES.md).


## Known Limits

- The shipped block backend is simulated and driver-plumbed, not a full AHCI/NVMe/virtio implementation.
- PCI scanning is read-only enumeration for observability and future driver binding.
- Block I/O is synchronous and polling-style.
- DMA, MSI/MSI-X, interrupt-driven I/O, and production hardware drivers are deferred.
