#!/usr/bin/env python3
"""Phase 130 platform integration smoke (QEMU)."""

import os
import re
import signal
import subprocess
import sys
from pathlib import Path

PHASE_RE = re.compile(
    r"Phase130-Platform:.*?ok=(true|false)"
)

REPO_ROOT = Path(__file__).resolve().parents[1]


def cleanup():
    if os.name == "nt":
        subprocess.run(
            ["taskkill", "/IM", "qemu-system-x86_64.exe", "/F"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )


def main() -> int:
    p = subprocess.Popen(
        ["cargo", "run", "-p", "kernel", "--features", "preemption"],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        cwd=REPO_ROOT,
    )
    try:
        out, _ = p.communicate(timeout=300)
    except subprocess.TimeoutExpired:
        p.kill()
        out, _ = p.communicate(timeout=5)
        cleanup()
        return 1
    for line in out.splitlines():
        m = PHASE_RE.search(line)
        if m and m.group(1) == "true":
            print("phase130_platform_check: OK")
            return 0
    print(out[-3000:], file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
