#!/usr/bin/env python3
"""Host check: signal user-frame delivery + SigReturn syscall."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
SIGNAL = REPO / "kernel" / "src" / "signal.rs"
SYSCALL = REPO / "kernel" / "src" / "syscall.rs"
HW = REPO / "kernel" / "src" / "user_syscall_hw.rs"
KO = REPO / "kernel" / "src" / "kernel_object.rs"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=30)
    _ = ap.parse_args()
    signal_text = SIGNAL.read_text(encoding="utf-8")
    syscall_text = SYSCALL.read_text(encoding="utf-8")
    hw_text = HW.read_text(encoding="utf-8")
    for needle in (
        "try_deliver_on_syscall_return",
        "take_sigreturn_restoration",
        "sigreturn_syscall",
        "smoke_signal_delivery",
    ):
        if needle not in signal_text:
            print(f"gate/signal_delivery: missing {needle}", file=sys.stderr)
            return 1
    if "SigReturn = 86" not in syscall_text:
        print("gate/signal_delivery: missing SigReturn syscall id", file=sys.stderr)
        return 1
    if "SyscallId::SigReturn" not in hw_text:
        print("gate/signal_delivery: SigReturn not allowlisted", file=sys.stderr)
        return 1
    if "try_deliver_on_syscall_return" not in hw_text:
        print("gate/signal_delivery: hw trampoline not wired", file=sys.stderr)
        return 1
    if not re.search(r"max_id <= 86", KO.read_text(encoding="utf-8")):
        print("gate/signal_delivery: allowlist bound stale", file=sys.stderr)
        return 1
    print("gate/signal_delivery: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
