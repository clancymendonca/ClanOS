//! Post-100 constitutional foundation (phase 110) and capability milestone (phase 120).

/// Phases 101-110 documentation ratified; gates G1-G5 defined in docs/AXIOMS.md.
pub const CONSTITUTIONAL_FOUNDATION_RATIFIED: bool = true;

/// Compat syscall surface frozen as ares-abi-v1 (docs/ABI_SYSCALL.md).
pub const ARE_ABI_V1: bool = true;

/// Native semantic laws draft ratified as ares-semantics-v1 (docs/ABI_STABILITY.md).
pub const ARE_SEMANTICS_V1: bool = true;

/// Reserved native syscall ID range base (docs/ABI_SYSCALL.md).
pub const NATIVE_SYSCALL_ID_BASE: u64 = 256;

/// Phase 110 decision: immutable ObjectId + generation invalidation.
pub const IMMUTABLE_OBJECT_IDENTITY: bool = true;

/// Returns true when constitutional foundation constants and HW allowlist are consistent.
pub fn phase110_constitutional_smoke() -> bool {
    CONSTITUTIONAL_FOUNDATION_RATIFIED
        && ARE_ABI_V1
        && ARE_SEMANTICS_V1
        && IMMUTABLE_OBJECT_IDENTITY
        && !crate::user_syscall_hw::ALLOWED_HW_SYSCALLS.is_empty()
        && crate::user_syscall_hw::ALLOWED_HW_SYSCALLS.len() >= 24
}

pub fn phase120_cap_compat_smoke() -> bool {
    CONSTITUTIONAL_FOUNDATION_RATIFIED
        && crate::kernel_object::phase111_kernel_object_smoke()
        && crate::kernel_object::phase112_cap_lifecycle_smoke()
        && crate::kernel_object::phase113_rights_smoke()
        && crate::kernel_object::phase114_storage_grant_smoke()
        && crate::path_broker::phase115_path_broker_smoke()
        && crate::kernel_object::phase116_ambient_deny_smoke()
        && crate::kernel_object::phase117_namespace_smoke()
        && crate::storage_broker::phase118_broker_mint_smoke()
        && crate::kernel_object::phase119_compat_bridge_smoke()
}

pub fn status() -> (bool, bool, bool, bool) {
    (
        ARE_ABI_V1,
        ARE_SEMANTICS_V1,
        IMMUTABLE_OBJECT_IDENTITY,
        phase110_constitutional_smoke(),
    )
}

pub fn phase121_service_loader_smoke() -> bool {
    crate::service_loader::phase121_service_loader_smoke()
}

pub fn phase122_storage_broker_smoke() -> bool {
    crate::storage_broker::phase122_storage_broker_smoke()
}

pub fn phase123_permission_broker_smoke() -> bool {
    crate::permission_broker::phase123_permission_broker_smoke()
}

pub fn phase124_device_broker_smoke() -> bool {
    crate::device_broker::phase124_device_broker_smoke()
}

pub fn phase125_network_broker_smoke() -> bool {
    crate::network_broker::phase125_network_broker_smoke()
}

pub fn phase126_clipboard_broker_smoke() -> bool {
    crate::clipboard_broker::phase126_clipboard_broker_smoke()
}

pub fn phase127_service_isolation_smoke() -> bool {
    crate::service_isolation::phase127_service_isolation_smoke()
}

pub fn phase128_native_manifest_smoke() -> bool {
    crate::native_manifest::phase128_g4_smoke()
}

pub fn phase129_scoped_grants_smoke() -> bool {
    crate::native_manifest::phase129_scoped_grants_smoke()
}

pub fn phase201_virtio_blk_smoke() -> bool {
    crate::virtio_blk::phase201_virtio_blk_smoke()
}

pub fn phase131_build_integrity_smoke() -> bool {
    crate::build_integrity::phase131_image_identity_smoke()
}

pub fn phase132_repro_smoke() -> bool {
    crate::build_integrity::phase132_repro_build_smoke()
}

pub fn phase133_rollback_smoke() -> bool {
    crate::build_integrity::phase133_rollback_smoke()
}

