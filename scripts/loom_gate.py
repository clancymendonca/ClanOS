#!/usr/bin/env python3
"""Loom gate — registry present; full harnesses graduate before real SMP AP."""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "loom_harness_registry.toml"


def harness_sources_ok(text: str) -> bool:
    """Graduated harnesses must reference real queue structures on disk."""
    files = re.findall(r'file = "([^"]+)"', text)
    for rel in files:
        path = ROOT / rel.replace("/", "\\") if os.name == "nt" else ROOT / rel
        if not path.exists():
            return False
        body = path.read_text(encoding="utf-8", errors="replace")
        if "queue" not in body.lower() and "endpoint" not in body.lower():
            return False
    return bool(files)


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
    if not harness_sources_ok(text):
        print("loom_gate: harness source files missing queue/endpoint logic", file=sys.stderr)
        return 1
    print(f"loom_gate: OK ({count} harnesses graduated, sources verified)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
