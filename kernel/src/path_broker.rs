//! Compat-only path broker (scope 115, G1): resolves paths to FDs without new handle types.

use crate::task::process::ProcessId;

static BROKER_OPENS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

pub fn broker_open_count() -> u64 {
    BROKER_OPENS.load(core::sync::atomic::Ordering::Relaxed)
}

/// Resolve a compat path and allocate an FD via the existing fd table.
pub fn resolve_open_compat(pid: ProcessId, path: &str) -> Result<u32, ()> {
    BROKER_OPENS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    crate::fd_table::open_file_for_process_inner(pid, path)
}

pub fn smoke_path_broker_smoke() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    crate::task::process::set_smoke_process_id(Some(pid));
    let before = broker_open_count();
    let fd = resolve_open_compat(pid, "/bin/hello").ok();
    let after = broker_open_count();
    crate::task::process::set_smoke_process_id(None);
    fd.is_some() && after > before
}
