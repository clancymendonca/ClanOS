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
