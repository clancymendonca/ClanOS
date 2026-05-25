//! In-kernel storage broker stub (phase 118); mints FsNode caps from grants (phase 114).

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
