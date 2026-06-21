#!/usr/bin/env python3
"""Host check: POSIX compat server skeleton (native service + IPC endpoint)."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
POSIX = REPO / "kernel" / "src" / "posix_server.rs"
LIB = REPO / "kernel" / "src" / "lib.rs"
GATE = REPO / "kernel" / "src" / "validation_gate.rs"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=30)
    _ = ap.parse_args()
    posix_text = POSIX.read_text(encoding="utf-8")
    for needle in (
        "posix.compat.v1",
        "ensure_posix_server",
        "invoke_compat",
        "OP_GETPID",
        "OP_OPEN",
        "smoke_posix_server",
        "load_service_with_stubs",
    ):
        if needle not in posix_text:
            print(f"gate/posix_server: missing {needle}", file=sys.stderr)
            return 1
    if "pub mod posix_server;" not in LIB.read_text(encoding="utf-8"):
        print("gate/posix_server: module not in lib.rs", file=sys.stderr)
        return 1
    if "posix_server" not in GATE.read_text(encoding="utf-8"):
        print("gate/posix_server: not wired in validation_gate", file=sys.stderr)
        return 1
    print("gate/posix_server: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
