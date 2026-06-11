#!/usr/bin/env python3
"""Epoch 5 integration smoke — scheduler, SMP, compositor, OOM (QEMU)."""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from post150_milestone_check import run_smoke  # noqa: E402


def main() -> int:
    return run_smoke(
        r"Phase149-Epoch5: ok=(true|false)",
        "phase149_epoch5_check",
        300,
        ["--features", "preemption"],
    )


if __name__ == "__main__":
    raise SystemExit(main())
