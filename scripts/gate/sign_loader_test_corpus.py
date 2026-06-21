#!/usr/bin/env python3
"""Regenerate config/loader_signed_exec/ from epoch-460 loader dev anchor (host only).

Self-verify at end is happy-path round-trip only. Negative cases:
scripts/gate/test_loader_signed_exec.py — uses loader_signed_exec_lib only, not signed_elf_lib.
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CORPUS = ROOT / "config" / "loader_signed_exec"
ANCHOR = ROOT / "config" / "trust_anchor_epoch460_loader.toml"

sys.path.insert(0, str(Path(__file__).resolve().parent))
import loader_signed_exec_lib as lsel  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--payload-text",
        default="clan-rt loader-exec-gate-corpus-v1\n",
        help="Payload bytes written to payload.bin",
    )
    ap.add_argument("--name", default="loader-gate-test")
    ap.add_argument("--image", default="/bin/loader-gate-test.elf")
    args = ap.parse_args()

    payload = args.payload_text.encode("utf-8")
    digest = lsel.sha256_hex(payload)
    manifest = lsel.ExecManifest(
        name=args.name,
        kind="elf64-image",
        entry="0x400000",
        image=args.image,
        requires="execute",
        digest_hex=digest,
        trust=lsel.TRUST_SIGNED,
        owner="admin",
        description="ADR-0003 loader signing gate corpus",
        signature_hex=None,
    )
    sk = lsel.epoch460_loader_dev_private_key()
    manifest_text = lsel.sign_manifest(manifest, sk)

    CORPUS.mkdir(parents=True, exist_ok=True)
    (CORPUS / lsel.PAYLOAD_NAME).write_bytes(payload)
    (CORPUS / lsel.MANIFEST_NAME).write_text(manifest_text, encoding="utf-8")
    signed_fields = lsel.ExecManifest(
        name=manifest.name,
        kind=manifest.kind,
        entry=manifest.entry,
        image=manifest.image,
        requires=manifest.requires,
        digest_hex=digest,
        trust=manifest.trust,
        owner=None,
        description=None,
        signature_hex=None,
    )
    (CORPUS / "canonical_body.utf8").write_bytes(
        lsel.canonical_signed_body_bytes(signed_fields)
    )

    anchor = lsel.load_trust_anchor(ANCHOR)
    ok, msg = lsel.verify_signed_exec(payload, manifest_text, anchor)
    if not ok:
        print(f"sign_loader_test_corpus: FAIL happy-path self-verify: {msg}", file=sys.stderr)
        return 1
    print(
        f"sign_loader_test_corpus: OK wrote {CORPUS} (digest=sha256:{digest}; "
        "happy-path self-verify only — see test_loader_signed_exec.py for negatives)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
