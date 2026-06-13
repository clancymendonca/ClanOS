//! Post-100 constitutional foundation (scope 110) and capability milestone (scope 120).

/// Constitutional documentation ratified; gates G1-G5 defined in docs/AXIOMS.md.
pub const CONSTITUTIONAL_FOUNDATION_RATIFIED: bool = true;

/// Compat syscall surface frozen as ares-abi-v1 (docs/ABI_SYSCALL.md).
pub const ARE_ABI_V1: bool = true;

/// Native semantic laws draft ratified as ares-semantics-v1 (docs/ABI_STABILITY.md).
pub const ARE_SEMANTICS_V1: bool = true;

/// Reserved native syscall ID range base (docs/ABI_SYSCALL.md).
pub const NATIVE_SYSCALL_ID_BASE: u64 = 256;

/// decision: immutable ObjectId + generation invalidation.
pub const IMMUTABLE_OBJECT_IDENTITY: bool = true;

/// Returns true when constitutional foundation constants and HW allowlist are consistent.
pub fn smoke_constitutional() -> bool {
    CONSTITUTIONAL_FOUNDATION_RATIFIED
        && ARE_ABI_V1
        && ARE_SEMANTICS_V1
        && IMMUTABLE_OBJECT_IDENTITY
        && !crate::user_syscall_hw::ALLOWED_HW_SYSCALLS.is_empty()
        && crate::user_syscall_hw::ALLOWED_HW_SYSCALLS.len() >= 24
}

pub fn smoke_cap_compat() -> bool {
    CONSTITUTIONAL_FOUNDATION_RATIFIED
        && crate::kernel_object::smoke_kernel_object_smoke()
        && crate::kernel_object::smoke_cap_lifecycle_smoke()
        && crate::kernel_object::smoke_rights_smoke()
        && crate::kernel_object::smoke_storage_grant_smoke()
        && crate::path_broker::smoke_path_broker_smoke()
        && crate::kernel_object::smoke_ambient_deny_smoke()
        && crate::kernel_object::smoke_namespace_smoke()
        && crate::storage_broker::smoke_broker_mint_smoke()
        && crate::kernel_object::smoke_compat_bridge_smoke()
}

pub fn status() -> (bool, bool, bool, bool) {
    (
        ARE_ABI_V1,
        ARE_SEMANTICS_V1,
        IMMUTABLE_OBJECT_IDENTITY,
        smoke_constitutional(),
    )
}

pub fn smoke_service_loader_init() -> bool {
    crate::service_loader::smoke_service_loader_init()
}

pub fn smoke_storage_broker() -> bool {
    crate::storage_broker::smoke_storage_broker()
}

pub fn smoke_permission_broker() -> bool {
    crate::permission_broker::smoke_permission_broker()
}

pub fn smoke_device_broker() -> bool {
    crate::device_broker::smoke_device_broker()
}

pub fn smoke_network_broker() -> bool {
    crate::network_broker::smoke_network_broker()
}

pub fn smoke_clipboard_broker() -> bool {
    crate::clipboard_broker::smoke_clipboard_broker()
}

pub fn smoke_service_isolation() -> bool {
    crate::service_isolation::smoke_service_isolation()
}

pub fn smoke_native_manifest() -> bool {
    crate::native_manifest::smoke_g4_smoke()
}

pub fn smoke_scoped_grants() -> bool {
    crate::native_manifest::smoke_scoped_grants()
}

pub fn smoke_virtio_blk() -> bool {
    crate::virtio_blk::smoke_virtio_blk()
}

pub fn smoke_build_integrity() -> bool {
    crate::build_integrity::smoke_image_identity()
}

pub fn smoke_repro_build() -> bool {
    crate::build_integrity::smoke_repro_build_host()
}

pub fn smoke_rollback() -> bool {
    crate::build_integrity::smoke_rollback()
}

pub fn smoke_ipc_endpoint() -> bool {
    crate::ipc_endpoints::smoke_ipc_endpoint()
}

pub fn smoke_audit_wire() -> bool {
    crate::audit_wire::smoke_audit_correlation_smoke()
}

pub fn smoke_wait_set() -> bool {
    crate::audit_wire::smoke_wait_set()
}

pub fn smoke_error_taxonomy() -> bool {
    crate::audit_wire::smoke_error_taxonomy_wire_smoke()
}

pub fn smoke_schema() -> bool {
    crate::audit_wire::smoke_schema_registry_smoke()
}

