#!/usr/bin/env python3
"""QEMU smoke gate for system validation (M500 subsystem gates)."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

_SCRIPTS = Path(__file__).resolve().parents[1]
if str(_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS))

from gate.map import SYSTEM_GATES
from gate.qemu import run_smoke


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--gate",
        choices=sorted(SYSTEM_GATES.keys()),
        help="System subsystem gate",
    )
    ap.add_argument("--timeout", type=int, default=360)
    args = ap.parse_args(argv)
    gate = args.gate or "system"
    pattern = SYSTEM_GATES[gate]
    label = f"gate/system:{gate}"
    return run_smoke(pattern, label, args.timeout)


if __name__ == "__main__":
    raise SystemExit(main())
