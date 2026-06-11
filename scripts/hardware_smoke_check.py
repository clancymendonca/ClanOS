#!/usr/bin/env python3
"""Hardware transition host smoke (epoch 10) — architecture_state flags documented."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
STATE = ROOT / "architecture_state.toml"
TARGETS = ROOT / "docs" / "ARCHITECTURE_TARGETS.md"


def main() -> int:
    if not STATE.exists() or not TARGETS.exists():
        print("hardware_smoke_check: missing architecture_state or ARCHITECTURE_TARGETS", file=sys.stderr)
        return 1
    text = STATE.read_text(encoding="utf-8")
    if "qemu" not in text.lower() and "hardware" not in text.lower():
        print("hardware_smoke_check: architecture_state missing hw/qemu flags", file=sys.stderr)
        return 1
    print("hardware_smoke_check: OK (architecture_state + targets present)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
