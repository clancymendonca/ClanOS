#!/usr/bin/env python3
"""Shared QEMU smoke matcher for post-150 milestone serial lines."""

from __future__ import annotations

import os
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]


def cleanup() -> None:
    if os.name == "nt":
        subprocess.run(
            ["taskkill", "/IM", "qemu-system-x86_64.exe", "/F"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )


def run_smoke(pattern: str, label: str, timeout: int = 300) -> int:
    phase_re = re.compile(pattern)
    p = subprocess.Popen(
        ["cargo", "run", "-p", "kernel", "--features", "preemption"],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        cwd=REPO,
    )
    try:
        out, _ = p.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        p.kill()
        cleanup()
        print(f"{label}: timeout", file=sys.stderr)
        return 1
    for line in out.splitlines():
        m = phase_re.search(line)
        if m and m.group(1) == "true":
            print(f"{label}: OK")
            return 0
    print(out[-3000:], file=sys.stderr)
    return 1
