#!/usr/bin/env python3
"""Host-side validation for unified validation gate (no QEMU)."""

from __future__ import annotations

import sys
from pathlib import Path

_SCRIPTS = Path(__file__).resolve().parents[1]
if str(_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS))

from gate.gate_host import main as gate_main


def main() -> int:
    return gate_main()


if __name__ == "__main__":
    raise SystemExit(main())
