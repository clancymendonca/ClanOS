#!/usr/bin/env python3
"""Host check: consolidated compat subsystem gates in validation_gate.rs."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GATE = ROOT / "kernel" / "src" / "validation_gate.rs"

SMOKES = (
    "smoke_compat_runtime",
    "smoke_compat_fd_vm",
    "smoke_compat_signal",
    "smoke_storage_depth",
    "smoke_posix_compat",
    "compat_subsystems_smoke",
)


def main() -> int:
    text = GATE.read_text(encoding="utf-8")
    for fn in SMOKES:
        if fn not in text:
            print(f"gate/compat_subsystems: missing {fn}", file=sys.stderr)
            return 1
    if "compat_subsystems_smoke()" not in text.split("functional_gate", 1)[-1]:
        print("gate/compat_subsystems: not wired into functional_gate", file=sys.stderr)
        return 1
    proc = subprocess.run(
        [
            "cargo",
            "build",
            "-p",
            "sig-demo",
            "--release",
            "--target",
            "x86_64-unknown-none",
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        print(proc.stdout + proc.stderr, file=sys.stderr)
        print("gate/compat_subsystems: FAIL (sig-demo build)", file=sys.stderr)
        return 1
    print("gate/compat_subsystems: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
