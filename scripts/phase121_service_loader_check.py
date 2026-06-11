#!/usr/bin/env python3
"""Phase 121 — service loader contract smoke (QEMU boot)."""

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from post150_milestone_check import run_smoke  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=300)
    args = ap.parse_args()
    return run_smoke(
        r"Phase121-ServiceLoader:.*?bootstrap=(true|false).*?ok=(true|false)",
        "phase121_service_loader_check",
        args.timeout,
        ["--features", "preemption"],
        match_ok=lambda m: m.group(1) == "true" and m.group(2) == "true",
    )


if __name__ == "__main__":
    raise SystemExit(main())
