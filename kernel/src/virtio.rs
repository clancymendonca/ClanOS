//! Virtio constants and PCI identification (epoch 2 — VIRTIO_SAFETY.md).

pub const VIRTIO_VENDOR_ID: u16 = 0x1af4;
pub const VIRTIO_LEGACY_BLK_DEVICE_ID: u16 = 0x1001;
pub const VIRTIO_MODERN_BLK_DEVICE_ID: u16 = 0x1042;

pub const VIRTIO_STATUS_ACKNOWLEDGE: u8 = 1;
pub const VIRTIO_STATUS_DRIVER: u8 = 2;
pub const VIRTIO_STATUS_DRIVER_OK: u8 = 4;
pub const VIRTIO_STATUS_FEATURES_OK: u8 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VirtioPciLocation {
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
    pub device_id: u16,
}

pub fn is_virtio_blk_device(vendor_id: u16, device_id: u16) -> bool {
    vendor_id == VIRTIO_VENDOR_ID
        && (device_id == VIRTIO_LEGACY_BLK_DEVICE_ID
            || device_id == VIRTIO_MODERN_BLK_DEVICE_ID)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn virtio_blk_pci_ids_match_red_hat_vendor() {
        assert!(is_virtio_blk_device(
            VIRTIO_VENDOR_ID,
            VIRTIO_LEGACY_BLK_DEVICE_ID
        ));
        assert!(is_virtio_blk_device(
            VIRTIO_VENDOR_ID,
            VIRTIO_MODERN_BLK_DEVICE_ID
        ));
    }

    #[test_case]
    fn virtio_blk_pci_ids_reject_non_virtio() {
        assert!(!is_virtio_blk_device(0x8086, VIRTIO_LEGACY_BLK_DEVICE_ID));
        assert!(!is_virtio_blk_device(VIRTIO_VENDOR_ID, 0x0001));
    }
}
