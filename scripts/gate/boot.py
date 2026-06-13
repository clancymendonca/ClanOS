#!/usr/bin/env python3
"""QEMU smoke gate for boot validation (phases 6–150)."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

_SCRIPTS = Path(__file__).resolve().parents[1]
if str(_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS))

from gate.map import BOOT_GATES, boot_gate_for_phase
from gate.qemu import run_smoke

PREEMPTION_GATES = frozenset({"boot", "boundary", "shell_storage"})


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--gate", choices=sorted(BOOT_GATES.keys()), help="Boot subsystem gate")
    ap.add_argument("--phase", type=int, help="Legacy phase number (maps to boot gate)")
    ap.add_argument("--timeout", type=int, default=360)
    args = ap.parse_args(argv)
    gate = args.gate
    if args.phase is not None:
        mapped = boot_gate_for_phase(args.phase)
        if mapped is None:
            print(f"gate/boot: no boot gate mapping for phase {args.phase}", file=sys.stderr)
            return 1
        gate = mapped
    if gate is None:
        ap.error("specify --gate or --phase")
    pattern = BOOT_GATES[gate]
    label = f"gate/boot:{gate}"
    features = ["--features", "preemption"] if gate in PREEMPTION_GATES else None
    return run_smoke(pattern, label, args.timeout, features)


if __name__ == "__main__":
    raise SystemExit(main())
