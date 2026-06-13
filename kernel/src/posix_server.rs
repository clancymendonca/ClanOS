//! POSIX compatibility server skeleton (out-of-kernel service contract).
//!
//! Wire format: `posix.compat.v1` — opcode byte + payload. Compat clients send
//! requests to the server endpoint; dispatch runs synchronously until a ring-3
//! server binary replaces the in-kernel pump (`// STUB(scope-408):` userspace server).

use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use lazy_static::lazy_static;
use spin::Mutex;

use crate::ipc_endpoints::{self, EndpointId};
use crate::service_loader;
use crate::task::process::{self, ProcessId, ProcessMode};

pub const POSIX_COMPAT_SCHEMA: &str = "posix.compat.v1";

const OP_GETPID: u8 = 0x01;
const OP_OPEN: u8 = 0x02;
const OP_RESP_OK: u8 = 0x80;
const OP_RESP_ERR: u8 = 0xFF;

#[derive(Debug, Clone, Copy)]
struct PosixServerState {
    service_pid: ProcessId,
    endpoint: EndpointId,
    service_cap: u32, // retained for future cap-delegation wiring
}

lazy_static! {
    static ref SERVER: Mutex<Option<PosixServerState>> = Mutex::new(None);
}

static REQUESTS_HANDLED: AtomicU64 = AtomicU64::new(0);

pub fn posix_server_request_count() -> u64 {
    REQUESTS_HANDLED.load(Ordering::Relaxed)
}

pub fn server_endpoint() -> Option<EndpointId> {
    SERVER.lock().as_ref().map(|s| s.endpoint)
}

pub fn service_pid() -> Option<ProcessId> {
    SERVER.lock().as_ref().map(|s| s.service_pid)
}

/// Register native POSIX server process + IPC endpoint (first boot / smoke).
pub fn ensure_posix_server() -> Result<EndpointId, ()> {
    if let Some(ep) = server_endpoint() {
        return Ok(ep);
    }

    let Some(pid) = process::create_process_for_smoke("posix-server") else {
        return Err(());
    };
    process::set_process_mode(pid, ProcessMode::Native);
    let cap = service_loader::load_service_with_stubs(pid, 4096).map_err(|_| ())?;
    let endpoint = ipc_endpoints::create_endpoint();
    *SERVER.lock() = Some(PosixServerState {
        service_pid: pid,
        endpoint,
        service_cap: cap,
    });
    Ok(endpoint)
}

fn dispatch_request(client: ProcessId, request: &[u8]) -> Vec<u8> {
    match request.first() {
        Some(&OP_GETPID) => {
            let mut resp = vec![OP_RESP_OK];
            resp.extend_from_slice(&client.as_u64().to_le_bytes());
            resp
        }
        Some(&OP_OPEN) => {
            let path = core::str::from_utf8(&request[1..]).unwrap_or("");
            if path.is_empty() {
                return vec![OP_RESP_ERR];
            }
            match crate::fd_table::open_file_for_process(client, path) {
                Ok(fd) => {
                    let mut resp = vec![OP_RESP_OK];
                    resp.extend_from_slice(&fd.to_le_bytes());
                    resp
                }
                Err(()) => vec![OP_RESP_ERR],
            }
        }
        _ => vec![OP_RESP_ERR],
    }
}

/// Compat client round-trip through the POSIX server endpoint.
pub fn invoke_compat(client: ProcessId, request: &[u8]) -> Result<Vec<u8>, ()> {
    if process::process_mode(client) != ProcessMode::Compat {
        return Err(());
    }
    let endpoint = ensure_posix_server()?;
    ipc_endpoints::send(endpoint, client, request).map_err(|_| ())?;
    let msg = ipc_endpoints::recv(endpoint).map_err(|_| ())?;
    if msg.sender != client || msg.payload != request {
        return Err(());
    }
    REQUESTS_HANDLED.fetch_add(1, Ordering::Relaxed);
    Ok(dispatch_request(client, &msg.payload))
}

pub fn smoke_posix_server() -> bool {
    let Some(client) = process::create_process_for_smoke("posix-client") else {
        return false;
    };
    process::set_process_mode(client, ProcessMode::Compat);

    let endpoint_ok = ensure_posix_server().is_ok();
    let native_server = service_pid()
        .map(|pid| process::process_mode(pid) == ProcessMode::Native)
        .unwrap_or(false);

    let getpid_req = [OP_GETPID];
    let getpid_resp = invoke_compat(client, &getpid_req).ok();
    let getpid_ok = getpid_resp
        .as_ref()
        .map(|r| {
            r.len() >= 9
                && r[0] == OP_RESP_OK
                && u64::from_le_bytes(r[1..9].try_into().unwrap_or([0; 8])) == client.as_u64()
        })
        .unwrap_or(false);

    let mut open_req = vec![OP_OPEN];
    open_req.extend_from_slice(b"/bin/hello");
    let open_resp = invoke_compat(client, &open_req).ok();
    let open_ok = open_resp
        .as_ref()
        .map(|r| r.len() >= 5 && r[0] == OP_RESP_OK && u32::from_le_bytes(r[1..5].try_into().unwrap_or([0; 4])) > 0)
        .unwrap_or(false);

    let Some(native_client) = process::create_process_for_smoke("posix-native-client") else {
        return false;
    };
    process::set_process_mode(native_client, ProcessMode::Native);
    let native_client_rejected = invoke_compat(native_client, &getpid_req).is_err();

    let handled = posix_server_request_count() >= 2;

    let cap_minted = SERVER
        .lock()
        .as_ref()
        .map(|s| s.service_cap > 0)
        .unwrap_or(false);

    endpoint_ok
        && native_server
        && cap_minted
        && getpid_ok
        && open_ok
        && native_client_rejected
        && handled
        && server_endpoint().is_some()
}
