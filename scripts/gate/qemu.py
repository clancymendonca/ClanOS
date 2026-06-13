"""QEMU serial-line smoke matcher."""

from __future__ import annotations

import os
import re
import subprocess
import sys
import threading
import time
from collections.abc import Callable
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]


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
    attempts: int = 3,
) -> int:
    phase_re = re.compile(pattern)
    last_tail: list[str] = []
    for attempt in range(1, attempts + 1):
        cleanup()
        if attempt > 1:
            time.sleep(3.0)
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
        last_tail = tail[-80:]
    print(f"{label}: timeout", file=sys.stderr)
    print("".join(last_tail), file=sys.stderr)
    return 1
