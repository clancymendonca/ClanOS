#!/usr/bin/env python3
"""Resolve a historical scope index to the correct validation gate QEMU check."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

_SCRIPTS = Path(__file__).resolve().parents[1]
if str(_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS))

from gate.map import PREEMPTION_GATES, VALIDATION_GATES, gate_cli, gate_for_scope, pattern_for_gate
from gate.qemu import run_smoke


def run_gate(gate: str, timeout: int) -> int:
    if gate not in VALIDATION_GATES:
        print(f"gate/legacy: unknown gate {gate!r}", file=sys.stderr)
        return 1
    features = None
    if gate in PREEMPTION_GATES:
        features = ["--features", "preemption"]
    pattern = pattern_for_gate(gate)
    label = f"gate/legacy:{gate}"
    return run_smoke(pattern, label, timeout, features)


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--gate", help="Validation subsystem gate name")
    ap.add_argument(
        "--scope",
        type=int,
        help="Historical scope checklist index (maps to --gate)",
    )
    ap.add_argument("--timeout", type=int, default=360)
    args = ap.parse_args(argv)

    if args.gate:
        return run_gate(args.gate, args.timeout)
    if args.scope is None:
        ap.error("specify --gate or --scope")
    gate = gate_for_scope(args.scope)
    if gate is None:
        print(
            f"gate/legacy: scope {args.scope} has no gate mapping; "
            f"use: {gate_cli(args.scope, args.timeout)}",
            file=sys.stderr,
        )
        return 1
    return run_gate(gate, args.timeout)


if __name__ == "__main__":
    raise SystemExit(main())
