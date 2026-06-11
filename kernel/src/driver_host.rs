//! Userspace driver host stub (epoch 11 DRIVER_MODEL).

use core::sync::atomic::{AtomicU64, Ordering};

static DRIVER_MOUNTS: AtomicU64 = AtomicU64::new(0);

pub fn driver_mount_count() -> u64 {
    DRIVER_MOUNTS.load(Ordering::Relaxed)
}

pub fn mount_userspace_driver(driver_id: u32) -> bool {
    if driver_id == 0 {
        return false;
    }
    DRIVER_MOUNTS.fetch_add(1, Ordering::Relaxed);
    true
}

pub fn epoch11_driver_graduated() -> bool {
    mount_userspace_driver(1) && driver_mount_count() > 0
}
