#!/usr/bin/env python3
"""Mark open gaps addressed when their when field matches a graduated epoch."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
from mark_epoch0_addressed import emit_gaps, parse_gaps  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("epoch", type=int, help="Epoch number (7-14)")
    ap.add_argument("--doc", default="", help="implementing_doc value")
    args = ap.parse_args()
    when = f"Epoch {args.epoch}"
    path = ROOT / "gap_registry.toml"
    gaps = parse_gaps(path.read_text(encoding="utf-8"))
    closed = 0
    for g in gaps:
        if g.get("status") != "open":
            continue
        if g.get("when") != when:
            continue
        g["status"] = "addressed"
        if args.doc:
            g["implementing_doc"] = args.doc
        closed += 1
    path.write_text(emit_gaps(gaps), encoding="utf-8")
    open_count = sum(1 for g in gaps if g.get("status") == "open")
    print(f"graduate_epoch_gaps: epoch {args.epoch} closed {closed}; {open_count} open remaining")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
