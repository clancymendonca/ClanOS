#!/usr/bin/env python3
"""M350 release scorecard gate (epoch 14 graduation)."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
from mark_epoch0_addressed import parse_gaps  # noqa: E402


def main() -> int:
    catalog = (ROOT / "kernel" / "src" / "phase_catalog.rs").read_text(encoding="utf-8")
    m = re.search(r"COMPLETED_PHASE: u32 = (\d+)", catalog)
    if not m or int(m.group(1)) < 350:
        print("release_scorecard_check: COMPLETED_PHASE < 350", file=sys.stderr)
        return 1
    gaps = parse_gaps((ROOT / "gap_registry.toml").read_text(encoding="utf-8"))
    open_gaps = sum(1 for g in gaps if g.get("status") == "open")
    if open_gaps > 0:
        print(f"release_scorecard_check: {open_gaps} open gaps remain", file=sys.stderr)
        return 1
    nodes = parse_gaps((ROOT / "docs" / "THREAT_NODES.toml").read_text(encoding="utf-8"))
    # THREAT_NODES uses [[nodes]] not [[gaps]]
    threat_text = (ROOT / "docs" / "THREAT_NODES.toml").read_text(encoding="utf-8")
    open_threats = len(re.findall(r'status = "open"', threat_text))
    if open_threats > 0:
        print(f"release_scorecard_check: {open_threats} open threat nodes", file=sys.stderr)
        return 1
    print("release_scorecard_check: OK (phase 350, 0 open gaps, 0 open threats)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
