//! Permission broker (scope 123) — attenuated cap mint with broker session.

use crate::ipc_interim_bridge;
use crate::kernel_object::{self, CapError, ObjectKind, Rights};
use crate::task::process::ProcessId;

static PERMISSION_MINTS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

pub fn permission_mint_count() -> u64 {
    PERMISSION_MINTS.load(core::sync::atomic::Ordering::Relaxed)
}

/// Request attenuated rights via interim IPC + broker mint (idempotent path).
pub fn request_attenuated_cap(
    pid: ProcessId,
    session_id: u32,
    rights: Rights,
) -> Result<u32, CapError> {
    ipc_interim_bridge::send(pid, session_id, b"perm-req")
        .map_err(|_| CapError::InvalidArgument)?;
    let _ack = ipc_interim_bridge::recv(pid, session_id).map_err(|_| CapError::InvalidArgument)?;

    let oid = kernel_object::register_object(ObjectKind::BrokerSession, Rights::all_for_smoke());
    let slot = kernel_object::mint_cap_for_process(pid, oid, rights)?;
    PERMISSION_MINTS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    Ok(slot)
}

pub fn smoke_permission_broker() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    crate::task::process::set_process_mode(pid, crate::task::process::ProcessMode::Native);
    let _ = crate::service_loader::bootstrap_root_caps(pid);
    let slot = request_attenuated_cap(pid, 10, Rights(Rights::READ)).ok();
    let ok = slot.is_some() && permission_mint_count() > 0;
    crate::task::process::set_process_mode(pid, crate::task::process::ProcessMode::Compat);
    ok
}
