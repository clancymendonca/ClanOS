#!/usr/bin/env python3
"""Phase 121 — service loader contract smoke (QEMU boot)."""

import argparse
import os
import re
import signal
import subprocess
import sys
from pathlib import Path

PHASE_RE = re.compile(
    r"Phase121-ServiceLoader:\s+bootstrap=(true|false),\s+stubs=(true|false),\s+"
    r"budget_rej=(true|false),\s+mem_total=(\d+),\s+mem_used=(\d+),\s+mem_free=(\d+),\s+ok=(true|false)"
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


def run_kernel(timeout: int) -> tuple[int, str]:
    p = subprocess.Popen(
        ["cargo", "run", "-p", "kernel", "--features", "preemption"],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        cwd=REPO_ROOT,
    )
    try:
        out, _ = p.communicate(timeout=timeout)
        return p.returncode or 0, out
    except subprocess.TimeoutExpired:
        p.kill()
        out, _ = p.communicate(timeout=5)
        cleanup()
        return 124, out


def ok(output: str) -> bool:
    for line in output.splitlines():
        m = PHASE_RE.search(line)
        if m:
            groups = m.groups()
            return groups[0] == "true" and groups[-1] == "true"
    return False


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--timeout", type=int, default=300)
    args = parser.parse_args()

    code, out = run_kernel(args.timeout)
    if code == 124:
        print("timeout", file=sys.stderr)
        return 1
    if not ok(out):
        print(out[-4000:], file=sys.stderr)
        return 1
    print("phase121_service_loader_check: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
