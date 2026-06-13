#!/usr/bin/env python3
"""Host check: compat signal skeleton syscalls."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
SIGNAL = REPO / "kernel" / "src" / "signal.rs"
SYSCALL = REPO / "kernel" / "src" / "syscall.rs"
HW = REPO / "kernel" / "src" / "user_syscall_hw.rs"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=30)
    _ = ap.parse_args()
    signal_text = SIGNAL.read_text(encoding="utf-8")
    syscall_text = SYSCALL.read_text(encoding="utf-8")
    hw_text = HW.read_text(encoding="utf-8")
    for needle in ("kill_checked", "sigaction_lite", "smoke_signal_register"):
        if needle not in signal_text:
            print(f"gate/signal_skeleton: missing {needle}", file=sys.stderr)
            return 1
    for name in ("Kill = 83", "SigActionLite = 84", "SigPending = 85"):
        if name not in syscall_text:
            print(f"gate/signal_skeleton: missing {name}", file=sys.stderr)
            return 1
    if "SyscallId::Kill" not in hw_text:
        print("gate/signal_skeleton: Kill not allowlisted", file=sys.stderr)
        return 1
    if not re.search(r"max_id <= 86", Path(REPO / "kernel/src/kernel_object.rs").read_text()):
        print("gate/signal_skeleton: allowlist bound stale", file=sys.stderr)
        return 1
    print("gate/signal_skeleton: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
