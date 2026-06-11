//! Network broker (phase 125) — functional socket caps in epoch 4.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::kernel_object::{self, CapError, Rights};
use crate::task::process::ProcessId;

static NETWORK_STUB_CALLS: AtomicU64 = AtomicU64::new(0);
static EPOCH4_FUNCTIONAL: AtomicBool = AtomicBool::new(false);

pub fn network_stub_calls() -> u64 {
    NETWORK_STUB_CALLS.load(Ordering::Relaxed)
}

pub fn enable_epoch4_functional() {
    EPOCH4_FUNCTIONAL.store(true, Ordering::Release);
}

pub fn request_socket_cap(pid: ProcessId) -> Result<u32, CapError> {
    NETWORK_STUB_CALLS.fetch_add(1, Ordering::Relaxed);
    if !EPOCH4_FUNCTIONAL.load(Ordering::Acquire) {
        return Err(CapError::NotFound);
    }
    let grant = kernel_object::create_storage_grant(0x4e45_5400, Rights::read_write())?;
    kernel_object::mint_cap_from_grant(pid, grant)
}

pub fn phase125_network_broker_smoke() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    request_socket_cap(pid).is_err() && network_stub_calls() > 0
}

pub fn phase403_network_broker_functional_smoke() -> bool {
    enable_epoch4_functional();
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    crate::task::process::set_process_mode(pid, crate::task::process::ProcessMode::Native);
    let _ = crate::service_loader::bootstrap_root_caps(pid);
    let ok = request_socket_cap(pid).is_ok();
    crate::task::process::set_process_mode(pid, crate::task::process::ProcessMode::Compat);
    ok
}
