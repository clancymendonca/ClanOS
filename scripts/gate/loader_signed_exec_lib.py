#!/usr/bin/env python3
"""Loader exec manifest signing (ADR-0003) — host reference implementation.

CLOSED SIGNED-FIELD SET: see config/loader_signed_exec/WIRE_FORMAT.txt § canonical signed body.
New clan-exec-v1 fields default outside the signed set unless an ADR amendment extends it.

DEV-ONLY SIGNING SEED — NOT A PRODUCTION SECRET
------------------------------------------------
EPOCH460_LOADER_DEV_SEED_DOMAIN is public and deterministic. Anyone with this repo
can derive the Ed25519 private key. Separate from epoch-450 gate corpus seed.
NEVER call signed_elf_lib.canonical_signed_body* from this module.

Canonical signed bytes: config/loader_signed_exec/WIRE_FORMAT.txt
Golden bytes file:      config/loader_signed_exec/canonical_body.utf8
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

MANIFEST_MAGIC = "clan-exec-v1"
TRUST_SIGNED = "system-signed"
PAYLOAD_NAME = "payload.bin"
MANIFEST_NAME = "manifest.toml"

# PUBLIC deterministic domain — loader exec ONLY. Not epoch-450 gate corpus.
EPOCH460_LOADER_DEV_SEED_DOMAIN = b"clanos-epoch460-loader-exec-signing-anchor-v1"


@dataclass(frozen=True)
class TrustAnchor:
    algorithm: str
    public_key_hex: str
    epoch: int


@dataclass(frozen=True)
class ExecManifest:
    name: str
    kind: str
    entry: str
    image: str | None
    requires: str
    digest_hex: str
    trust: str
    owner: str | None
    description: str | None
    signature_hex: str | None


def epoch460_loader_dev_private_key() -> Ed25519PrivateKey:
    seed = hashlib.sha256(EPOCH460_LOADER_DEV_SEED_DOMAIN).digest()
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


def builtin_alias_digest_payload(name: str, entry: str) -> bytes:
    """Digest input for kind=builtin-alias (seed migration)."""
    return f"clan-builtin-alias-v1\0{name}\0{entry}\0".encode("utf-8")


def digest_payload_for_manifest(manifest: ExecManifest) -> bytes:
    if manifest.kind == "builtin-alias":
        return builtin_alias_digest_payload(manifest.name, manifest.entry)
    raise ValueError("elf64-image requires explicit ELF byte payload")


def canonical_signed_body(manifest: ExecManifest) -> str:
    """Return canonical signed body string (UTF-8, LF-only). See WIRE_FORMAT.txt."""
    if manifest.trust != TRUST_SIGNED:
        raise ValueError(f"signed path requires trust={TRUST_SIGNED}")
    if manifest.requires != "execute":
        raise ValueError("signed path requires requires=execute")
    digest_hex = manifest.digest_hex.lower()
    if len(digest_hex) != 64 or not re.fullmatch(r"[0-9a-f]+", digest_hex):
        raise ValueError("invalid digest hex")
    if manifest.kind not in ("builtin-alias", "elf64-image"):
        raise ValueError(f"unsupported kind: {manifest.kind!r}")

    lines = [
        MANIFEST_MAGIC,
        f"name={manifest.name}",
        f"kind={manifest.kind}",
        f"entry={manifest.entry}",
    ]
    if manifest.kind == "elf64-image":
        if not manifest.image:
            raise ValueError("elf64-image requires image=")
        lines.append(f"image={manifest.image}")
    elif manifest.image:
        raise ValueError("image= only permitted for elf64-image")
    lines.extend(
        [
            "requires=execute",
            f"digest=sha256:{digest_hex}",
            f"trust={TRUST_SIGNED}",
        ]
    )
    return "\n".join(lines) + "\n"


def canonical_signed_body_bytes(manifest: ExecManifest) -> bytes:
    return canonical_signed_body(manifest).encode("utf-8")


def parse_manifest(text: str) -> ExecManifest:
    lines = [ln.rstrip("\r") for ln in text.splitlines()]
    if not lines or lines[0] != MANIFEST_MAGIC:
        raise ValueError("manifest missing clan-exec-v1 header")

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
    kind = fields.get("kind")
    entry = fields.get("entry")
    requires = fields.get("requires", "")
    trust = fields.get("trust", "")
    digest_raw = fields.get("digest", "")
    if not name:
        raise ValueError("manifest missing name")
    if kind not in ("builtin-alias", "elf64-image"):
        raise ValueError("manifest missing or invalid kind")
    if not entry:
        raise ValueError("manifest missing entry")
    if requires != "execute":
        raise ValueError("manifest requires=execute required for signed path")
    if not digest_raw.startswith("sha256:"):
        raise ValueError("manifest digest must be sha256:")
    digest_hex = digest_raw.removeprefix("sha256:").lower()
    if len(digest_hex) != 64 or not re.fullmatch(r"[0-9a-f]+", digest_hex):
        raise ValueError("invalid digest hex")
    if trust != TRUST_SIGNED:
        raise ValueError(f"manifest trust must be {TRUST_SIGNED} for signed verify")

    image = fields.get("image")
    if kind == "elf64-image" and not image:
        raise ValueError("elf64-image requires image=")

    return ExecManifest(
        name=name,
        kind=kind,
        entry=entry,
        image=image,
        requires=requires,
        digest_hex=digest_hex,
        trust=trust,
        owner=fields.get("owner"),
        description=fields.get("description"),
        signature_hex=sig,
    )


def render_manifest(manifest: ExecManifest, signature_hex: str) -> str:
    """Render on-disk manifest including unsigned metadata lines."""
    body_manifest = ExecManifest(
        name=manifest.name,
        kind=manifest.kind,
        entry=manifest.entry,
        image=manifest.image,
        requires=manifest.requires,
        digest_hex=manifest.digest_hex,
        trust=manifest.trust,
        owner=manifest.owner,
        description=manifest.description,
        signature_hex=None,
    )
    lines = canonical_signed_body(body_manifest).rstrip("\n").split("\n")
    if manifest.owner:
        lines.append(f"owner={manifest.owner}")
    if manifest.description:
        lines.append(f"description={manifest.description}")
    lines.append(f"sig=ed25519:{signature_hex.lower()}")
    return "\n".join(lines) + "\n"


def sign_manifest(manifest: ExecManifest, private_key: Ed25519PrivateKey) -> str:
    body = canonical_signed_body(manifest)
    sig = private_key.sign(body.encode("utf-8")).hex()
    return render_manifest(manifest, sig)


def verify_signed_manifest(
    manifest_text: str,
    digest_payload: bytes,
    anchor: TrustAnchor,
) -> tuple[bool, str]:
    try:
        manifest = parse_manifest(manifest_text)
    except ValueError as exc:
        return False, f"manifest parse: {exc}"

    if manifest.signature_hex is None:
        return False, "manifest missing sig=ed25519"

    actual_digest = sha256_hex(digest_payload)
    if actual_digest != manifest.digest_hex:
        return False, "digest payload mismatch"

    signed = ExecManifest(
        name=manifest.name,
        kind=manifest.kind,
        entry=manifest.entry,
        image=manifest.image,
        requires=manifest.requires,
        digest_hex=actual_digest,
        trust=manifest.trust,
        owner=manifest.owner,
        description=manifest.description,
        signature_hex=manifest.signature_hex,
    )
    try:
        pub = Ed25519PublicKey.from_public_bytes(bytes.fromhex(anchor.public_key_hex))
        pub.verify(
            bytes.fromhex(manifest.signature_hex),
            canonical_signed_body_bytes(signed),
        )
    except (ValueError, InvalidSignature):
        return False, "signature verify failed"

    return True, "ok"


def verify_signed_exec(
    payload: bytes,
    manifest_text: str,
    anchor: TrustAnchor,
) -> tuple[bool, str]:
    return verify_signed_manifest(manifest_text, payload, anchor)


def verify_signed_builtin_alias(
    manifest_text: str,
    anchor: TrustAnchor,
) -> tuple[bool, str]:
    try:
        manifest = parse_manifest(manifest_text)
    except ValueError as exc:
        return False, f"manifest parse: {exc}"
    if manifest.kind != "builtin-alias":
        return False, "expected kind=builtin-alias"
    payload = digest_payload_for_manifest(manifest)
    return verify_signed_manifest(manifest_text, payload, anchor)


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
    return verify_signed_exec(payload, manifest_text, anchor)
