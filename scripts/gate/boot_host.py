#!/usr/bin/env python3
"""Deprecated — use `gate/gate_host.py`."""

from __future__ import annotations

import sys
from gate.gate_host import main

if __name__ == "__main__":
    print("gate/boot_host.py: deprecated; use scripts/gate/gate_host.py", file=sys.stderr)
    raise SystemExit(main())