pub fn phase134_endpoint_smoke() -> bool {
    crate::ipc_endpoints::phase134_endpoint_smoke()
}

pub fn phase135_audit_wire_smoke() -> bool {
    crate::audit_wire::phase135_audit_correlation_smoke()
}

pub fn phase136_wait_set_smoke() -> bool {
    crate::audit_wire::phase136_wait_set_smoke()
}

pub fn phase137_error_taxonomy_smoke() -> bool {
    crate::audit_wire::phase137_error_taxonomy_wire_smoke()
}

pub fn phase138_schema_smoke() -> bool {
    crate::audit_wire::phase138_schema_registry_smoke()
}

pub fn phase140_ipc_integration_smoke() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    let ep = crate::ipc_endpoints::create_endpoint();
    let mut ok = true;
    for i in 0..64u8 {
        ok &= crate::ipc_endpoints::send(ep, pid, &[i]).is_ok();
    }
    let bridge_zero = crate::ipc_interim_bridge::ipc_bridge_compat_internal_count() == 0;
    phase131_build_integrity_smoke()
        && phase134_endpoint_smoke()
        && phase135_audit_wire_smoke()
        && phase138_schema_smoke()
        && ok
        && bridge_zero
}

pub fn phase401_virtio_net_smoke() -> bool {
    crate::virtio_net::phase401_virtio_net_smoke()
}

pub fn phase402_compat_socket_smoke() -> bool {
    crate::compat_socket::phase402_compat_socket_smoke()
}

pub fn phase403_network_broker_smoke() -> bool {
    crate::network_broker::phase403_network_broker_functional_smoke()
}

pub fn phase404_network_epoch_smoke() -> bool {
    phase401_virtio_net_smoke() && phase402_compat_socket_smoke() && phase403_network_broker_smoke()
}

pub fn phase141_scheduler_smoke() -> bool {
    crate::service_scheduler::phase141_service_scheduler_smoke()
}

pub fn phase142_smp_smoke() -> bool {
    crate::service_scheduler::phase142_smp_readiness_smoke()
}

pub fn phase145_compositor_smoke() -> bool {
    crate::compositor::phase145_compositor_smoke()
}

pub fn phase147_oom_smoke() -> bool {
    crate::oom_policy::phase147_oom_smoke()
}

pub fn phase149_epoch5_integration_smoke() -> bool {
    phase141_scheduler_smoke()
        && phase142_smp_smoke()
        && phase145_compositor_smoke()
        && phase147_oom_smoke()
}

pub fn phase150_milestone_smoke() -> bool {
    crate::milestone150::phase150_milestone_smoke()
}

pub fn phase130_platform_integration_smoke() -> bool {
    phase121_service_loader_smoke()
        && crate::ipc_interim_bridge::phase_interim_ipc_smoke()
        && phase122_storage_broker_smoke()
        && phase123_permission_broker_smoke()
        && phase124_device_broker_smoke()
        && phase125_network_broker_smoke()
        && phase126_clipboard_broker_smoke()
        && phase127_service_isolation_smoke()
        && phase128_native_manifest_smoke()
        && phase129_scoped_grants_smoke()
        && phase120_cap_compat_smoke()
}

pub fn phase121_status() -> (bool, bool, bool, bool) {
    let (quota_rej, e00_rej, budget_rej, bootstrap_mints) = crate::service_loader::stub_status();
    (
        bootstrap_mints > 0,
        e00_rej > 0,
        budget_rej > 0,
        quota_rej > 0,
    )
}

pub fn phase120_status() -> (bool, bool, bool, bool, bool) {
    let cap_table = crate::kernel_object::phase111_kernel_object_smoke();
    let rights = crate::kernel_object::phase113_rights_smoke();
    let grant = crate::kernel_object::phase114_storage_grant_smoke();
    let broker = crate::storage_broker::phase118_broker_mint_smoke();
    let compat = crate::kernel_object::phase119_compat_bridge_smoke();
    (cap_table, rights, grant, broker, compat)
}
