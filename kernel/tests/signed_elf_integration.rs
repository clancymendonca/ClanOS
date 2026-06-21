//! Kernel signed ELF verifier integration — ADR-0002 negative fixture gauntlet.
//!
//! Exercises the in-kernel Ed25519 path against committed fixture bytes (not host Python).

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kernel::{allocator, hlt_loop, image_digest, signed_elf};
use x86_64::VirtAddr;

entry_point!(main);

fn init_test_kernel(boot_info: &'static BootInfo) {
    kernel::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    kernel::user_paging::init(phys_mem_offset);
    let mut mapper = unsafe { kernel::memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { kernel::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialisation failed");
}

fn main(boot_info: &'static BootInfo) -> ! {
    init_test_kernel(boot_info);
    test_main();
    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

const PROD_PAYLOAD: &[u8] = include_bytes!("../../config/signed_elf_test_corpus/payload.bin");
const PROD_MANIFEST: &str = include_str!("../../config/signed_elf_test_corpus/manifest.toml");
const GOLDEN_BODY: &[u8] = include_bytes!("../../config/signed_elf_test_corpus/canonical_body.utf8");

const GOOD_PAYLOAD: &[u8] = include_bytes!("../../scripts/gate/fixtures/signed_elf/good/payload.bin");
const GOOD_MANIFEST: &str = include_str!("../../scripts/gate/fixtures/signed_elf/good/manifest.toml");

const TAMPERED_PAYLOAD: &[u8] =
    include_bytes!("../../scripts/gate/fixtures/signed_elf/tampered_payload/payload.bin");
const TAMPERED_MANIFEST: &str =
    include_str!("../../scripts/gate/fixtures/signed_elf/tampered_payload/manifest.toml");

const WRONG_SIG_PAYLOAD: &[u8] =
    include_bytes!("../../scripts/gate/fixtures/signed_elf/wrong_sig/payload.bin");
const WRONG_SIG_MANIFEST: &str =
    include_str!("../../scripts/gate/fixtures/signed_elf/wrong_sig/manifest.toml");

const UNSIGNED_MANIFEST: &str =
    include_str!("../../scripts/gate/fixtures/signed_elf/unsigned/manifest.toml");

const WRONG_KEY_PAYLOAD: &[u8] =
    include_bytes!("../../scripts/gate/fixtures/signed_elf/wrong_key/payload.bin");
const WRONG_KEY_MANIFEST: &str =
    include_str!("../../scripts/gate/fixtures/signed_elf/wrong_key/manifest.toml");

fn assert_rejects(payload: &[u8], manifest: &str, expected: signed_elf::VerifyError) {
    match signed_elf::verify_signed_corpus(payload, manifest) {
        Err(err) => assert_eq!(err, expected),
        Ok(()) => panic!("expected reject ({expected:?})"),
    }
}

#[test_case]
fn kernel_accepts_pinned_production_corpus() {
    signed_elf::verify_pinned_gate_corpus().expect("production corpus");
}

#[test_case]
fn kernel_accepts_good_fixture() {
    signed_elf::verify_signed_corpus(GOOD_PAYLOAD, GOOD_MANIFEST).expect("good fixture");
}

#[test_case]
fn kernel_canonical_body_matches_golden_octets() {
    let parsed = signed_elf::parse_manifest(PROD_MANIFEST).expect("parse");
    let digest = image_digest::sha256_hex(PROD_PAYLOAD);
    let body = signed_elf::canonical_signed_body_bytes(parsed.name, &digest, parsed.trust);
    assert_eq!(body.as_slice(), GOLDEN_BODY);
}

#[test_case]
fn kernel_rejects_tampered_payload() {
    assert_rejects(
        TAMPERED_PAYLOAD,
        TAMPERED_MANIFEST,
        signed_elf::VerifyError::DigestMismatch,
    );
}

#[test_case]
fn kernel_rejects_wrong_signature() {
    assert_rejects(
        WRONG_SIG_PAYLOAD,
        WRONG_SIG_MANIFEST,
        signed_elf::VerifyError::SignatureInvalid,
    );
}

#[test_case]
fn kernel_rejects_unsigned_manifest() {
    assert_rejects(PROD_PAYLOAD, UNSIGNED_MANIFEST, signed_elf::VerifyError::MissingSignature);
}

#[test_case]
fn kernel_rejects_wrong_signing_key() {
    assert_rejects(
        WRONG_KEY_PAYLOAD,
        WRONG_KEY_MANIFEST,
        signed_elf::VerifyError::SignatureInvalid,
    );
}

#[test_case]
fn kernel_rejects_forged_digest_in_manifest() {
    let parsed = signed_elf::parse_manifest(PROD_MANIFEST).expect("parse");
    let forged = PROD_MANIFEST.replace(parsed.digest_hex, &"0".repeat(64));
    assert_rejects(PROD_PAYLOAD, &forged, signed_elf::VerifyError::DigestMismatch);
}

#[test_case]
fn build_integrity_smoke_uses_kernel_verifier() {
    assert!(kernel::build_integrity::verify_signed_user_elf_corpus());
    assert!(kernel::build_integrity::smoke_signed_user_elf());
}
