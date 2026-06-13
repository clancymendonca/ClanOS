#!/usr/bin/env python3
"""Host check: clan-rt no_std (scope 401)."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=120)
    _ = ap.parse_args()
    r = subprocess.run(
        ["cargo", "check", "-p", "clan-rt", "--lib"],
        cwd=REPO,
        capture_output=True,
        text=True,
        timeout=120,
    )
    if r.returncode != 0:
        print(r.stdout + r.stderr, file=sys.stderr)
        print("gate/clan_rt: FAIL", file=sys.stderr)
        return 1
    lib = REPO / "userland" / "src" / "lib.rs"
    if "#![no_std]" not in lib.read_text(encoding="utf-8"):
        print("gate/clan_rt: missing #![no_std]", file=sys.stderr)
        return 1
    print("gate/clan_rt: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
