#!/usr/bin/env python3
"""QEMU smoke gate for unified validation (subsystem gates)."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

_SCRIPTS = Path(__file__).resolve().parents[1]
if str(_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS))

from gate.map import PREEMPTION_GATES, VALIDATION_GATES, pattern_for_gate
from gate.qemu import run_smoke


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--gate", choices=sorted(VALIDATION_GATES.keys()), help="Validation subsystem gate")
    ap.add_argument("--timeout", type=int, default=360)
    args = ap.parse_args(argv)
    if args.gate is None:
        ap.error("specify --gate")
    pattern = pattern_for_gate(args.gate)
    label = f"gate/run:{args.gate}"
    features = ["--features", "preemption"] if args.gate in PREEMPTION_GATES else None
    return run_smoke(pattern, label, args.timeout, features)


if __name__ == "__main__":
    raise SystemExit(main())