pub fn smoke_ipc_integration() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    let ep = crate::ipc_endpoints::create_endpoint();
    // 64-message burst through the bounded queue (MAX_ENDPOINT_QUEUE), draining
    // each chunk and verifying per-sender FIFO order. Backpressure (QueueFull on
    // overflow) is the specified behavior, not a failure — see ABI_IPC FIFO spec.
    let mut ok = true;
    let chunk = crate::ipc_endpoints::MAX_ENDPOINT_QUEUE as u8;
    let mut next = 0u8;
    while next < 64 {
        let end = next.saturating_add(chunk).min(64);
        for i in next..end {
            ok &= crate::ipc_endpoints::send(ep, pid, &[i]).is_ok();
        }
        for expected in next..end {
            ok &= crate::ipc_endpoints::recv(ep)
                .map(|m| m.sender == pid && m.payload == [expected])
                .unwrap_or(false);
        }
        next = end;
    }
    // Overflow returns QueueFull (transient class) rather than dropping or panicking.
    for i in 0..chunk {
        let _ = crate::ipc_endpoints::send(ep, pid, &[i]);
    }
    ok &= crate::ipc_endpoints::send(ep, pid, &[0xFF])
        == Err(crate::ipc_endpoints::EndpointError::QueueFull);
    while crate::ipc_endpoints::recv(ep).is_ok() {}
    let bridge_zero = crate::ipc_interim_bridge::ipc_bridge_compat_internal_count() == 0;
    smoke_build_integrity()
        && smoke_ipc_endpoint()
        && smoke_audit_wire()
        && smoke_schema()
        && ok
        && bridge_zero
}

pub fn smoke_virtio_net() -> bool {
    crate::virtio_net::smoke_virtio_net()
}

pub fn smoke_compat_socket() -> bool {
    crate::compat_socket::smoke_compat_socket()
}

pub fn smoke_network_broker_epoch() -> bool {
    crate::network_broker::smoke_network_broker_functional_smoke()
}

pub fn smoke_network_epoch() -> bool {
    smoke_virtio_net() && smoke_compat_socket() && smoke_network_broker_epoch()
}

pub fn smoke_scheduler_smoke() -> bool {
    crate::service_scheduler::smoke_service_scheduler()
}

pub fn smoke_service_smp() -> bool {
    crate::service_scheduler::smoke_smp_readiness_smoke()
}

pub fn smoke_compositor() -> bool {
    crate::compositor::smoke_compositor()
}

pub fn smoke_oom_policy() -> bool {
    crate::oom_policy::smoke_oom_policy()
}

pub fn smoke_scheduler_epoch_integration() -> bool {
    smoke_scheduler_smoke()
        && smoke_service_smp()
        && smoke_compositor()
        && smoke_oom_policy()
}

pub fn smoke_milestone_boundary() -> bool {
    crate::milestone150::smoke_milestone_boundary()
}

pub fn smoke_epoch7_integrity() -> bool {
    crate::system_gate::integrity_gate()
}

pub fn smoke_scheduling_milestone() -> bool {
    crate::system_gate::scheduling_gate()
}

pub fn smoke_hardware_milestone() -> bool {
    crate::system_gate::hardware_gate()
}

pub fn smoke_federation_milestone() -> bool {
    crate::system_gate::federation_gate()
}

pub fn smoke_release_milestone() -> bool {
    crate::system_gate::release_gate()
}

/// Final release compat — compat sunset + build integrity + functional OS.
pub fn smoke_release_final() -> bool {
    crate::system_gate::release_compat_smoke()
}

pub fn smoke_platform_integration() -> bool {
    smoke_service_loader_init()
        && crate::ipc_interim_bridge::smoke_interim_ipc()
        && smoke_storage_broker()
        && smoke_permission_broker()
        && smoke_device_broker()
        && smoke_network_broker()
        && smoke_clipboard_broker()
        && smoke_service_isolation()
        && smoke_native_manifest()
        && smoke_scoped_grants()
        && smoke_cap_compat()
}

pub fn service_loader_status() -> (bool, bool, bool, bool) {
    let (quota_rej, e00_rej, budget_rej, bootstrap_mints) = crate::service_loader::stub_status();
    (
        bootstrap_mints > 0,
        e00_rej > 0,
        budget_rej > 0,
        quota_rej > 0,
    )
}

pub fn cap_compat_status() -> (bool, bool, bool, bool, bool) {
    let cap_table = crate::kernel_object::smoke_kernel_object_smoke();
    let rights = crate::kernel_object::smoke_rights_smoke();
    let grant = crate::kernel_object::smoke_storage_grant_smoke();
    let broker = crate::storage_broker::smoke_broker_mint_smoke();
    let compat = crate::kernel_object::smoke_compat_bridge_smoke();
    (cap_table, rights, grant, broker, compat)
}
