#!/usr/bin/env python3
"""CI: count ipc-bridge-compat-internal references in kernel sources."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
KERNEL = ROOT / "kernel" / "src"
PATTERNS = [
    re.compile(r"ipc_bridge_compat_internal"),
    re.compile(r"ipc_interim_bridge::"),
]

def main() -> int:
    count = 0
    for path in KERNEL.rglob("*.rs"):
        text = path.read_text(encoding="utf-8")
        for pat in PATTERNS:
            count += len(pat.findall(text))
    print(f"ipc_bridge_compat_internal_refs={count}")
    # Phase 134 gate requires counter API only — not zero refs until bridge removal.
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
