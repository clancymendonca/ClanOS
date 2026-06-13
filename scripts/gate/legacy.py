#!/usr/bin/env python3
"""Resolve a historical scope index to the correct boot/system gate QEMU check."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

_SCRIPTS = Path(__file__).resolve().parents[1]
if str(_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS))

from gate.map import (
    BOOT_GATES,
    PREEMPTION_BOOT_GATES,
    SYSTEM_GATES,
    gate_cli,
    gate_family,
    gate_for_scope,
)
from gate.qemu import run_smoke


def run_gate(gate: str, timeout: int) -> int:
    family = gate_family(gate)
    gates = SYSTEM_GATES if family == "system" else BOOT_GATES
    if gate not in gates:
        print(f"gate/legacy: unknown gate {gate!r}", file=sys.stderr)
        return 1
    features = None
    if family == "boot" and gate in PREEMPTION_BOOT_GATES:
        features = ["--features", "preemption"]
    pattern = gates[gate]
    label = f"gate/{family}:{gate}"
    return run_smoke(pattern, label, timeout, features)


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--gate", help="Boot or system subsystem gate name")
    ap.add_argument(
        "--scope",
        type=int,
        help="Historical scope checklist index (maps to --gate)",
    )
    ap.add_argument(
        "--milestone",
        type=int,
        help="Deprecated alias for --scope",
    )
    ap.add_argument("--timeout", type=int, default=360)
    args = ap.parse_args(argv)

    scope = args.scope if args.scope is not None else args.milestone
    if args.gate:
        return run_gate(args.gate, args.timeout)
    if scope is None:
        ap.error("specify --gate or --scope")
    gate = gate_for_scope(scope)
    if gate is None:
        print(
            f"gate/legacy: scope {scope} has no gate mapping; "
            f"use: {gate_cli(scope, args.timeout)}",
            file=sys.stderr,
        )
        return 1
    if args.milestone is not None and args.scope is None:
        print(
            f"gate/legacy: --milestone is deprecated; "
            f"use: {gate_cli(scope, args.timeout)}",
            file=sys.stderr,
        )
    return run_gate(gate, args.timeout)


if __name__ == "__main__":
    raise SystemExit(main())
