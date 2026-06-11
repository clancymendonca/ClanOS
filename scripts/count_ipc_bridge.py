#!/usr/bin/env python3
"""CI: ipc bridge retired by phase 134 — counter API must reach zero after retire."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ENDPOINTS = ROOT / "kernel" / "src" / "ipc_endpoints.rs"
GOVERNANCE = ROOT / "kernel" / "src" / "governance.rs"


def main() -> int:
    ep = ENDPOINTS.read_text(encoding="utf-8")
    gov = GOVERNANCE.read_text(encoding="utf-8")
    if "retire_bridge()" not in ep:
        print("count_ipc_bridge: retire_bridge() not called from ipc_endpoints", file=sys.stderr)
        return 1
    if "ipc_bridge_compat_internal_count() == 0" not in gov:
        print("count_ipc_bridge: phase140 bridge_zero check missing", file=sys.stderr)
        return 1
    refs = len(re.findall(r"ipc_bridge_compat_internal", ep + gov))
    print(f"count_ipc_bridge: OK (retire path present; static_refs={refs})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
