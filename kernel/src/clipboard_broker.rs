//! Clipboard broker stub (scope 126).

use crate::kernel_object::CapError;
use crate::task::process::ProcessId;

static CLIPBOARD_STUB_CALLS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

pub fn clipboard_stub_calls() -> u64 {
    CLIPBOARD_STUB_CALLS.load(core::sync::atomic::Ordering::Relaxed)
}

pub fn request_clipboard_cap(_pid: ProcessId) -> Result<u32, CapError> {
    CLIPBOARD_STUB_CALLS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    Err(CapError::NotFound)
}

pub fn smoke_clipboard_broker() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    request_clipboard_cap(pid).is_err() && clipboard_stub_calls() > 0
}
