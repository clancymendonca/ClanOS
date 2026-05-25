//! Universal kernel object model and per-process capability table (phases 111–113).
//!
//! Single cap handle table per process: `(ObjectId, Kind, Rights, Generation)`.
//! See docs/KERNEL_OBJECT_MODEL.md and docs/RIGHTS_ALGEBRA.md.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

use lazy_static::lazy_static;
use spin::Mutex;

use crate::task::process::{self, ProcessId};

pub const MAX_CAPS: usize = 16;

/// Stable object identity (phase 110 immutable identity decision).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId(pub u64);

impl ObjectId {
    pub const fn from_raw(id: u64) -> Self {
        ObjectId(id)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

/// Invalidation epoch; bump on revoke (R-03).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Generation(pub u32);

impl Generation {
    pub const INITIAL: Generation = Generation(1);

    pub fn bump(self) -> Self {
        Generation(self.0.saturating_add(1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectKind {
    Process,
    Endpoint,
    MemoryRegion,
    Service,
    Device,
    FsNode,
    GpuContext,
    BrokerSession,
}

/// Rights subset for a cap (G2 monotonicity).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rights(pub u32);

impl Rights {
    pub const READ: u32 = 1 << 0;
    pub const WRITE: u32 = 1 << 1;
    pub const MAP: u32 = 1 << 2;
    pub const DELEGATE: u32 = 1 << 3;
    pub const REVOKE: u32 = 1 << 4;

    pub const fn empty() -> Self {
        Rights(0)
    }

    pub const fn read_write() -> Self {
        Rights(Self::READ | Self::WRITE)
    }

    pub const fn all_for_smoke() -> Self {
        Rights(Self::READ | Self::WRITE | Self::MAP | Self::DELEGATE | Self::REVOKE)
    }

    pub fn contains(self, other: Rights) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn intersect(self, other: Rights) -> Rights {
        Rights(self.0 & other.0)
    }
}

#[derive(Debug, Clone)]
pub struct CapSlotStorage {
    pub object_id: ObjectId,
    pub kind: ObjectKind,
    pub generation: Generation,
    pub rights: Rights,
}

#[derive(Debug, Clone)]
struct KernelObjectRecord {
    kind: ObjectKind,
    generation: Generation,
    max_rights: Rights,
    /// Broker-local id for FsNode (no path string in cap metadata, phase 114).
    fsnode_local_id: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct StorageGrant {
    pub grant_id: u32,
    pub object_id: ObjectId,
    pub rights: Rights,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapError {
    NoSlot,
    NotFound,
    InvalidGeneration,
    AmplificationDenied,
    AmbientDenied,
    InvalidArgument,
}

static NEXT_OBJECT_ID: AtomicU64 = AtomicU64::new(1);
static NEXT_GRANT_ID: AtomicU32 = AtomicU32::new(1);
static AMPLIFICATION_DENIED: AtomicU64 = AtomicU64::new(0);
static CAP_CREATES: AtomicU64 = AtomicU64::new(0);
static CAP_CLOSES: AtomicU64 = AtomicU64::new(0);
static CAP_TRANSFERS: AtomicU64 = AtomicU64::new(0);
static CAP_DELEGATES: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    static ref OBJECT_REGISTRY: Mutex<BTreeMap<ObjectId, KernelObjectRecord>> =
        Mutex::new(BTreeMap::new());
    static ref STORAGE_GRANTS: Mutex<Vec<StorageGrant>> = Mutex::new(Vec::new());
}

pub fn amplification_denied_count() -> u64 {
    AMPLIFICATION_DENIED.load(Ordering::Relaxed)
}

pub fn cap_status() -> (u64, u64, u64, u64) {
    (
        CAP_CREATES.load(Ordering::Relaxed),
        CAP_CLOSES.load(Ordering::Relaxed),
        CAP_TRANSFERS.load(Ordering::Relaxed),
        CAP_DELEGATES.load(Ordering::Relaxed),
    )
}

pub fn register_object(kind: ObjectKind, max_rights: Rights) -> ObjectId {
    let id = ObjectId(NEXT_OBJECT_ID.fetch_add(1, Ordering::Relaxed));
    OBJECT_REGISTRY.lock().insert(
        id,
        KernelObjectRecord {
            kind,
            generation: Generation::INITIAL,
            max_rights,
            fsnode_local_id: None,
        },
    );
    id
}

pub fn register_fsnode_object(local_id: u32, max_rights: Rights) -> ObjectId {
    let id = register_object(ObjectKind::FsNode, max_rights);
    if let Some(rec) = OBJECT_REGISTRY.lock().get_mut(&id) {
        rec.fsnode_local_id = Some(local_id);
    }
    id
}

pub fn object_generation(object_id: ObjectId) -> Option<Generation> {
    OBJECT_REGISTRY
        .lock()
        .get(&object_id)
        .map(|r| r.generation)
}

pub fn bump_object_generation(object_id: ObjectId) -> bool {
    let mut reg = OBJECT_REGISTRY.lock();
    if let Some(rec) = reg.get_mut(&object_id) {
        rec.generation = rec.generation.bump();
        true
    } else {
        false
    }
}

fn validate_cap_slot(
    object_id: ObjectId,
    kind: ObjectKind,
    generation: Generation,
    rights: Rights,
) -> Result<(), CapError> {
    let reg = OBJECT_REGISTRY.lock();
    let rec = reg.get(&object_id).ok_or(CapError::NotFound)?;
    if rec.kind != kind {
        return Err(CapError::NotFound);
    }
    if rec.generation != generation {
        return Err(CapError::InvalidGeneration);
    }
    if !rec.max_rights.contains(rights) {
        return Err(CapError::AmplificationDenied);
    }
    Ok(())
}

pub fn native_ambient_allows(pid: ProcessId) -> bool {
    if process::process_mode(pid) != process::ProcessMode::Native {
        return true;
    }
    cap_count_for_process(pid) > 0
}

pub fn cap_count_for_process(pid: ProcessId) -> usize {
    process::with_process_mut(pid, |p| p.caps().iter().filter(|s| s.is_some()).count())
        .unwrap_or(0)
}

pub fn mint_cap_for_process(
    pid: ProcessId,
    object_id: ObjectId,
    rights: Rights,
) -> Result<u32, CapError> {
    if process::process_mode(pid) == process::ProcessMode::Native && cap_count_for_process(pid) == 0 {
        return Err(CapError::AmbientDenied);
    }
    let reg = OBJECT_REGISTRY.lock();
    let rec = reg.get(&object_id).ok_or(CapError::NotFound)?;
    if !rec.max_rights.contains(rights) {
        AMPLIFICATION_DENIED.fetch_add(1, Ordering::Relaxed);
        return Err(CapError::AmplificationDenied);
    }
    let slot_storage = CapSlotStorage {
        object_id,
        kind: rec.kind,
        generation: rec.generation,
        rights,
    };
    drop(reg);
    alloc_cap_slot(pid, slot_storage).map_err(|_| CapError::NoSlot)
}

pub fn alloc_cap_slot(pid: ProcessId, cap: CapSlotStorage) -> Result<u32, CapError> {
    validate_cap_slot(cap.object_id, cap.kind, cap.generation, cap.rights)?;
    match process::with_process_mut(pid, |process| {
        for (idx, slot) in process.caps_mut().iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(cap);
                CAP_CREATES.fetch_add(1, Ordering::Relaxed);
                return Ok(idx as u32);
            }
        }
        Err(CapError::NoSlot)
    }) {
        Some(Ok(fd)) => Ok(fd),
        Some(Err(e)) => Err(e),
        None => Err(CapError::NotFound),
    }
}

pub fn close_cap_for_process(pid: ProcessId, slot: u32) -> Result<(), CapError> {
    let idx = slot as usize;
    match process::with_process_mut(pid, |process| {
        let caps = process.caps_mut();
        if idx >= MAX_CAPS {
            return Err(CapError::NotFound);
        }
        if caps[idx].take().is_some() {
            CAP_CLOSES.fetch_add(1, Ordering::Relaxed);
            Ok(())
        } else {
            Err(CapError::NotFound)
        }
    }) {
        Some(Ok(())) => Ok(()),
        Some(Err(e)) => Err(e),
        None => Err(CapError::NotFound),
    }
}

pub fn get_cap(pid: ProcessId, slot: u32) -> Option<CapSlotStorage> {
    let idx = slot as usize;
    process::with_process_mut(pid, |process| {
        process.caps().get(idx).and_then(|s| s.clone())
    })
    .flatten()
}

pub fn cap_delegate(
    pid: ProcessId,
    parent_slot: u32,
    child_rights: Rights,
) -> Result<u32, CapError> {
    let parent = get_cap(pid, parent_slot).ok_or(CapError::NotFound)?;
    if !parent.rights.contains(child_rights) {
        AMPLIFICATION_DENIED.fetch_add(1, Ordering::Relaxed);
        return Err(CapError::AmplificationDenied);
    }
    if !parent.rights.contains(Rights(Rights::DELEGATE)) {
        AMPLIFICATION_DENIED.fetch_add(1, Ordering::Relaxed);
        return Err(CapError::AmplificationDenied);
    }
    let child = CapSlotStorage {
        object_id: parent.object_id,
        kind: parent.kind,
        generation: parent.generation,
        rights: child_rights,
    };
    let slot = alloc_cap_slot(pid, child)?;
    CAP_DELEGATES.fetch_add(1, Ordering::Relaxed);
    Ok(slot)
}

pub fn cap_transfer_move(
    from_pid: ProcessId,
    from_slot: u32,
    to_pid: ProcessId,
) -> Result<u32, CapError> {
    let cap = get_cap(from_pid, from_slot).ok_or(CapError::NotFound)?;
    close_cap_for_process(from_pid, from_slot)?;
    let to_slot = alloc_cap_slot(to_pid, cap)?;
    CAP_TRANSFERS.fetch_add(1, Ordering::Relaxed);
    Ok(to_slot)
}

pub fn create_storage_grant(fsnode_local_id: u32, rights: Rights) -> Result<u32, CapError> {
    let object_id = register_fsnode_object(fsnode_local_id, rights);
    let grant_id = NEXT_GRANT_ID.fetch_add(1, Ordering::Relaxed);
    STORAGE_GRANTS.lock().push(StorageGrant {
        grant_id,
        object_id,
        rights,
    });
    Ok(grant_id)
}

pub fn storage_grant_by_id(grant_id: u32) -> Option<StorageGrant> {
    STORAGE_GRANTS
        .lock()
        .iter()
        .find(|g| g.grant_id == grant_id)
        .cloned()
}

pub fn mint_cap_from_grant(pid: ProcessId, grant_id: u32) -> Result<u32, CapError> {
    let grant = storage_grant_by_id(grant_id).ok_or(CapError::NotFound)?;
    mint_cap_for_process_bootstrap(pid, grant.object_id, grant.rights)
}

fn mint_cap_for_process_bootstrap(
    pid: ProcessId,
    object_id: ObjectId,
    rights: Rights,
) -> Result<u32, CapError> {
    let reg = OBJECT_REGISTRY.lock();
    let rec = reg.get(&object_id).ok_or(CapError::NotFound)?;
    if !rec.max_rights.contains(rights) {
        AMPLIFICATION_DENIED.fetch_add(1, Ordering::Relaxed);
        return Err(CapError::AmplificationDenied);
    }
    let slot_storage = CapSlotStorage {
        object_id,
        kind: rec.kind,
        generation: rec.generation,
        rights,
    };
    drop(reg);
    alloc_cap_slot(pid, slot_storage).map_err(|_| CapError::NoSlot)
}

pub fn ensure_smoke_process() -> Option<ProcessId> {
    if let Some(pid) = process::smoke_process_id() {
        return Some(pid);
    }
    process::create_process_for_smoke("cap-smoke")
}

pub fn phase111_kernel_object_smoke() -> bool {
    let Some(pid) = ensure_smoke_process() else {
        return false;
    };
    process::set_smoke_process_id(Some(pid));
    let object_id = register_object(ObjectKind::MemoryRegion, Rights::all_for_smoke());
    let gen = object_generation(object_id) == Some(Generation::INITIAL);
    let slot = mint_cap_for_process(pid, object_id, Rights::read_write()).ok();
    let lookup = slot
        .and_then(|s| get_cap(pid, s))
        .map(|c| c.generation == Generation::INITIAL && c.kind == ObjectKind::MemoryRegion)
        .unwrap_or(false);
    gen && lookup
}

pub fn phase112_cap_lifecycle_smoke() -> bool {
    let Some(pid_a) = ensure_smoke_process() else {
        return false;
    };
    let Some(pid_b) = process::create_process_for_smoke("cap-transfer") else {
        return false;
    };
    let oid = register_object(ObjectKind::FsNode, Rights::all_for_smoke());
    let slot_a = mint_cap_for_process(pid_a, oid, Rights::read_write()).ok();
    let Some(slot_a) = slot_a else {
        return false;
    };
    let moved = crate::native_syscall::invoke_native(
        crate::native_syscall::NativeSyscallId::CapTransfer as u64,
        pid_a,
        slot_a as u64,
        pid_b.as_u64(),
        0,
    )
    .ok();
    let sender_empty = get_cap(pid_a, slot_a).is_none();
    let receiver = moved
        .and_then(|s| get_cap(pid_b, s as u32))
        .is_some();
    let closed = moved
        .and_then(|s| {
            crate::native_syscall::invoke_native(
                crate::native_syscall::NativeSyscallId::CapClose as u64,
                pid_b,
                s,
                0,
                0,
            )
            .ok()
        })
        .is_some();
    sender_empty && receiver && closed
}

pub fn phase113_rights_smoke() -> bool {
    let Some(pid) = ensure_smoke_process() else {
        return false;
    };
    let oid = register_object(ObjectKind::FsNode, Rights::all_for_smoke());
    let parent_rights = Rights(Rights::READ | Rights::DELEGATE);
    let parent = mint_cap_for_process(pid, oid, parent_rights).ok();
    let Some(parent) = parent else {
        return false;
    };
    let child_ok = cap_delegate(pid, parent, Rights(Rights::READ)).is_ok();
    let amp_before = amplification_denied_count();
    let amp_fail = cap_delegate(pid, parent, Rights::read_write()).is_err();
    let amp_after = amplification_denied_count() > amp_before;
    child_ok && amp_fail && amp_after
}

pub fn phase114_storage_grant_smoke() -> bool {
    let Some(pid) = ensure_smoke_process() else {
        return false;
    };
    process::set_process_mode(pid, process::ProcessMode::Native);
    let grant = create_storage_grant(42, Rights::read_write()).ok();
    let minted = grant.and_then(|g| mint_cap_from_grant(pid, g).ok());
    let cap = minted.and_then(|s| get_cap(pid, s));
    let no_path = cap
        .map(|c| c.kind == ObjectKind::FsNode && c.rights.contains(Rights::read_write()))
        .unwrap_or(false);
    process::set_process_mode(pid, process::ProcessMode::Compat);
    no_path
}

pub fn phase116_ambient_deny_smoke() -> bool {
    let Some(pid) = process::create_process_for_smoke("native-zero-cap")
    else {
        return false;
    };
    process::set_process_mode(pid, process::ProcessMode::Native);
    let count_zero = cap_count_for_process(pid) == 0;
    let deny = !native_ambient_allows(pid);
    let mint_fail = {
        let oid = register_object(ObjectKind::FsNode, Rights::read_write());
        mint_cap_for_process(pid, oid, Rights(Rights::READ)) == Err(CapError::AmbientDenied)
    };
    let broker_fail = crate::storage_broker::grant_fsnode(pid, 999).is_err();
    process::set_process_mode(pid, process::ProcessMode::Compat);
    count_zero && deny && mint_fail && broker_fail
}

pub fn phase117_namespace_smoke() -> bool {
    let Some(native_pid) = process::create_process_for_smoke("native-ns") else {
        return false;
    };
    let Some(compat_pid) = process::create_process_for_smoke("compat-ns")
    else {
        return false;
    };
    process::set_process_mode(native_pid, process::ProcessMode::Native);
    process::set_process_mode(compat_pid, process::ProcessMode::Compat);
    process::set_smoke_process_id(Some(compat_pid));
    let native_blocked = process::native_blocks_path_probe(native_pid);
    let compat_ok = !process::native_blocks_path_probe(compat_pid);
    process::set_smoke_process_id(None);
    native_blocked && compat_ok
}

pub fn phase119_compat_bridge_smoke() -> bool {
    let hw_ok = crate::user_syscall_hw::phase71_smoke();
    let fd45 = crate::fd_table::phase45_smoke();
    let fd51 = crate::fd_table::phase51_smoke();
    let allowlist = !crate::user_syscall_hw::ALLOWED_HW_SYSCALLS.is_empty()
        && crate::user_syscall_hw::ALLOWED_HW_SYSCALLS.len() >= 24;
    let max_id = crate::user_syscall_hw::ALLOWED_HW_SYSCALLS
        .iter()
        .map(|id| *id as u64)
        .max()
        .unwrap_or(0);
    hw_ok && fd45 && fd51 && allowlist && max_id <= 82
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: R-01
    #[test]
    fn delegate_subset_of_parent() {
        assert!(Rights::read_write().contains(Rights(Rights::READ)));
    }

    // spec: R-06
    #[test]
    fn amplification_denied_when_not_subset() {
        let parent = Rights(Rights::READ);
        let child = Rights::read_write();
        assert!(!parent.contains(child));
    }
}
