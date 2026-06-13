#!/usr/bin/env python3
"""Route a legacy phase number to the correct boot or system gate check."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

_SCRIPTS = Path(__file__).resolve().parents[1]
if str(_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS))

from gate.boot import main as boot_main
from gate.map import boot_gate_for_phase, system_gate_for_phase
from gate.system import main as system_main


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--phase", type=int, required=True, help="Legacy phase or milestone number")
    ap.add_argument("--timeout", type=int, default=360)
    args = ap.parse_args(argv)
    phase = args.phase
    if boot_gate_for_phase(phase) is not None:
        return boot_main(["--phase", str(phase), "--timeout", str(args.timeout)])
    if system_gate_for_phase(phase) is not None:
        return system_main(["--phase", str(phase), "--timeout", str(args.timeout)])
    print(f"gate/legacy: no gate mapping for phase {phase}", file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
