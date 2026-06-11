#!/usr/bin/env python3
"""Native SDK path smoke (epochs 9–10) — userland + ABI doc present."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REQUIRED = [
    ROOT / "userland" / "Cargo.toml",
    ROOT / "userland" / "src" / "lib.rs",
    ROOT / "docs" / "ABI_ARES_RT.md",
    ROOT / "scripts" / "install_userland.py",
]


def main() -> int:
    missing = [p for p in REQUIRED if not p.exists()]
    if missing:
        for p in missing:
            print(f"sdk_smoke_check: missing {p}", file=sys.stderr)
        return 1
    host = subprocess.check_output(
        [sys.executable, str(ROOT / "scripts" / "host_target.py")],
        text=True,
        cwd=ROOT,
    ).strip()
    proc = subprocess.run(
        [
            "cargo",
            "check",
            "--manifest-path",
            str(ROOT / "userland" / "Cargo.toml"),
            "--target",
            host,
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        print(proc.stdout + proc.stderr, file=sys.stderr)
        return 1
    print("sdk_smoke_check: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
