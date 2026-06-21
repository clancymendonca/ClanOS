#!/usr/bin/env python3
"""Deprecated — use `gate/run.py`."""

from __future__ import annotations

import sys
from pathlib import Path

_SCRIPTS = Path(__file__).resolve().parents[1]
if str(_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS))

from gate.run import main


def _warn() -> None:
    print("gate/system.py: deprecated; use scripts/gate/run.py", file=sys.stderr)


if __name__ == "__main__":
    import argparse

    ap = argparse.ArgumentParser(add_help=False)
    ap.add_argument("--gate", default="system")
    ap.add_argument("--timeout", type=int, default=360)
    args, _ = ap.parse_known_args()
    _warn()
    gate = "all" if args.gate == "system" else args.gate
    raise SystemExit(main(["--gate", gate, "--timeout", str(args.timeout)]))
