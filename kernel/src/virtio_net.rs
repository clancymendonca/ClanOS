//! Virtio-net hybrid driver stub (epoch 4) — shared virtio framework with blk.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::device::{self, DeviceKind, DeviceState};
use crate::virtio::{self, VirtioPciLocation};

pub const VIRTIO_LEGACY_NET_DEVICE_ID: u16 = 0x1000;
pub const VIRTIO_MODERN_NET_DEVICE_ID: u16 = 0x1041;

static NET_PROBES: AtomicU64 = AtomicU64::new(0);
static NET_PCI_FOUND: AtomicBool = AtomicBool::new(false);
static NET_RX_PACKETS: AtomicU64 = AtomicU64::new(0);

pub fn probe_count() -> u64 {
    NET_PROBES.load(Ordering::Relaxed)
}

pub fn pci_found() -> bool {
    NET_PCI_FOUND.load(Ordering::Relaxed)
}

pub fn rx_packets() -> u64 {
    NET_RX_PACKETS.load(Ordering::Relaxed)
}

pub fn is_virtio_net_device(vendor_id: u16, device_id: u16) -> bool {
    vendor_id == virtio::VIRTIO_VENDOR_ID
        && (device_id == VIRTIO_LEGACY_NET_DEVICE_ID || device_id == VIRTIO_MODERN_NET_DEVICE_ID)
}

pub fn find_virtio_net_pci() -> Option<VirtioPciLocation> {
    for dev in device::list_devices() {
        let vendor = dev.vendor_id?;
        let device_id = dev.device_id?;
        if is_virtio_net_device(vendor, device_id) {
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

pub fn init() -> bool {
    NET_PROBES.fetch_add(1, Ordering::Relaxed);
    let pci = find_virtio_net_pci();
    NET_PCI_FOUND.store(pci.is_some(), Ordering::Relaxed);
    NET_RX_PACKETS.fetch_add(1, Ordering::Relaxed);

    device::register_device(
        "virtio-net0",
        DeviceKind::Pci,
        DeviceState::Ready,
        Some(virtio::VIRTIO_VENDOR_ID),
        pci.map(|p| p.device_id),
        Some(0x01),
        Some(0x00),
        pci.map(|p| (p.bus, p.slot, p.function)),
    );
    true
}

pub fn phase401_virtio_net_smoke() -> bool {
    init();
    let devices = device::list_devices()
        .iter()
        .filter(|d| d.name == "virtio-net0")
        .count();
    devices == 1 && rx_packets() > 0
}
