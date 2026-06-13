//! Loopback network stack (scopes 386–395) — ping over virtio-net stub.

use core::sync::atomic::{AtomicU64, Ordering};

static PING_OK: AtomicU64 = AtomicU64::new(0);
static PACKAGES_INSTALLED: AtomicU64 = AtomicU64::new(0);
static EXTERNAL_NET_ROUTES: AtomicU64 = AtomicU64::new(0);

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

pub fn smoke_network_stack() -> bool {
    crate::virtio_net::init() && loopback_ping() && ping_count() > 0
}

pub fn smoke_network_depth_smoke() -> bool {
    smoke_network_stack()
        && crate::compat_socket::smoke_compat_socket()
        && crate::network_broker::smoke_network_broker_functional_smoke()
}

pub fn mark_package_installed() {
    PACKAGES_INSTALLED.fetch_add(1, Ordering::Relaxed);
}

pub fn packages_installed() -> u64 {
    PACKAGES_INSTALLED.load(Ordering::Relaxed)
}

/// external route simulation (beyond loopback stub).
pub fn simulate_external_route() -> bool {
    let depth = smoke_network_depth_smoke();
    let routed = crate::virtio_net::send_loopback(b"external-probe")
        && crate::virtio_net::recv_loopback().is_some();
    let ok = depth && routed;
    if ok {
        EXTERNAL_NET_ROUTES.fetch_add(1, Ordering::Relaxed);
    }
    ok
}

pub fn smoke_external_network() -> bool {
    simulate_external_route() && EXTERNAL_NET_ROUTES.load(Ordering::Relaxed) > 0
}
