//! Interim IPC bridge (`compat-internal`) — phases 122–133.
//! See docs/IPC_INTERIM_BRIDGE.md.

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use lazy_static::lazy_static;
use spin::Mutex;

use crate::service_loader::NativeError;
use crate::task::process::ProcessId;

pub const MAX_MSG_BYTES: usize = 256;
pub const MAX_QUEUE_PER_SESSION: usize = 4;

#[derive(Debug, Clone)]
pub struct InterimMessage {
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeError {
    Saturated,
    TooLarge,
    Empty,
    Retired,
}

static IPC_BRIDGE_COMPAT_INTERNAL: AtomicU64 = AtomicU64::new(0);
static BRIDGE_RETIRED: core::sync::atomic::AtomicBool =
    core::sync::atomic::AtomicBool::new(false);
static E00_SATURATIONS: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    static ref SESSION_QUEUES: Mutex<
        alloc::collections::BTreeMap<(ProcessId, u32), VecDeque<InterimMessage>>,
    > = Mutex::new(alloc::collections::BTreeMap::new());
}

/// CI counter — must reach zero by phase 134.
pub fn ipc_bridge_compat_internal_count() -> u64 {
    IPC_BRIDGE_COMPAT_INTERNAL.load(Ordering::Relaxed)
}

pub fn is_retired() -> bool {
    BRIDGE_RETIRED.load(Ordering::Acquire)
}

/// Retire compat-internal bridge; reset CI counter to zero (phase 134).
pub fn retire_bridge() {
    BRIDGE_RETIRED.store(true, Ordering::Release);
    IPC_BRIDGE_COMPAT_INTERNAL.store(0, Ordering::Release);
}

pub fn e00_saturation_count() -> u64 {
    E00_SATURATIONS.load(Ordering::Relaxed)
}

fn touch_bridge() {
    if BRIDGE_RETIRED.load(Ordering::Acquire) {
        return;
    }
    IPC_BRIDGE_COMPAT_INTERNAL.fetch_add(1, Ordering::Relaxed);
}

pub fn send(pid: ProcessId, session_id: u32, payload: &[u8]) -> Result<(), BridgeError> {
    if BRIDGE_RETIRED.load(Ordering::Acquire) {
        return Err(BridgeError::Retired);
    }
    touch_bridge();
    if payload.len() > MAX_MSG_BYTES {
        return Err(BridgeError::TooLarge);
    }
    let key = (pid, session_id);
    let mut queues = SESSION_QUEUES.lock();
    let queue = queues.entry(key).or_default();
    if queue.len() >= MAX_QUEUE_PER_SESSION {
        E00_SATURATIONS.fetch_add(1, Ordering::Relaxed);
        return Err(BridgeError::Saturated);
    }
    queue.push_back(InterimMessage {
        payload: payload.to_vec(),
    });
    Ok(())
}

pub fn recv(pid: ProcessId, session_id: u32) -> Result<InterimMessage, BridgeError> {
    if BRIDGE_RETIRED.load(Ordering::Acquire) {
        return Err(BridgeError::Retired);
    }
    touch_bridge();
    let key = (pid, session_id);
    let mut queues = SESSION_QUEUES.lock();
    let Some(queue) = queues.get_mut(&key) else {
        return Err(BridgeError::Empty);
    };
    queue.pop_front().ok_or(BridgeError::Empty)
}

pub fn map_bridge_error(err: BridgeError) -> NativeError {
    match err {
        BridgeError::Saturated => NativeError::e00_saturated(),
        BridgeError::Retired | BridgeError::TooLarge | BridgeError::Empty => NativeError {
            code: crate::service_loader::ERR_CAP_QUOTA,
            class: crate::service_loader::ErrorClass::StructuralRemediable,
        },
    }
}

pub fn phase_interim_ipc_smoke() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    let before = ipc_bridge_compat_internal_count();
    let s1 = send(pid, 1, b"hello").is_ok();
    let msg = recv(pid, 1).ok();
    let fifo_ok = msg.as_ref().map(|m| m.payload.as_slice() == b"hello").unwrap_or(false);

    for i in 0..MAX_QUEUE_PER_SESSION {
        let _ = send(pid, 2, &[i as u8]);
    }
    let saturated = send(pid, 2, b"x").is_err();

    let after = ipc_bridge_compat_internal_count();
    s1 && fifo_ok && saturated && after > before
}
