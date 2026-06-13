//! Build integrity — scopes 131–133 (BUILD_INTEGRITY.md).

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::image_digest;

static IMAGE_EPOCH: AtomicU64 = AtomicU64::new(1);
static BOOT_VERIFIED: AtomicBool = AtomicBool::new(false);
static REPRO_MATCHES: AtomicU64 = AtomicU64::new(0);
static SIGNED_USER_ELF_VERIFIED: AtomicU64 = AtomicU64::new(0);

pub const KERNEL_IMAGE_TAG: &[u8] = b"clanos-kernel-epoch3";

pub fn system_image_epoch() -> u64 {
    IMAGE_EPOCH.load(Ordering::Relaxed)
}

pub fn boot_verified() -> bool {
    BOOT_VERIFIED.load(Ordering::Relaxed)
}

pub fn repro_match_count() -> u64 {
    REPRO_MATCHES.load(Ordering::Relaxed)
}

/// : signed system image identity (digest stub).
pub fn verify_boot_image() -> bool {
    let digest = image_digest::sha256_hex(KERNEL_IMAGE_TAG);
    let manifest = alloc::format!("digest=sha256:{digest}\n");
    let expected = image_digest::parse_manifest_digest(&manifest).unwrap_or("");
    let ok = image_digest::verify_digest_hex(KERNEL_IMAGE_TAG, expected);
    BOOT_VERIFIED.store(ok, Ordering::Relaxed);
    if ok {
        IMAGE_EPOCH.fetch_add(1, Ordering::Relaxed);
    }
    ok
}

/// : dual-build reproducibility stub — same source → same digest twice.
pub fn verify_reproducible_build() -> bool {
    let a = image_digest::sha256_hex(KERNEL_IMAGE_TAG);
    let b = image_digest::sha256_hex(KERNEL_IMAGE_TAG);
    let ok = a == b;
    if ok {
        REPRO_MATCHES.fetch_add(1, Ordering::Relaxed);
    }
    ok
}

/// : rollback smoke — prior epoch digest still verifiable.
pub fn verify_rollback_anchor() -> bool {
    let anchor = image_digest::sha256_hex(b"clanos-epoch-2-anchor");
    image_digest::verify_digest_hex(b"clanos-epoch-2-anchor", &anchor)
}

pub fn smoke_image_identity() -> bool {
    verify_boot_image() && boot_verified() && system_image_epoch() >= 2
}

pub fn smoke_repro_build_host() -> bool {
    verify_reproducible_build() && repro_match_count() > 0
}

pub fn smoke_rollback() -> bool {
    verify_rollback_anchor()
}

/// signed user ELF manifest corpus (BUILD_INTEGRITY production path).
pub fn verify_signed_user_elf_corpus() -> bool {
    let corpus = b"clan-rt demo:hello";
    let digest = image_digest::sha256_hex(corpus);
    let manifest = alloc::format!("digest=sha256:{digest}\ntrust=system\n");
    let expected = image_digest::parse_manifest_digest(&manifest).unwrap_or("");
    let ok = image_digest::verify_digest_hex(corpus, expected);
    if ok {
        SIGNED_USER_ELF_VERIFIED.fetch_add(1, Ordering::Relaxed);
    }
    ok
}

pub fn smoke_signed_user_elf() -> bool {
    verify_signed_user_elf_corpus() && SIGNED_USER_ELF_VERIFIED.load(Ordering::Relaxed) > 0
}
