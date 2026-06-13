#!/usr/bin/env python3
"""Host-side validation for boot + system gates (no QEMU)."""

from __future__ import annotations

import sys
from pathlib import Path

_SCRIPTS = Path(__file__).resolve().parents[1]
if str(_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS))

from gate.boot_host import main as boot_main
from gate.system_host import main as system_main


def main() -> int:
    for fn, label in ((boot_main, "boot"), (system_main, "system")):
        code = fn()
        if code != 0:
            return code
    print("gate/host: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
