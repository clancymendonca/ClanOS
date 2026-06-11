//! In-kernel storage broker (phase 118/122); mints FsNode caps from grants via interim IPC.

use crate::ipc_interim_bridge;
use crate::kernel_object::{self, CapError, Rights};
use crate::task::process::ProcessId;

static BROKER_MINTS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

pub fn broker_mint_count() -> u64 {
    BROKER_MINTS.load(core::sync::atomic::Ordering::Relaxed)
}

/// Mint a cap for `pid` from an existing storage grant (no ambient `/`).
pub fn grant_fsnode(pid: ProcessId, grant_id: u32) -> Result<u32, CapError> {
    if !kernel_object::storage_grant_by_id(grant_id).is_some() {
        return Err(CapError::NotFound);
    }
    let slot = kernel_object::mint_cap_from_grant(pid, grant_id)?;
    BROKER_MINTS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    Ok(slot)
}

/// Phase 122: IPC-mediated storage grant (compat-internal FIFO session).
pub fn request_fs_grant_via_ipc(
    pid: ProcessId,
    session_id: u32,
    grant_id: u32,
) -> Result<u32, CapError> {
    if ipc_interim_bridge::is_retired() {
        return request_fs_grant_via_endpoint(pid, grant_id);
    }
    ipc_interim_bridge::send(pid, session_id, b"fs-grant-req")
        .map_err(|_| CapError::InvalidArgument)?;
    let _ = ipc_interim_bridge::recv(pid, session_id).map_err(|_| CapError::InvalidArgument)?;
    grant_fsnode(pid, grant_id)
}

/// Post-134: native endpoint path for broker storage grants.
pub fn request_fs_grant_via_endpoint(pid: ProcessId, grant_id: u32) -> Result<u32, CapError> {
    let ep = crate::ipc_endpoints::create_endpoint();
    crate::ipc_endpoints::send(ep, pid, b"fs-grant-req")
        .map_err(|_| CapError::InvalidArgument)?;
    let _ = crate::ipc_endpoints::recv(ep).map_err(|_| CapError::InvalidArgument)?;
    grant_fsnode(pid, grant_id)
}

pub fn phase122_storage_broker_smoke() -> bool {
    let Some(pid) = kernel_object::ensure_smoke_process() else {
        return false;
    };
    crate::task::process::set_process_mode(pid, crate::task::process::ProcessMode::Native);
    let _ = crate::service_loader::bootstrap_root_caps(pid);
    let grant = kernel_object::create_storage_grant(99, Rights::read_write()).ok();
    let minted = grant.and_then(|g| request_fs_grant_via_ipc(pid, 5, g).ok());
    let ok = minted.is_some();
    crate::task::process::set_process_mode(pid, crate::task::process::ProcessMode::Compat);
    ok
}

pub fn phase118_broker_mint_smoke() -> bool {
    let Some(pid) = kernel_object::ensure_smoke_process() else {
        return false;
    };
    crate::task::process::set_process_mode(pid, crate::task::process::ProcessMode::Native);
    let grant = kernel_object::create_storage_grant(7, Rights::read_write()).ok();
    let minted = grant.and_then(|g| grant_fsnode(pid, g).ok());
    let ok = minted.is_some() && kernel_object::cap_count_for_process(pid) > 0;
    crate::task::process::set_process_mode(pid, crate::task::process::ProcessMode::Compat);
    ok
}
