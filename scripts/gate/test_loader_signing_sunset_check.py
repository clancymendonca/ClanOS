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

    policy_path = ROOT / "config" / "loader_signing_policy.toml"
    original = policy_path.read_text(encoding="utf-8")
    try:
        policy_path.write_text(
            original.replace("current_scope = 460", "current_scope = 465"),
            encoding="utf-8",
        )
        proc = subprocess.run([sys.executable, str(CHECK)], capture_output=True, text=True)
        if proc.returncode == 0:
            print(
                "test_loader_signing_sunset_check: FAIL should reject scope>=sunset with allowlist",
                file=sys.stderr,
            )
            return 1
        print("test_loader_signing_sunset_check: OK rejects scope 465 with non-empty allowlist")
    finally:
        policy_path.write_text(original, encoding="utf-8")

    print("test_loader_signing_sunset_check: OK (all cases)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
