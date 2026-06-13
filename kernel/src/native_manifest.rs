//! `clan-native-v1` manifest validation (scopes 128–129, G4).

use crate::kernel_object::Rights;

#[derive(Debug, Clone, Copy)]
pub struct ScopedGrant {
    pub kind_tag: &'static str,
    pub rights_mask: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct NativeManifestV1 {
    pub name: &'static str,
    pub grants: &'static [ScopedGrant],
}

impl NativeManifestV1 {
    pub fn validate(&self) -> bool {
        !self.name.is_empty() && !self.grants.is_empty()
    }

    pub fn grants_within(&self, allowed: Rights) -> bool {
        self.grants.iter().all(|g| {
            let r = Rights(g.rights_mask);
            allowed.contains(r)
        })
    }
}

static MANIFEST_LOADS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);
static MANIFEST_REJECTIONS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

pub fn manifest_load_count() -> u64 {
    MANIFEST_LOADS.load(core::sync::atomic::Ordering::Relaxed)
}

pub fn manifest_rejection_count() -> u64 {
    MANIFEST_REJECTIONS.load(core::sync::atomic::Ordering::Relaxed)
}

/// V-01: manifest caps must be subset of broker-granted rights.
pub fn load_native_manifest(manifest: &NativeManifestV1, broker_rights: Rights) -> bool {
    if !manifest.validate() || !manifest.grants_within(broker_rights) {
        MANIFEST_REJECTIONS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        return false;
    }
    MANIFEST_LOADS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    true
}

pub const DEMO_MANIFEST: NativeManifestV1 = NativeManifestV1 {
    name: "demo-service",
    grants: &[ScopedGrant {
        kind_tag: "kernel.service",
        rights_mask: Rights::READ | Rights::WRITE,
    }],
};

pub fn smoke_g4_smoke() -> bool {
    let ok = load_native_manifest(&DEMO_MANIFEST, Rights::read_write());
    let bad = !load_native_manifest(
        &NativeManifestV1 {
            name: "bad",
            grants: &[ScopedGrant {
                kind_tag: "kernel.service",
                rights_mask: Rights::READ | Rights::WRITE | Rights::REVOKE,
            }],
        },
        Rights(Rights::READ),
    );
    ok && bad && manifest_load_count() > 0 && manifest_rejection_count() > 0
}

pub fn smoke_scoped_grants() -> bool {
    let manifest = NativeManifestV1 {
        name: "scoped",
        grants: &[
            ScopedGrant {
                kind_tag: "kernel.endpoint",
                rights_mask: Rights::READ,
            },
            ScopedGrant {
                kind_tag: "device.block",
                rights_mask: Rights::READ | Rights::MAP,
            },
        ],
    };
    load_native_manifest(&manifest, Rights::all_for_smoke())
}
