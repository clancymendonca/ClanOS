//! Device broker skeleton (phase 124).

use crate::kernel_object::{self, CapError, ObjectKind, Rights};
use crate::task::process::ProcessId;

static DEVICE_MINTS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

pub fn device_mint_count() -> u64 {
    DEVICE_MINTS.load(core::sync::atomic::Ordering::Relaxed)
}

pub fn grant_device_cap(pid: ProcessId, device_local_id: u32) -> Result<u32, CapError> {
    let oid = kernel_object::register_object(ObjectKind::Device, Rights::all_for_smoke());
    let _ = device_local_id;
    let slot = kernel_object::mint_cap_for_process(pid, oid, Rights(Rights::READ | Rights::MAP))?;
    DEVICE_MINTS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    Ok(slot)
}

pub fn phase124_device_broker_smoke() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    crate::task::process::set_process_mode(pid, crate::task::process::ProcessMode::Native);
    let _ = crate::service_loader::bootstrap_root_caps(pid);
    let slot = grant_device_cap(pid, 1).ok();
    let ok = slot.is_some();
    crate::task::process::set_process_mode(pid, crate::task::process::ProcessMode::Compat);
    ok
}
