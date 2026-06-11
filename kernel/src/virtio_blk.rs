//! Virtio-blk hybrid driver stub (epoch 2).
//!
//! Kernel trampoline + in-memory virtqueue simulation per DRIVER_MODEL.md.
//! Full PCI BAR mapping and userspace ring processing deferred to driver host epoch.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::block::{self, BlockDeviceId};
use crate::device::{self, DeviceKind, DeviceState};
use crate::virtio::{self, VirtioPciLocation};

static VIRTIO_PROBES: AtomicU64 = AtomicU64::new(0);
static VIRTIO_PCI_FOUND: AtomicBool = AtomicBool::new(false);
static VIRTIO_BLK_ID: spin::Mutex<Option<BlockDeviceId>> = spin::Mutex::new(None);

pub fn probe_count() -> u64 {
    VIRTIO_PROBES.load(Ordering::Relaxed)
}

pub fn pci_found() -> bool {
    VIRTIO_PCI_FOUND.load(Ordering::Relaxed)
}

pub fn find_virtio_blk_pci() -> Option<VirtioPciLocation> {
    for dev in device::list_devices() {
        let vendor = dev.vendor_id?;
        let device_id = dev.device_id?;
        if virtio::is_virtio_blk_device(vendor, device_id) {
            return Some(VirtioPciLocation {
                bus: dev.bus.unwrap_or(0),
                slot: dev.slot.unwrap_or(0),
                function: dev.function.unwrap_or(0),
                device_id,
            });
        }
    }
    None
}

/// Probe PCI and register virtio-blk (or hybrid stub when QEMU has no virtio device).
pub fn init() -> bool {
    VIRTIO_PROBES.fetch_add(1, Ordering::Relaxed);
    let pci = find_virtio_blk_pci();
    VIRTIO_PCI_FOUND.store(pci.is_some(), Ordering::Relaxed);

    let sector_count = 128usize;
    let id = block::register_virtio_blk("virtio-blk0", sector_count, false);
    *VIRTIO_BLK_ID.lock() = Some(id);

    device::register_device(
        "virtio-blk0",
        DeviceKind::Block,
        DeviceState::Ready,
        Some(virtio::VIRTIO_VENDOR_ID),
        pci.map(|p| p.device_id),
        Some(0x01),
        Some(0x00),
        pci.map(|p| (p.bus, p.slot, p.function)),
    );

    true
}

pub fn phase201_virtio_blk_smoke() -> bool {
    if VIRTIO_BLK_ID.lock().is_none() {
        init();
    }
    let Some(virtio_id) = *VIRTIO_BLK_ID.lock() else {
        return false;
    };
    let restore = block::active_info().ok().map(|i| i.id);
    let _ = block::set_active(virtio_id);
    let backend_ok = block::active_info()
        .ok()
        .map(|i| i.driver_backed && i.sector_count >= 64)
        .unwrap_or(false);

    let mut write = [0u8; crate::storage::SECTOR_SIZE];
    write[0] = 0x56; // 'V'
    let rw_ok = block::write_active_sector(0, &write).is_ok();
    let mut read = [0u8; crate::storage::SECTOR_SIZE];
    let read_ok = block::read_active_sector(0, &mut read).is_ok() && read[0] == 0x56 && backend_ok;
    if let Some(id) = restore.or_else(block::primary_id) {
        let _ = block::set_active(id);
    }
    read_ok && rw_ok
}

pub fn status() -> (bool, u64, usize) {
    let driver_backed = block::list_block_devices()
        .iter()
        .filter(|d| d.driver_backed)
        .count();
    (pci_found(), probe_count(), driver_backed)
}
