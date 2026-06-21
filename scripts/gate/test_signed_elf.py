#!/usr/bin/env python3
"""Self-test for signed_elf host verifier — negatives must fail before good passes."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LIB_DIR = Path(__file__).resolve().parent
ANCHOR = ROOT / "config" / "trust_anchor_epoch450.toml"
CORPUS = ROOT / "config" / "signed_elf_test_corpus"
FIXTURES = LIB_DIR / "fixtures" / "signed_elf"
HOST = LIB_DIR / "signed_elf.py"


def main() -> int:
    failures = 0
    sys.path.insert(0, str(LIB_DIR))
    import signed_elf_lib as sel  # noqa: E402

    anchor = sel.load_trust_anchor(ANCHOR)

    negative_cases = (
        ("tampered payload", FIXTURES / "tampered_payload"),
        ("wrong signature", FIXTURES / "wrong_sig"),
        ("unsigned manifest", FIXTURES / "unsigned"),
        ("wrong signing key", FIXTURES / "wrong_key"),
    )
    for label, path in negative_cases:
        ok, msg = sel.verify_corpus_dir(path, ANCHOR)
        if ok:
            print(
                f"test_signed_elf: FAIL {label} should not verify (got ok)",
                file=sys.stderr,
            )
            failures += 1
        else:
            print(f"test_signed_elf: OK rejects {label} ({msg})")

    ok, msg = sel.verify_corpus_dir(FIXTURES / "good", ANCHOR)
    if not ok:
        print(f"test_signed_elf: FAIL good fixture: {msg}", file=sys.stderr)
        failures += 1
    else:
        print("test_signed_elf: OK accepts good fixture")

    ok, msg = sel.verify_corpus_dir(CORPUS, ANCHOR)
    if not ok:
        print(f"test_signed_elf: FAIL production corpus: {msg}", file=sys.stderr)
        failures += 1
    else:
        print("test_signed_elf: OK accepts config/signed_elf_test_corpus")

    golden_path = CORPUS / "canonical_body.utf8"
    if not golden_path.is_file():
        print("test_signed_elf: FAIL missing canonical_body.utf8 golden bytes", file=sys.stderr)
        failures += 1
    else:
        payload = (CORPUS / sel.PAYLOAD_NAME).read_bytes()
        manifest_text = (CORPUS / sel.MANIFEST_NAME).read_text(encoding="utf-8")
        parsed = sel.parse_manifest(manifest_text)
        actual_digest = sel.sha256_hex(payload)
        expected = sel.canonical_signed_body_bytes(parsed.name, actual_digest, parsed.trust)
        golden = golden_path.read_bytes()
        if expected != golden:
            print(
                "test_signed_elf: FAIL canonical_body.utf8 mismatch "
                f"(lib={len(expected)} golden={len(golden)})",
                file=sys.stderr,
            )
            failures += 1
        else:
            print("test_signed_elf: OK canonical_body.utf8 matches lib (kernel handoff bytes)")

    # Digest is computed from payload — caller cannot supply expected digest via manifest alone.
    payload = (CORPUS / sel.PAYLOAD_NAME).read_bytes()
    manifest_text = (CORPUS / sel.MANIFEST_NAME).read_text(encoding="utf-8")
    parsed = sel.parse_manifest(manifest_text)
    forged = manifest_text.replace(
        parsed.digest_hex,
        "0" * 64,
    )
    ok, _ = sel.verify_signed_corpus(payload, forged, anchor)
    if ok:
        print("test_signed_elf: FAIL forged digest in manifest should not verify", file=sys.stderr)
        failures += 1
    else:
        print("test_signed_elf: OK rejects forged digest without matching payload")

    proc = subprocess.run([sys.executable, str(HOST)], capture_output=True, text=True)
    if proc.returncode != 0:
        print(f"test_signed_elf: FAIL signed_elf.py: {proc.stderr}", file=sys.stderr)
        failures += 1
    else:
        print("test_signed_elf: OK gate/signed_elf host check")

    if failures:
        print(f"test_signed_elf: {failures} failure(s)", file=sys.stderr)
        return 1
    print("test_signed_elf: OK (all cases)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
