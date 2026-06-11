#!/usr/bin/env python3
"""MEMORY_SAFETY_BOUNDARY — forbidden-path unsafe scan."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
KERNEL = ROOT / "kernel" / "src"
# Userland policy stubs are allowed unsafe; kernel user-facing brokers are not.
FORBIDDEN = [
    "service_loader.rs",
    "permission_broker.rs",
    "audit_wire.rs",
]


def main() -> int:
    errors: list[str] = []
    pat = re.compile(r"\bunsafe\b")
    for name in FORBIDDEN:
        path = KERNEL / name
        if not path.exists():
            continue
        for i, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            if pat.search(line) and not line.strip().startswith("//"):
                errors.append(f"{name}:{i}: unsafe in forbidden module")
    if errors:
        for e in errors:
            print(f"unsafe_boundary_check: {e}", file=sys.stderr)
        return 1
    print("unsafe_boundary_check: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
