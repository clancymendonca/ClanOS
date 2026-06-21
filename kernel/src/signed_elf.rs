//! ADR-0002 signed ELF gate corpus verification (epoch 450).
//!
//! Wire format: `config/signed_elf_test_corpus/WIRE_FORMAT.txt`
//! Golden signed bytes: `config/signed_elf_test_corpus/canonical_body.utf8`
//!
//! Uses `ed25519-dalek` 1.x with `sha2/force-soft` for `x86_64-unknown-none` (no CPU SHA extensions).

use ed25519_dalek::{PublicKey, Signature, Verifier};

use crate::image_digest;

pub const MANIFEST_MAGIC: &str = "clan-signed-manifest-v1";

/// Epoch-450 trust anchor public key (`config/trust_anchor_epoch450.toml`).
pub const EPOCH450_PUBLIC_KEY: [u8; 32] = [
    0xd0, 0xde, 0x5d, 0x01, 0xad, 0x51, 0x1b, 0xd1, 0x0c, 0x6b, 0xe1, 0xb8, 0xb4, 0xfe, 0x71,
    0xe9, 0xac, 0x9e, 0x4f, 0x3f, 0x4b, 0x74, 0x0f, 0xde, 0x8d, 0x61, 0xaf, 0x58, 0xb1, 0x1f,
    0xc8, 0x25,
];

const PINNED_PAYLOAD: &[u8] = include_bytes!("../../config/signed_elf_test_corpus/payload.bin");
const PINNED_MANIFEST: &str = include_str!("../../config/signed_elf_test_corpus/manifest.toml");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedManifest<'a> {
    pub name: &'a str,
    pub digest_hex: &'a str,
    pub trust: &'a str,
    pub signature_hex: Option<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyError {
    ManifestParse,
    MissingSignature,
    DigestMismatch,
    SignatureInvalid,
}

impl VerifyError {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManifestParse => "manifest parse",
            Self::MissingSignature => "manifest missing sig=ed25519",
            Self::DigestMismatch => "payload digest mismatch",
            Self::SignatureInvalid => "signature verify failed",
        }
    }
}

pub fn verify_pinned_gate_corpus() -> Result<(), VerifyError> {
    verify_signed_corpus(PINNED_PAYLOAD, PINNED_MANIFEST)
}

/// Verify payload + manifest against the epoch-450 embedded trust anchor.
/// Digest is always recomputed from `payload`; manifest digest is never trusted alone.
pub fn verify_signed_corpus(payload: &[u8], manifest_text: &str) -> Result<(), VerifyError> {
    let manifest = parse_manifest(manifest_text)?;
    let signature_hex = manifest.signature_hex.ok_or(VerifyError::MissingSignature)?;

    let actual_digest = image_digest::sha256_hex(payload);
    if actual_digest != manifest.digest_hex {
        return Err(VerifyError::DigestMismatch);
    }

    let body = canonical_signed_body_bytes(manifest.name, &actual_digest, manifest.trust);
    let signature = decode_hex::<64>(signature_hex).map_err(|_| VerifyError::SignatureInvalid)?;
    let public_key =
        PublicKey::from_bytes(&EPOCH450_PUBLIC_KEY).map_err(|_| VerifyError::SignatureInvalid)?;
    public_key
        .verify(
            &body,
            &Signature::from_bytes(&signature).map_err(|_| VerifyError::SignatureInvalid)?,
        )
        .map_err(|_| VerifyError::SignatureInvalid)
}

pub fn parse_manifest(manifest_text: &str) -> Result<ParsedManifest<'_>, VerifyError> {
    let mut lines = manifest_text.split('\n');
    let header = lines
        .next()
        .map(|line| line.trim_end_matches('\r'))
        .ok_or(VerifyError::ManifestParse)?;
    if header != MANIFEST_MAGIC {
        return Err(VerifyError::ManifestParse);
    }

    let mut name: Option<&str> = None;
    let mut digest_hex: Option<&str> = None;
    let mut trust: Option<&str> = None;
    let mut signature_hex: Option<&str> = None;

    for line in lines {
        let line = line.trim_end_matches('\r');
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(hex) = line.strip_prefix("sig=ed25519:") {
            let hex = hex.trim();
            if hex.len() != 128 || !hex.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
                return Err(VerifyError::ManifestParse);
            }
            signature_hex = Some(hex);
            continue;
        }
        let Some((key, val)) = line.split_once('=') else {
            return Err(VerifyError::ManifestParse);
        };
        match key.trim() {
            "name" => name = Some(val.trim()),
            "digest" => {
                let Some(hex) = val.trim().strip_prefix("sha256:") else {
                    return Err(VerifyError::ManifestParse);
                };
                if hex.len() != 64 || !hex.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
                    return Err(VerifyError::ManifestParse);
                }
                digest_hex = Some(hex);
            }
            "trust" => trust = Some(val.trim()),
            _ => return Err(VerifyError::ManifestParse),
        }
    }

    let name = name.ok_or(VerifyError::ManifestParse)?;
    let digest_hex = digest_hex.ok_or(VerifyError::ManifestParse)?;
    let trust = trust.ok_or(VerifyError::ManifestParse)?;
    if trust != "system" {
        return Err(VerifyError::ManifestParse);
    }

    Ok(ParsedManifest {
        name,
        digest_hex,
        trust,
        signature_hex,
    })
}

pub fn canonical_signed_body_bytes(name: &str, digest_hex: &str, trust: &str) -> alloc::vec::Vec<u8> {
    alloc::format!(
        "{MANIFEST_MAGIC}\nname={name}\ndigest=sha256:{digest_hex}\ntrust={trust}\n"
    )
    .into_bytes()
}

fn decode_hex<const N: usize>(hex: &str) -> Result<[u8; N], ()> {
    if hex.len() != N * 2 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(());
    }
    let mut out = [0u8; N];
    for (i, chunk) in hex.as_bytes().chunks_exact(2).enumerate() {
        let hi = hex_nibble(chunk[0])?;
        let lo = hex_nibble(chunk[1])?;
        out[i] = (hi << 4) | lo;
    }
    Ok(out)
}

fn hex_nibble(byte: u8) -> Result<u8, ()> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(()),
    }
}
