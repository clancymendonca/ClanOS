"""Compatibility shim — use scripts/gate/qemu.py."""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from gate.qemu import cleanup, run_smoke

__all__ = ["cleanup", "run_smoke"]
