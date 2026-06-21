#!/usr/bin/env python3
"""Self-test for loader_signing_sunset_check — synthetic fail cases."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CHECK = Path(__file__).resolve().parent / "loader_signing_sunset_check.py"


def main() -> int:
    import subprocess

    proc = subprocess.run([sys.executable, str(CHECK)], capture_output=True, text=True)
    if proc.returncode != 0:
        print(f"test_loader_signing_sunset_check: FAIL live check: {proc.stderr}", file=sys.stderr)
        return 1
    print("test_loader_signing_sunset_check: OK live policy passes")

    arch_path = ROOT / "architecture_state.toml"
    arch_original = arch_path.read_text(encoding="utf-8")
    if "loader_digest_only_grace = false" not in arch_original:
        print(
            "test_loader_signing_sunset_check: FAIL expected loader_digest_only_grace=false "
            "after scope 465 close",
            file=sys.stderr,
        )
        return 1
    print("test_loader_signing_sunset_check: OK grace=false with empty allowlist")

    policy_path = ROOT / "config" / "loader_signing_policy.toml"
    original = policy_path.read_text(encoding="utf-8")
    try:
        # Non-empty allowlist must still fail at sunset scope.
        allowlist_path = ROOT / "config" / "loader_digest_only_allowlist.toml"
        allowlist_original = allowlist_path.read_text(encoding="utf-8")
        allowlist_path.write_text(
            allowlist_original + '\n[[programs]]\nname = "rollback-stub"\n',
            encoding="utf-8",
        )
        proc = subprocess.run([sys.executable, str(CHECK)], capture_output=True, text=True)
        if proc.returncode == 0:
            print(
                "test_loader_signing_sunset_check: FAIL should reject non-empty allowlist at scope 465",
                file=sys.stderr,
            )
            return 1
        print("test_loader_signing_sunset_check: OK rejects non-empty allowlist at scope 465")
    finally:
        allowlist_path.write_text(allowlist_original, encoding="utf-8")

    print("test_loader_signing_sunset_check: OK (all cases)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
