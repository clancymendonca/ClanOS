//! Native IPC endpoints (scope 134+) — ABI_IPC.md per-sender FIFO.

use alloc::collections::BTreeMap;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use lazy_static::lazy_static;
use spin::Mutex;

use crate::ipc_interim_bridge;
use crate::task::process::ProcessId;

pub const MAX_ENDPOINT_MSG_BYTES: usize = 256;
pub const MAX_ENDPOINT_QUEUE: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EndpointId(u64);

impl EndpointId {
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct EndpointMessage {
    pub sender: ProcessId,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointError {
    NotFound,
    QueueFull,
    Empty,
    TooLarge,
    BridgeActive,
}

static NEXT_ENDPOINT: AtomicU64 = AtomicU64::new(1);
static ENDPOINT_SENDS: AtomicU64 = AtomicU64::new(0);
static ENDPOINT_RECVS: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    static ref ENDPOINT_QUEUES: Mutex<BTreeMap<EndpointId, VecDeque<EndpointMessage>>> =
        Mutex::new(BTreeMap::new());
}

pub fn endpoint_send_count() -> u64 {
    ENDPOINT_SENDS.load(Ordering::Relaxed)
}

pub fn endpoint_recv_count() -> u64 {
    ENDPOINT_RECVS.load(Ordering::Relaxed)
}

pub fn create_endpoint() -> EndpointId {
    let id = EndpointId(NEXT_ENDPOINT.fetch_add(1, Ordering::Relaxed));
    ENDPOINT_QUEUES.lock().entry(id).or_default();
    id
}

pub fn send(endpoint: EndpointId, sender: ProcessId, payload: &[u8]) -> Result<(), EndpointError> {
    if payload.len() > MAX_ENDPOINT_MSG_BYTES {
        return Err(EndpointError::TooLarge);
    }
    let mut queues = ENDPOINT_QUEUES.lock();
    let queue = queues.get_mut(&endpoint).ok_or(EndpointError::NotFound)?;
    if queue.len() >= MAX_ENDPOINT_QUEUE {
        return Err(EndpointError::QueueFull);
    }
    queue.push_back(EndpointMessage {
        sender,
        payload: payload.to_vec(),
    });
    ENDPOINT_SENDS.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

pub fn recv(endpoint: EndpointId) -> Result<EndpointMessage, EndpointError> {
    let mut queues = ENDPOINT_QUEUES.lock();
    let queue = queues.get_mut(&endpoint).ok_or(EndpointError::NotFound)?;
    let msg = queue.pop_front().ok_or(EndpointError::Empty)?;
    ENDPOINT_RECVS.fetch_add(1, Ordering::Relaxed);
    Ok(msg)
}

/// : retire interim bridge; native endpoints become sole broker IPC path.
pub fn activate_native_endpoints() -> bool {
    if ipc_interim_bridge::ipc_bridge_compat_internal_count() > 0
        && !ipc_interim_bridge::is_retired()
    {
        ipc_interim_bridge::retire_bridge();
    }
    ipc_interim_bridge::ipc_bridge_compat_internal_count() == 0
}

/// P-134 semantic ordering corpus: per-sender FIFO 1,2,3.
pub fn p134_ordering_corpus() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    let ep = create_endpoint();
    for n in 1u8..=3 {
        if send(ep, pid, &[n]).is_err() {
            return false;
        }
    }
    for expected in 1u8..=3 {
        let msg = recv(ep).ok();
        let ok = msg
            .as_ref()
            .map(|m| m.sender == pid && m.payload == [expected])
            .unwrap_or(false);
        if !ok {
            return false;
        }
    }
    true
}

pub fn smoke_ipc_endpoint() -> bool {
    let retired = activate_native_endpoints();
    let counter_zero = ipc_interim_bridge::ipc_bridge_compat_internal_count() == 0;
    let ordering = p134_ordering_corpus();
    retired && counter_zero && ordering
}
