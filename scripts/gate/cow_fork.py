#!/usr/bin/env python3
"""Host check: PF-driven CoW fork wiring."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
COW = REPO / "kernel" / "src" / "cow_fork.rs"
DEMAND = REPO / "kernel" / "src" / "demand_paging.rs"
PAGING = REPO / "kernel" / "src" / "user_paging.rs"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=30)
    _ = ap.parse_args()
    cow_text = COW.read_text(encoding="utf-8")
    demand_text = DEMAND.read_text(encoding="utf-8")
    paging_text = PAGING.read_text(encoding="utf-8")
    if "try_break_on_write" not in cow_text:
        print("gate/cow_fork: missing try_break_on_write", file=sys.stderr)
        return 1
    if "share_after_fork" not in cow_text:
        print("gate/cow_fork: missing share_after_fork", file=sys.stderr)
        return 1
    if "cow_fork::try_break_on_write" not in demand_text:
        print("gate/cow_fork: demand paging not wired", file=sys.stderr)
        return 1
    if "privatize_cow_page" not in paging_text:
        print("gate/cow_fork: missing privatize_cow_page", file=sys.stderr)
        return 1
    if "smoke_cow_fork" not in cow_text:
        print("gate/cow_fork: missing smoke", file=sys.stderr)
        return 1
    print("gate/cow_fork: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
