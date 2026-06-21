#!/usr/bin/env python3
"""Regenerate scripts/gate/fixtures/loader_signed/ negative + good fixtures."""

from __future__ import annotations

import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CORPUS = ROOT / "config" / "loader_signed_exec"
FIXTURES = Path(__file__).resolve().parent / "fixtures" / "loader_signed"
ANCHOR = ROOT / "config" / "trust_anchor_epoch460_loader.toml"

sys.path.insert(0, str(Path(__file__).resolve().parent))
import loader_signed_exec_lib as lsel  # noqa: E402
import signed_elf_lib as sel  # noqa: E402 — wrong-key fixture only


def _write_dir(path: Path, payload: bytes, manifest_text: str) -> None:
    path.mkdir(parents=True, exist_ok=True)
    (path / lsel.PAYLOAD_NAME).write_bytes(payload)
    (path / lsel.MANIFEST_NAME).write_text(manifest_text, encoding="utf-8")


def main() -> int:
    payload = (CORPUS / lsel.PAYLOAD_NAME).read_bytes()
    good_manifest = (CORPUS / lsel.MANIFEST_NAME).read_text(encoding="utf-8")
    parsed = lsel.parse_manifest(good_manifest)
    digest = lsel.sha256_hex(payload)

    if FIXTURES.exists():
        shutil.rmtree(FIXTURES)
    FIXTURES.mkdir(parents=True)

    _write_dir(FIXTURES / "good", payload, good_manifest)

    # Tampered payload, original manifest digest/sig.
    _write_dir(
        FIXTURES / "tampered_payload",
        payload + b"\x00",
        good_manifest,
    )

    # Wrong signature bytes.
    bad_sig = good_manifest.replace(
        parsed.signature_hex or "",
        "0" * 128,
    )
    _write_dir(FIXTURES / "wrong_sig", payload, bad_sig)

    # Unsigned — strip sig line.
    unsigned_lines = [
        ln
        for ln in good_manifest.splitlines()
        if not ln.startswith("sig=ed25519:")
    ]
    _write_dir(FIXTURES / "unsigned", payload, "\n".join(unsigned_lines) + "\n")

    # Wrong key — signed with epoch-450 gate key (must fail epoch-460 verify).
    body_manifest = lsel.ExecManifest(
        name=parsed.name,
        kind=parsed.kind,
        entry=parsed.entry,
        image=parsed.image,
        requires=parsed.requires,
        digest_hex=digest,
        trust=parsed.trust,
        owner=parsed.owner,
        description=parsed.description,
        signature_hex=None,
    )
    body = lsel.canonical_signed_body(body_manifest)
    wrong_sig = sel.epoch450_dev_private_key().sign(body.encode("utf-8")).hex()
    wrong_key_manifest = lsel.render_manifest(body_manifest, wrong_sig)
    _write_dir(FIXTURES / "wrong_key", payload, wrong_key_manifest)

    # Kind tamper — valid sig for original body but kind changed on disk.
    kind_tampered = good_manifest.replace("kind=elf64-image", "kind=builtin-alias", 1)
    _write_dir(FIXTURES / "tampered_kind", payload, kind_tampered)

    # Entry tamper — dispatch target changed after sign.
    entry_tampered = good_manifest.replace("entry=0x400000", "entry=0x500000", 1)
    _write_dir(FIXTURES / "tampered_entry", payload, entry_tampered)

    anchor = lsel.load_trust_anchor(ANCHOR)
    ok, msg = lsel.verify_corpus_dir(FIXTURES / "good", ANCHOR)
    if not ok:
        print(f"sign_loader_fixtures: FAIL good fixture: {msg}", file=sys.stderr)
        return 1

    negatives = (
        "tampered_payload",
        "wrong_sig",
        "unsigned",
        "wrong_key",
        "tampered_kind",
        "tampered_entry",
    )
    for name in negatives:
        ok, _ = lsel.verify_corpus_dir(FIXTURES / name, ANCHOR)
        if ok:
            print(f"sign_loader_fixtures: FAIL {name} should not verify", file=sys.stderr)
            return 1

    print(f"sign_loader_fixtures: OK wrote {FIXTURES}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
