#!/usr/bin/env python3
"""Host check: compat signal skeleton (kernel-internal kill/sigaction smokes)."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
SIGNAL = REPO / "kernel" / "src" / "signal.rs"
LIB = REPO / "kernel" / "src" / "lib.rs"
GATE = REPO / "kernel" / "src" / "validation_gate.rs"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=30)
    _ = ap.parse_args()
    signal_text = SIGNAL.read_text(encoding="utf-8")
    for needle in (
        "kill_checked",
        "sigaction_lite",
        "smoke_signal_register",
        "smoke_signal_delivery",
    ):
        if needle not in signal_text:
            print(f"gate/signal_skeleton: missing {needle}", file=sys.stderr)
            return 1
    if "pub mod signal;" not in LIB.read_text(encoding="utf-8"):
        print("gate/signal_skeleton: module not in lib.rs", file=sys.stderr)
        return 1
    gate_text = GATE.read_text(encoding="utf-8")
    if "crate::signal::smoke_signal_register" not in gate_text:
        print("gate/signal_skeleton: not wired in validation_gate", file=sys.stderr)
        return 1
    print("gate/signal_skeleton: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
