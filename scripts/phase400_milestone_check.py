#!/usr/bin/env python3
"""Milestone 400 functional OS gate."""

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from post150_milestone_check import run_smoke  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=360)
    args = ap.parse_args()
    return run_smoke(r"Phase400-Milestone: ok=(true|false)", "phase400_milestone_check", args.timeout)


if __name__ == "__main__":
    raise SystemExit(main())
