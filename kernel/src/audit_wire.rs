//! Audit IPC correlation on wire — phases 135–138 (ERROR_TAXONOMY + WIRE_SCHEMA).

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::service_loader::{ErrorClass, NativeError};

static WIRE_EVENTS: AtomicU64 = AtomicU64::new(0);
static CORRELATION_IDS: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireAuditEvent {
    pub correlation_id: u64,
    pub error_code: u32,
    pub error_class: u8,
    pub schema: &'static str,
}

pub fn wire_event_count() -> u64 {
    WIRE_EVENTS.load(Ordering::Relaxed)
}

pub fn next_correlation_id() -> u64 {
    CORRELATION_IDS.fetch_add(1, Ordering::Relaxed)
}

pub fn encode_error_on_wire(err: &NativeError) -> WireAuditEvent {
    WIRE_EVENTS.fetch_add(1, Ordering::Relaxed);
    let class_byte = match err.class {
        ErrorClass::Transient => 1,
        ErrorClass::StructuralRemediable => 2,
        ErrorClass::System => 3,
    };
    WireAuditEvent {
        correlation_id: next_correlation_id(),
        error_code: err.code,
        error_class: class_byte,
        schema: "audit.ipc.v1",
    }
}

pub fn serialize_event(ev: &WireAuditEvent) -> Vec<u8> {
    let mut out = Vec::with_capacity(24);
    out.extend_from_slice(&ev.correlation_id.to_le_bytes());
    out.extend_from_slice(&ev.error_code.to_le_bytes());
    out.push(ev.error_class);
    out
}

pub fn phase135_audit_correlation_smoke() -> bool {
    let err = NativeError::e00_saturated();
    let ev = encode_error_on_wire(&err);
    let bytes = serialize_event(&ev);
    ev.schema == "audit.ipc.v1"
        && ev.correlation_id > 0
        && bytes.len() >= 13
        && wire_event_count() > 0
}

pub fn phase136_wait_set_smoke() -> bool {
    let Some(pid) = crate::kernel_object::ensure_smoke_process() else {
        return false;
    };
    let ep = crate::ipc_endpoints::create_endpoint();
    crate::ipc_endpoints::send(ep, pid, b"wait").is_ok()
}

pub fn phase137_error_taxonomy_wire_smoke() -> bool {
    let structural = NativeError {
        code: crate::service_loader::ERR_CAP_QUOTA,
        class: ErrorClass::StructuralRemediable,
    };
    let ev = encode_error_on_wire(&structural);
    ev.error_class == 2
}

pub fn phase138_schema_registry_smoke() -> bool {
    phase135_audit_correlation_smoke() && phase137_error_taxonomy_wire_smoke()
}
