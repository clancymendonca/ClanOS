//! Loopback network stack (phases 386–395) — ping over virtio-net stub.

use core::sync::atomic::{AtomicU64, Ordering};

static PING_OK: AtomicU64 = AtomicU64::new(0);
static PACKAGES_INSTALLED: AtomicU64 = AtomicU64::new(0);

pub fn loopback_ping() -> bool {
    let sent = crate::virtio_net::send_loopback(b"ping");
    let reply = crate::virtio_net::recv_loopback();
    let ok = sent && reply.as_deref() == Some(b"pong");
    if ok {
        PING_OK.fetch_add(1, Ordering::Relaxed);
    }
    ok
}

pub fn ping_count() -> u64 {
    PING_OK.load(Ordering::Relaxed)
}

pub fn phase386_network_smoke() -> bool {
    crate::virtio_net::init() && loopback_ping() && ping_count() > 0
}

pub fn phase395_network_depth_smoke() -> bool {
    phase386_network_smoke()
        && crate::compat_socket::phase402_compat_socket_smoke()
        && crate::network_broker::phase403_network_broker_functional_smoke()
}

pub fn mark_package_installed() {
    PACKAGES_INSTALLED.fetch_add(1, Ordering::Relaxed);
}

pub fn packages_installed() -> u64 {
    PACKAGES_INSTALLED.load(Ordering::Relaxed)
}
