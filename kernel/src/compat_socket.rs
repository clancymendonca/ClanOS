//! Compat TCP/UDP + multi-fd select stub (epoch 4) — COMPAT_SUNSET metric.

use core::sync::atomic::{AtomicU64, Ordering};

static TCP_CONNECTS: AtomicU64 = AtomicU64::new(0);
static UDP_SENDS: AtomicU64 = AtomicU64::new(0);
static SELECT_CALLS: AtomicU64 = AtomicU64::new(0);

pub const COMPAT_SOCKET_SCHEMA: &str = "compat.socket.v1";

pub fn tcp_connect_stub(_host: &str, _port: u16) -> Result<u32, ()> {
    TCP_CONNECTS.fetch_add(1, Ordering::Relaxed);
    Ok(1)
}

pub fn udp_send_stub(_port: u16, _payload: &[u8]) -> Result<(), ()> {
    UDP_SENDS.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

pub fn select_multi_fd_stub(fds: &[u32]) -> usize {
    SELECT_CALLS.fetch_add(1, Ordering::Relaxed);
    fds.first().copied().unwrap_or(0) as usize
}

pub fn compat_socket_calls() -> (u64, u64, u64) {
    (
        TCP_CONNECTS.load(Ordering::Relaxed),
        UDP_SENDS.load(Ordering::Relaxed),
        SELECT_CALLS.load(Ordering::Relaxed),
    )
}

pub fn phase402_compat_socket_smoke() -> bool {
    let tcp = tcp_connect_stub("127.0.0.1", 80).is_ok();
    let udp = udp_send_stub(53, b"ping").is_ok();
    let sel = select_multi_fd_stub(&[1, 2]) > 0;
    let (t, u, s) = compat_socket_calls();
    tcp && udp && sel && t > 0 && u > 0 && s > 0
}
