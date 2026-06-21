#!/usr/bin/env python3
"""Self-test for loader_signed_exec host verifier — negatives must fail before good passes."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LIB_DIR = Path(__file__).resolve().parent
ANCHOR = ROOT / "config" / "trust_anchor_epoch460_loader.toml"
CORPUS = ROOT / "config" / "loader_signed_exec"
FIXTURES = LIB_DIR / "fixtures" / "loader_signed"
HOST = LIB_DIR / "loader_signed_exec.py"

sys.path.insert(0, str(LIB_DIR))
import loader_signed_exec_lib as lsel  # noqa: E402
import signed_elf_lib as sel  # noqa: E402


def main() -> int:
    failures = 0
    anchor = lsel.load_trust_anchor(ANCHOR)
    payload = (CORPUS / lsel.PAYLOAD_NAME).read_bytes()
    actual_digest = lsel.sha256_hex(payload)

    negative_cases = (
        ("tampered payload", FIXTURES / "tampered_payload"),
        ("wrong signature", FIXTURES / "wrong_sig"),
        ("unsigned manifest", FIXTURES / "unsigned"),
        ("wrong signing key", FIXTURES / "wrong_key"),
        ("tampered kind", FIXTURES / "tampered_kind"),
        ("tampered entry", FIXTURES / "tampered_entry"),
    )
    for label, path in negative_cases:
        ok, msg = lsel.verify_corpus_dir(path, ANCHOR)
        if ok:
            print(
                f"test_loader_signed_exec: FAIL {label} should not verify (got ok)",
                file=sys.stderr,
            )
            failures += 1
        else:
            print(f"test_loader_signed_exec: OK rejects {label} ({msg})")

    ok, msg = lsel.verify_corpus_dir(FIXTURES / "good", ANCHOR)
    if not ok:
        print(f"test_loader_signed_exec: FAIL good fixture: {msg}", file=sys.stderr)
        failures += 1
    else:
        print("test_loader_signed_exec: OK accepts good fixture")

    ok, msg = lsel.verify_corpus_dir(CORPUS, ANCHOR)
    if not ok:
        print(f"test_loader_signed_exec: FAIL production corpus: {msg}", file=sys.stderr)
        failures += 1
    else:
        print("test_loader_signed_exec: OK accepts config/loader_signed_exec")

    golden_path = CORPUS / "canonical_body.utf8"
    if not golden_path.is_file():
        print(
            "test_loader_signed_exec: FAIL missing canonical_body.utf8 golden bytes",
            file=sys.stderr,
        )
        failures += 1
    else:
        payload = (CORPUS / lsel.PAYLOAD_NAME).read_bytes()
        manifest_text = (CORPUS / lsel.MANIFEST_NAME).read_text(encoding="utf-8")
        parsed = lsel.parse_manifest(manifest_text)
        signed = lsel.ExecManifest(
            name=parsed.name,
            kind=parsed.kind,
            entry=parsed.entry,
            image=parsed.image,
            requires=parsed.requires,
            digest_hex=actual_digest,
            trust=parsed.trust,
            owner=None,
            description=None,
            signature_hex=None,
        )
        expected = lsel.canonical_signed_body_bytes(signed)
        golden = golden_path.read_bytes()
        if expected != golden:
            print(
                "test_loader_signed_exec: FAIL canonical_body.utf8 mismatch "
                f"(lib={len(expected)} golden={len(golden)})",
                file=sys.stderr,
            )
            failures += 1
        else:
            print("test_loader_signed_exec: OK canonical_body.utf8 matches lib")

    # ADR-0002 vs ADR-0003 canonical body must differ (no accidental reuse).
    gate_body = sel.canonical_signed_body_bytes("loader-gate-test", actual_digest, "system")
    if gate_body == expected:
        print(
            "test_loader_signed_exec: FAIL loader body identical to signed_elf_lib body",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_loader_signed_exec: OK loader canonical body distinct from ADR-0002")

    payload = (CORPUS / lsel.PAYLOAD_NAME).read_bytes()
    manifest_text = (CORPUS / lsel.MANIFEST_NAME).read_text(encoding="utf-8")
    parsed = lsel.parse_manifest(manifest_text)
    forged = manifest_text.replace(parsed.digest_hex, "0" * 64, 1)
    ok, _ = lsel.verify_signed_exec(payload, forged, anchor)
    if ok:
        print(
            "test_loader_signed_exec: FAIL forged digest should not verify",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_loader_signed_exec: OK rejects forged digest")

    proc = subprocess.run([sys.executable, str(HOST)], capture_output=True, text=True)
    if proc.returncode != 0:
        print(
            f"test_loader_signed_exec: FAIL loader_signed_exec.py: {proc.stderr}",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_loader_signed_exec: OK gate/loader_signed_exec host check")

    if failures:
        print(f"test_loader_signed_exec: {failures} failure(s)", file=sys.stderr)
        return 1
    print("test_loader_signed_exec: OK (all cases)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
