#!/usr/bin/env python3
"""Loom gate — registry present; full harnesses graduate before real SMP AP."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "loom_harness_registry.toml"


def main() -> int:
    if not REGISTRY.exists():
        print("loom_gate: missing loom_harness_registry.toml", file=sys.stderr)
        return 1
    text = REGISTRY.read_text(encoding="utf-8")
    count = len(re.findall(r"\[\[harnesses\]\]", text))
    if count < 2:
        print("loom_gate: need >=2 harness entries", file=sys.stderr)
        return 1
    graduated = len(re.findall(r'status = "graduated"', text))
    if graduated < count:
        print(
            f"loom_gate: {graduated}/{count} harnesses graduated "
            "(epoch 7 requires all graduated)",
            file=sys.stderr,
        )
        return 1
    print(f"loom_gate: OK ({count} harnesses graduated)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
