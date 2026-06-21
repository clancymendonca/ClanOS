//! ADR-0003 loader exec manifest signing (epoch 460).
//!
//! Wire format: `config/loader_signed_exec/WIRE_FORMAT.txt`
//! Golden signed bytes: `config/loader_signed_exec/canonical_body.utf8`
//!
//! **Closed signed-field set:** only the lines listed in WIRE_FORMAT.txt § canonical signed body
//! are Ed25519-signed. New `clan-exec-v1` fields default outside the signed set; extending the
//! signed set requires ADR amendment, golden-byte regeneration, and kernel/host fixture updates.
//!
//! Do not reuse `signed_elf::canonical_signed_body_bytes` — ADR-0002 uses a different header/body.
//!
//! Uses `ed25519-dalek` 1.x with `sha2/force-soft` for `x86_64-unknown-none`.

use ed25519_dalek::{PublicKey, Signature, Verifier};

use crate::image_digest;

pub const MANIFEST_MAGIC: &str = "clan-exec-v1";
pub const TRUST_SIGNED: &str = "system-signed";

/// Epoch-460 loader trust anchor (`config/trust_anchor_epoch460_loader.toml`).
pub const EPOCH460_LOADER_PUBLIC_KEY: [u8; 32] = [
    0x4b, 0x4f, 0x7b, 0xa3, 0xcc, 0xbb, 0xd8, 0xc3, 0x24, 0x76, 0xf9, 0xb7, 0x15, 0x14, 0x99,
    0x5b, 0x16, 0xab, 0xcc, 0x6e, 0xc5, 0x1c, 0xde, 0xab, 0x71, 0x69, 0xf1, 0x1e, 0x13, 0xc4,
    0x92, 0xeb,
];

const PINNED_PAYLOAD: &[u8] = include_bytes!("../../config/loader_signed_exec/payload.bin");
const PINNED_MANIFEST: &str = include_str!("../../config/loader_signed_exec/manifest.toml");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedExecManifest<'a> {
    pub name: &'a str,
    pub kind: &'a str,
    pub entry: &'a str,
    pub image: Option<&'a str>,
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

pub fn verify_pinned_loader_corpus() -> Result<(), VerifyError> {
    verify_signed_exec(PINNED_PAYLOAD, PINNED_MANIFEST)
}

/// Verify ELF payload + clan-exec-v1 manifest against epoch-460 loader anchor.
pub fn verify_signed_exec(payload: &[u8], manifest_text: &str) -> Result<(), VerifyError> {
    let manifest = parse_manifest(manifest_text)?;
    let signature_hex = manifest.signature_hex.ok_or(VerifyError::MissingSignature)?;

    let actual_digest = image_digest::sha256_hex(payload);
    if actual_digest != manifest.digest_hex {
        return Err(VerifyError::DigestMismatch);
    }

    let body = canonical_signed_body_bytes(&manifest, &actual_digest);
    let signature = decode_hex::<64>(signature_hex).map_err(|_| VerifyError::SignatureInvalid)?;
    let public_key = PublicKey::from_bytes(&EPOCH460_LOADER_PUBLIC_KEY)
        .map_err(|_| VerifyError::SignatureInvalid)?;
    public_key
        .verify(
            &body,
            &Signature::from_bytes(&signature).map_err(|_| VerifyError::SignatureInvalid)?,
        )
        .map_err(|_| VerifyError::SignatureInvalid)
}

pub fn parse_manifest(manifest_text: &str) -> Result<ParsedExecManifest<'_>, VerifyError> {
    let mut lines = manifest_text.split('\n');
    let header = lines
        .next()
        .map(|line| line.trim_end_matches('\r'))
        .ok_or(VerifyError::ManifestParse)?;
    if header != MANIFEST_MAGIC {
        return Err(VerifyError::ManifestParse);
    }

    let mut name: Option<&str> = None;
    let mut kind: Option<&str> = None;
    let mut entry: Option<&str> = None;
    let mut image: Option<&str> = None;
    let mut requires: Option<&str> = None;
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
            "name" if !val.is_empty() => name = Some(val.trim()),
            "kind" if val == "builtin-alias" || val == "elf64-image" => kind = Some(val.trim()),
            "kind" => return Err(VerifyError::ManifestParse),
            "entry" if !val.is_empty() => entry = Some(val.trim()),
            "image" if !val.is_empty() => image = Some(val.trim()),
            "requires" if val == "execute" => requires = Some(val.trim()),
            "requires" => return Err(VerifyError::ManifestParse),
            "digest" => {
                let Some(hex) = val.trim().strip_prefix("sha256:") else {
                    return Err(VerifyError::ManifestParse);
                };
                if hex.len() != 64 || !hex.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
                    return Err(VerifyError::ManifestParse);
                }
                digest_hex = Some(hex);
            }
            "trust" if val == TRUST_SIGNED => trust = Some(val.trim()),
            "trust" => return Err(VerifyError::ManifestParse),
            "owner" | "description" => {}
            _ => return Err(VerifyError::ManifestParse),
        }
    }

    let name = name.ok_or(VerifyError::ManifestParse)?;
    let kind = kind.ok_or(VerifyError::ManifestParse)?;
    let entry = entry.ok_or(VerifyError::ManifestParse)?;
    if requires != Some("execute") {
        return Err(VerifyError::ManifestParse);
    }
    let digest_hex = digest_hex.ok_or(VerifyError::ManifestParse)?;
    let trust = trust.ok_or(VerifyError::ManifestParse)?;
    if kind == "elf64-image" && image.is_none() {
        return Err(VerifyError::ManifestParse);
    }

    Ok(ParsedExecManifest {
        name,
        kind,
        entry,
        image,
        digest_hex,
        trust,
        signature_hex,
    })
}

pub fn canonical_signed_body_bytes(
    manifest: &ParsedExecManifest<'_>,
    digest_hex: &str,
) -> alloc::vec::Vec<u8> {
    let mut out = alloc::format!(
        "{MANIFEST_MAGIC}\nname={}\nkind={}\nentry={}\n",
        manifest.name, manifest.kind, manifest.entry
    );
    if manifest.kind == "elf64-image" {
        if let Some(image) = manifest.image {
            out.push_str(&alloc::format!("image={image}\n"));
        }
    }
    out.push_str(&alloc::format!(
        "requires=execute\ndigest=sha256:{digest_hex}\ntrust={TRUST_SIGNED}\n"
    ));
    out.into_bytes()
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
