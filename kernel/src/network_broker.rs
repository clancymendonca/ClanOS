//! Network broker stub (phase 125) — functional in epoch 4.

use crate::kernel_object::CapError;
use crate::task::process::ProcessId;

static NETWORK_STUB_CALLS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

pub fn network_stub_calls() -> u64 {
    NETWORK_STUB_CALLS.load(core::sync::atomic::Ordering::Relaxed)
}

pub fn request_socket_cap(_pid: ProcessId) -> Result<u32, CapError> {
    NETWORK_STUB_CALLS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    Err(CapError::NotFound)
}

pub fn phase125_network_broker_smoke() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    request_socket_cap(pid).is_err() && network_stub_calls() > 0
}
