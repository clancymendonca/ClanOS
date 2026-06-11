#!/usr/bin/env python3
"""Phase 134 native endpoints + interim bridge counter zero (QEMU)."""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from post150_milestone_check import run_smoke  # noqa: E402


def main() -> int:
    return run_smoke(
        r"Phase140-IPC:.*?p134=(true|false).*?bridge=(\d+).*?ok=(true|false)",
        "phase134_endpoint_check",
        300,
        ["--features", "preemption"],
        match_ok=lambda m: m.group(1) == "true" and m.group(2) == "0",
    )


if __name__ == "__main__":
    raise SystemExit(main())
