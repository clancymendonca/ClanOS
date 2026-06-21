#!/usr/bin/env python3
"""Regenerate config/signed_elf_test_corpus/ from epoch-450 dev anchor (host only).

Self-verify at end is happy-path round-trip only (sign → verify same bytes).
Negative cases are enforced by scripts/gate/test_signed_elf.py, not this script.
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CORPUS = ROOT / "config" / "signed_elf_test_corpus"
ANCHOR = ROOT / "config" / "trust_anchor_epoch450.toml"

sys.path.insert(0, str(Path(__file__).resolve().parent))
import signed_elf_lib as sel  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--payload-text",
        default="clan-rt demo:hello-gate-corpus-v1\n",
        help="Payload bytes written to payload.bin",
    )
    ap.add_argument(
        "--name",
        default="demo-hello-gate-corpus",
        help="Manifest name= field",
    )
    args = ap.parse_args()

    payload = args.payload_text.encode("utf-8")
    digest = sel.sha256_hex(payload)
    body = sel.canonical_signed_body(args.name, digest)
    sk = sel.epoch450_dev_private_key()
    sig = sel.sign_manifest_body(body, sk)
    manifest = sel.render_manifest(body, sig)

    CORPUS.mkdir(parents=True, exist_ok=True)
    (CORPUS / sel.PAYLOAD_NAME).write_bytes(payload)
    (CORPUS / sel.MANIFEST_NAME).write_text(manifest, encoding="utf-8")
    (CORPUS / "canonical_body.utf8").write_bytes(
        sel.canonical_signed_body_bytes(args.name, digest)
    )

    anchor = sel.load_trust_anchor(ANCHOR)
    ok, msg = sel.verify_signed_corpus(payload, manifest, anchor)
    if not ok:
        print(f"sign_test_corpus: FAIL happy-path self-verify: {msg}", file=sys.stderr)
        return 1
    print(
        f"sign_test_corpus: OK wrote {CORPUS} (digest=sha256:{digest}; "
        "happy-path self-verify only — see test_signed_elf.py for negatives)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
