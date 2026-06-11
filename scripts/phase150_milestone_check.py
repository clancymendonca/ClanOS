#!/usr/bin/env python3
"""Milestone 150 four-layer boundary smoke (QEMU)."""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from post150_milestone_check import run_smoke  # noqa: E402


def main() -> int:
    return run_smoke(
        r"Phase150-Milestone: ok=(true|false)",
        "phase150_milestone_check",
        300,
        ["--features", "preemption"],
    )


if __name__ == "__main__":
    raise SystemExit(main())
