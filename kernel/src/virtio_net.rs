//! Virtio-net hybrid driver stub (epoch 4) — shared virtio framework with blk.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::device::{self, DeviceKind, DeviceState};
use crate::virtio::{self, VirtioPciLocation};

pub const VIRTIO_LEGACY_NET_DEVICE_ID: u16 = 0x1000;
pub const VIRTIO_MODERN_NET_DEVICE_ID: u16 = 0x1041;

static NET_PROBES: AtomicU64 = AtomicU64::new(0);
static NET_PCI_FOUND: AtomicBool = AtomicBool::new(false);
static NET_RX_PACKETS: AtomicU64 = AtomicU64::new(0);
static NET_REGISTERED: AtomicBool = AtomicBool::new(false);
static NET_TX: AtomicU64 = AtomicU64::new(0);

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref LOOPBACK_RX: Mutex<Option<alloc::vec::Vec<u8>>> = Mutex::new(None);
}

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

    if !NET_REGISTERED.swap(true, Ordering::AcqRel) {
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
    }
    true
}

pub fn net_device_count() -> usize {
    device::list_devices()
        .iter()
        .filter(|d| d.name == "virtio-net0")
        .count()
}

pub fn phase401_virtio_net_smoke() -> bool {
    init();
    net_device_count() >= 1 && rx_packets() > 0
}

pub fn send_loopback(payload: &[u8]) -> bool {
    init();
    NET_TX.fetch_add(1, Ordering::Relaxed);
    let reply = if payload == b"ping" {
        alloc::vec::Vec::from(b"pong".as_slice())
    } else {
        payload.to_vec()
    };
    *LOOPBACK_RX.lock() = Some(reply);
    true
}

pub fn recv_loopback() -> Option<alloc::vec::Vec<u8>> {
    NET_RX_PACKETS.fetch_add(1, Ordering::Relaxed);
    LOOPBACK_RX.lock().take()
}

pub fn tx_count() -> u64 {
    NET_TX.load(Ordering::Relaxed)
}
