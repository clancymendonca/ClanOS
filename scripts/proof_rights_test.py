#!/usr/bin/env python3
"""Epoch 0 tier-A rights algebra gate."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    proc = subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "rights_algebra_check.py")],
        cwd=ROOT,
    )
    if proc.returncode == 0:
        print("proof_rights_test: OK")
    return proc.returncode


if __name__ == "__main__":
    raise SystemExit(main())
