#!/usr/bin/env python3
"""Warn on gap_registry open gaps past their when epoch (milestone 150 complete)."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MILESTONE_EPOCH = 6


def parse_gaps(text: str) -> list[dict]:
    gaps: list[dict] = []
    for block in re.split(r"\[\[gaps\]\]", text)[1:]:
        g: dict = {}
        for line in block.splitlines():
            if "=" not in line:
                continue
            key, _, val = line.partition("=")
            key = key.strip()
            val = val.strip().strip('"')
            if key == "id":
                g[key] = int(val)
            else:
                g[key] = val
        if g:
            gaps.append(g)
    return gaps


def epoch_from_when(when: str) -> int | None:
    m = re.search(r"Epoch\s+(\d+)", when, re.I)
    return int(m.group(1)) if m else None


def main() -> int:
    text = (ROOT / "gap_registry.toml").read_text(encoding="utf-8")
    gaps = parse_gaps(text)
    overdue = []
    for g in gaps:
        if g.get("status") != "open":
            continue
        ep = epoch_from_when(g.get("when", ""))
        if ep is not None and ep <= MILESTONE_EPOCH:
            overdue.append(g["id"])
    if len(overdue) > 50:
        print(
            f"gap_registry_epoch_warn: WARN {len(overdue)} open gaps at/before epoch {MILESTONE_EPOCH} "
            f"(sample ids: {overdue[:10]}...)"
        )
    else:
        print(f"gap_registry_epoch_warn: OK ({len(overdue)} overdue open gaps, warn-only)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
