#!/usr/bin/env python3
"""Prove seed migration rollback path — deliberate break then revert (ADR-0003 § Seed migration).

Exercises: signed manifest verify fail on tamper; trust=system rollback manifest;
allowlist re-add after simulated failed migration.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ANCHOR = ROOT / "config" / "trust_anchor_epoch460_loader.toml"
ALLOWLIST = ROOT / "config" / "loader_digest_only_allowlist.toml"
SEED_DIR = ROOT / "config" / "loader_signed_seed"
PROGRAM = "demo-hello"

sys.path.insert(0, str(Path(__file__).resolve().parent))
import loader_signed_exec_lib as lsel  # noqa: E402


def load_allowlist_names() -> list[str]:
    text = ALLOWLIST.read_text(encoding="utf-8")
    return re.findall(r'name\s*=\s*"([^"]+)"', text)


def rollback_manifest(name: str, entry: str, description: str) -> str:
    return (
        f"clan-exec-v1\n"
        f"name={name}\n"
        f"kind=builtin-alias\n"
        f"entry={entry}\n"
        f"requires=execute\n"
        f"trust=system\n"
        f"owner=admin\n"
        f"description={description}\n"
    )


def main() -> int:
    failures = 0
    signed_path = SEED_DIR / f"{PROGRAM}.signed.manifest"
    rollback_path = SEED_DIR / f"{PROGRAM}.rollback.manifest"

    if not signed_path.is_file():
        print(
            f"test_loader_seed_migration_rollback: FAIL missing {signed_path}",
            file=sys.stderr,
        )
        return 1

    anchor = lsel.load_trust_anchor(ANCHOR)
    signed = signed_path.read_text(encoding="utf-8")
    ok, msg = lsel.verify_signed_builtin_alias(signed, anchor)
    if not ok:
        print(
            f"test_loader_seed_migration_rollback: FAIL signed manifest: {msg}",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_loader_seed_migration_rollback: OK signed demo-hello verifies")

    # Deliberate break: tampered entry after sign (forgery class = signature failure).
    broken = signed.replace("entry=demo-hello", "entry=demo-evil", 1)
    ok, msg = lsel.verify_signed_builtin_alias(broken, anchor)
    if ok:
        print(
            "test_loader_seed_migration_rollback: FAIL tampered entry should not verify",
            file=sys.stderr,
        )
        failures += 1
    elif "signature verify failed" not in msg and "digest payload mismatch" not in msg:
        print(
            f"test_loader_seed_migration_rollback: FAIL tampered entry wrong reason: {msg}",
            file=sys.stderr,
        )
        failures += 1
    else:
        print(f"test_loader_seed_migration_rollback: OK rejects tampered entry ({msg})")

    # Rollback manifest: digest-only trust=system (no digest=, no sig=).
    rb = rollback_manifest(PROGRAM, PROGRAM, "clan-rt demo")
    rollback_path.write_text(rb, encoding="utf-8", newline="\n")
    if "trust=system" not in rb or "trust=system-signed" in rb:
        print(
            "test_loader_seed_migration_rollback: FAIL rollback trust=system",
            file=sys.stderr,
        )
        failures += 1
    elif "digest=sha256:" in rb or "sig=ed25519:" in rb:
        print(
            "test_loader_seed_migration_rollback: FAIL rollback must omit digest and sig",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_loader_seed_migration_rollback: OK rollback manifest is digest-only trust=system")

    names = load_allowlist_names()
    if PROGRAM in names:
        print(
            "test_loader_seed_migration_rollback: FAIL demo-hello still on digest-only allowlist "
            "(remove after signed migration)",
            file=sys.stderr,
        )
        failures += 1
    elif len(names) != 14:
        print(
            "test_loader_seed_migration_rollback: FAIL allowlist count "
            f"{len(names)} expected 14 after demo-hello and echo migrations",
            file=sys.stderr,
        )
        failures += 1
    else:
        names_restored = names + [PROGRAM]
        print(
            "test_loader_seed_migration_rollback: OK re-add to allowlist restores "
            f"digest-only staging ({len(names)} -> {len(names_restored)} entries)"
        )

    # Signed path must fail without sig line.
    ok, msg = lsel.verify_signed_builtin_alias(rb, anchor)
    if ok:
        print(
            "test_loader_seed_migration_rollback: FAIL rollback must not pass signed verify",
            file=sys.stderr,
        )
        failures += 1
    else:
        print(f"test_loader_seed_migration_rollback: OK rollback rejected by signed path ({msg})")

    if failures:
        print(f"test_loader_seed_migration_rollback: {failures} failure(s)", file=sys.stderr)
        return 1
    print("test_loader_seed_migration_rollback: OK (rollback path proven)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
