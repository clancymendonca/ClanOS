#!/usr/bin/env python3
"""Signed ELF gate corpus verification (ADR-0002) — host reference implementation.

DEV-ONLY SIGNING SEED — NOT A PRODUCTION SECRET
------------------------------------------------
EPOCH450_DEV_SEED_DOMAIN is public and deterministic. Anyone with this repo can
derive the Ed25519 private key. That is intentional for the pinned gate test
corpus only. NEVER reuse epoch450_dev_private_key(), sign_test_corpus.py, or
this seed path to sign real userland binaries, loader manifests, or production
artifacts. See docs/SECURITY.md § Epoch-450 signed ELF gate corpus.

Canonical signed bytes: config/signed_elf_test_corpus/WIRE_FORMAT.txt
Golden bytes file:      config/signed_elf_test_corpus/canonical_body.utf8
"""

from __future__ import annotations

import hashlib
import re
from dataclasses import dataclass
from pathlib import Path

from cryptography.exceptions import InvalidSignature
from cryptography.hazmat.primitives.asymmetric.ed25519 import (
    Ed25519PrivateKey,
    Ed25519PublicKey,
)

MANIFEST_MAGIC = "clan-signed-manifest-v1"
PAYLOAD_NAME = "payload.bin"
MANIFEST_NAME = "manifest.toml"

# PUBLIC deterministic domain — NOT confidential. Gate test corpus ONLY.
EPOCH450_DEV_SEED_DOMAIN = b"clanos-epoch450-signed-elf-gate-test-anchor-v1"


@dataclass(frozen=True)
class TrustAnchor:
    algorithm: str
    public_key_hex: str
    epoch: int


@dataclass(frozen=True)
class ParsedManifest:
    name: str
    digest_hex: str
    trust: str
    signature_hex: str | None
    raw_lines: list[str]


def epoch450_dev_private_key() -> Ed25519PrivateKey:
    seed = hashlib.sha256(EPOCH450_DEV_SEED_DOMAIN).digest()
    return Ed25519PrivateKey.from_private_bytes(seed)


def load_trust_anchor(path: Path) -> TrustAnchor:
    text = path.read_text(encoding="utf-8")
    algo = _toml_value(text, "algorithm") or ""
    if algo != "ed25519":
        raise ValueError(f"unsupported anchor algorithm: {algo!r}")
    pub = _toml_value(text, "public_key_hex") or ""
    if len(pub) != 64 or not re.fullmatch(r"[0-9a-fA-F]+", pub):
        raise ValueError("invalid public_key_hex in trust anchor")
    epoch_raw = _toml_value(text, "epoch") or "0"
    return TrustAnchor(algorithm=algo, public_key_hex=pub.lower(), epoch=int(epoch_raw))


def _toml_value(text: str, key: str) -> str | None:
    m = re.search(rf'^{re.escape(key)}\s*=\s*"([^"]*)"', text, re.M)
    if m:
        return m.group(1)
    m = re.search(rf"^{re.escape(key)}\s*=\s*(\d+)", text, re.M)
    if m:
        return m.group(1)
    return None


def sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_signed_body(name: str, digest_hex: str, trust: str = "system") -> str:
    """Return canonical signed body string (UTF-8, LF-only). See WIRE_FORMAT.txt."""
    digest_hex = digest_hex.lower()
    if trust != "system":
        raise ValueError("epoch-450 gate corpus requires trust=system")
    return (
        f"{MANIFEST_MAGIC}\n"
        f"name={name}\n"
        f"digest=sha256:{digest_hex}\n"
        f"trust={trust}\n"
    )


def canonical_signed_body_bytes(name: str, digest_hex: str, trust: str = "system") -> bytes:
    """Exact octets Ed25519 verifies. Kernel must match this byte-for-byte."""
    return canonical_signed_body(name, digest_hex, trust).encode("utf-8")


def parse_manifest(text: str) -> ParsedManifest:
    lines = [ln.rstrip("\r") for ln in text.splitlines()]
    if not lines or lines[0] != MANIFEST_MAGIC:
        raise ValueError("manifest missing clan-signed-manifest-v1 header")
    fields: dict[str, str] = {}
    sig: str | None = None
    for line in lines[1:]:
        if not line or line.startswith("#"):
            continue
        if line.startswith("sig=ed25519:"):
            sig = line.split(":", 1)[1].strip().lower()
            continue
        if "=" not in line:
            raise ValueError(f"invalid manifest line: {line!r}")
        key, val = line.split("=", 1)
        fields[key.strip()] = val.strip()
    name = fields.get("name")
    digest_raw = fields.get("digest", "")
    trust = fields.get("trust", "")
    if not name:
        raise ValueError("manifest missing name")
    if not digest_raw.startswith("sha256:"):
        raise ValueError("manifest digest must be sha256:")
    digest_hex = digest_raw.removeprefix("sha256:").lower()
    if len(digest_hex) != 64 or not re.fullmatch(r"[0-9a-f]+", digest_hex):
        raise ValueError("invalid digest hex")
    if trust != "system":
        raise ValueError("manifest trust must be system for epoch-450 gate corpus")
    return ParsedManifest(
        name=name,
        digest_hex=digest_hex,
        trust=trust,
        signature_hex=sig,
        raw_lines=lines,
    )


def sign_manifest_body(body: str, private_key: Ed25519PrivateKey) -> str:
    return private_key.sign(body.encode("utf-8")).hex()


def render_manifest(body: str, signature_hex: str) -> str:
    return body + f"sig=ed25519:{signature_hex.lower()}\n"


def verify_signed_corpus(
    payload: bytes,
    manifest_text: str,
    anchor: TrustAnchor,
) -> tuple[bool, str]:
    """Verify pinned corpus. Never accepts caller-supplied expected digest."""
    try:
        manifest = parse_manifest(manifest_text)
    except ValueError as exc:
        return False, f"manifest parse: {exc}"

    if manifest.signature_hex is None:
        return False, "manifest missing sig=ed25519"

    actual_digest = sha256_hex(payload)
    if actual_digest != manifest.digest_hex:
        return False, "payload digest mismatch"

    body = canonical_signed_body(manifest.name, manifest.digest_hex, manifest.trust)
    try:
        pub = Ed25519PublicKey.from_public_bytes(bytes.fromhex(anchor.public_key_hex))
        pub.verify(
            bytes.fromhex(manifest.signature_hex),
            canonical_signed_body_bytes(manifest.name, manifest.digest_hex, manifest.trust),
        )
    except (ValueError, InvalidSignature):
        return False, "signature verify failed"

    return True, "ok"


def verify_corpus_dir(corpus_dir: Path, anchor_path: Path) -> tuple[bool, str]:
    payload_path = corpus_dir / PAYLOAD_NAME
    manifest_path = corpus_dir / MANIFEST_NAME
    if not payload_path.is_file() or not manifest_path.is_file():
        return False, f"missing {PAYLOAD_NAME} or {MANIFEST_NAME} in {corpus_dir}"
    try:
        anchor = load_trust_anchor(anchor_path)
    except ValueError as exc:
        return False, f"anchor: {exc}"
    payload = payload_path.read_bytes()
    manifest_text = manifest_path.read_text(encoding="utf-8")
    return verify_signed_corpus(payload, manifest_text, anchor)
