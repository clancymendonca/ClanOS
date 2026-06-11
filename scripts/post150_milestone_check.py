#!/usr/bin/env python3
"""Shared QEMU smoke matcher for post-150 milestone serial lines."""

from __future__ import annotations

import os
import re
import subprocess
import sys
import threading
import time
from collections.abc import Callable
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


def run_smoke(
    pattern: str,
    label: str,
    timeout: int = 300,
    features: list[str] | None = None,
    match_ok: Callable[[re.Match[str]], bool] | None = None,
) -> int:
    phase_re = re.compile(pattern)
    cleanup()
    cmd = ["cargo", "run", "-p", "kernel"]
    if features:
        cmd.extend(features)
    p = subprocess.Popen(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        cwd=REPO,
    )
    matched = threading.Event()
    tail: list[str] = []

    def reader() -> None:
        assert p.stdout is not None
        for line in p.stdout:
            tail.append(line)
            if len(tail) > 200:
                tail.pop(0)
            m = phase_re.search(line)
            if m:
                ok = match_ok(m) if match_ok else m.group(1) == "true"
                if ok:
                    matched.set()
                    break
        try:
            p.kill()
        except OSError:
            pass

    thread = threading.Thread(target=reader, daemon=True)
    thread.start()
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if matched.wait(timeout=0.25):
            cleanup()
            thread.join(timeout=5)
            print(f"{label}: OK")
            return 0
        if p.poll() is not None:
            break
    cleanup()
    thread.join(timeout=5)
    if p.poll() is None:
        try:
            p.kill()
        except OSError:
            pass
        print(f"{label}: timeout", file=sys.stderr)
    else:
        print(f"{label}: pattern not matched", file=sys.stderr)
    print("".join(tail[-80:]), file=sys.stderr)
    return 1
