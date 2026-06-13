#!/usr/bin/env python3
"""Compatibility shim — use scripts/gate/system_host.py."""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from gate.system_host import main

if __name__ == "__main__":
    raise SystemExit(main())
