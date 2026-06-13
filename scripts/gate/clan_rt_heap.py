#!/usr/bin/env python3
"""Host check: hello-alloc builds for x86_64-unknown-none with clan-rt heap."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
ELF = REPO / "target" / "x86_64-unknown-none" / "release" / "hello-alloc"
MAX_IMAGE_SIZE = 32768


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=180)
    args = ap.parse_args()
    r = subprocess.run(
        [
            "cargo",
            "build",
            "-p",
            "hello-alloc",
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
        print("gate/clan_rt_heap: FAIL (build)", file=sys.stderr)
        return 1
    if not ELF.is_file():
        print(f"gate/clan_rt_heap: missing ELF at {ELF}", file=sys.stderr)
        return 1
    if ELF.stat().st_size > MAX_IMAGE_SIZE:
        print("gate/clan_rt_heap: ELF exceeds MAX_IMAGE_SIZE", file=sys.stderr)
        return 1
    heap_rs = REPO / "userland" / "src" / "heap.rs"
    if "#[global_allocator]" not in heap_rs.read_text(encoding="utf-8"):
        print("gate/clan_rt_heap: missing global allocator", file=sys.stderr)
        return 1
    main_rs = REPO / "userland" / "hello-alloc" / "src" / "main.rs"
    if "extern crate alloc" not in main_rs.read_text(encoding="utf-8"):
        print("gate/clan_rt_heap: missing extern crate alloc", file=sys.stderr)
        return 1
    print("gate/clan_rt_heap: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
