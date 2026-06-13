#!/usr/bin/env python3
"""Host check: ring3-io-demo builds for x86_64-unknown-none."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
ELF = REPO / "target" / "x86_64-unknown-none" / "release" / "ring3-io-demo"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=120)
    args = ap.parse_args()
    r = subprocess.run(
        [
            "cargo",
            "build",
            "-p",
            "ring3-io-demo",
            "--release",
            "--target",
            "x86_64-unknown-none",
        ],
        cwd=REPO,
        capture_output=True,
        text=True,
        timeout=args.timeout,
    )
    if r.returncode != 0:
        print(r.stdout + r.stderr, file=sys.stderr)
        print("gate/clan_rt_ring3: FAIL (build)", file=sys.stderr)
        return 1
    if not ELF.is_file():
        print(f"gate/clan_rt_ring3: missing ELF at {ELF}", file=sys.stderr)
        return 1
    if ELF.stat().st_size > 32768:
        print("gate/clan_rt_ring3: ELF exceeds MAX_IMAGE_SIZE", file=sys.stderr)
        return 1
    main_rs = REPO / "userland" / "ring3-io-demo" / "src" / "main.rs"
    if "#![no_std]" not in main_rs.read_text(encoding="utf-8"):
        print("gate/clan_rt_ring3: missing #![no_std]", file=sys.stderr)
        return 1
    print("gate/clan_rt_ring3: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
