#!/usr/bin/env python3
"""Host check: per-process FD table capacity is 64."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
FD_TABLE = REPO / "kernel" / "src" / "fd_table.rs"
EXPECTED = 64


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=30)
    _ = ap.parse_args()
    text = FD_TABLE.read_text(encoding="utf-8")
    match = re.search(r"pub const MAX_FDS: usize = (\d+);", text)
    if not match:
        print("gate/fd_capacity: MAX_FDS not found", file=sys.stderr)
        return 1
    value = int(match.group(1))
    if value != EXPECTED:
        print(f"gate/fd_capacity: expected MAX_FDS={EXPECTED}, got {value}", file=sys.stderr)
        return 1
    if "smoke_fd_capacity" not in text:
        print("gate/fd_capacity: missing capacity smoke", file=sys.stderr)
        return 1
    print("gate/fd_capacity: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
