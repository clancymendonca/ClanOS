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
_SCRIPTS = Path(__file__).resolve().parents[1]
if str(_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS))

from smoke_qemu import (
    OBJCOPY_RETRY_SLEEP_S,
    ensure_qemu_on_path,
    is_bootimage_build_error,
    is_objcopy_lock_error,
    wait_for_bootimage_unlock,
)


def cleanup() -> None:
    wait_for_bootimage_unlock()


def _emit(line: str) -> None:
    sys.stdout.write(line)
    if not line.endswith("\n"):
        sys.stdout.write("\n")
    sys.stdout.flush()


def run_smoke(
    pattern: str,
    label: str,
    timeout: int = 300,
    features: list[str] | None = None,
    match_ok: Callable[[re.Match[str]], bool] | None = None,
    attempts: int = 3,
) -> int:
    gate_re = re.compile(pattern)
    last_tail: list[str] = []
    last_output = ""

    for attempt in range(1, attempts + 1):
        if attempt > 1:
            _emit(
                f"{label}: retry {attempt}/{attempts} "
                f"(waiting {OBJCOPY_RETRY_SLEEP_S}s for bootimage lock)..."
            )
            time.sleep(OBJCOPY_RETRY_SLEEP_S)
        cleanup()
        ensure_qemu_on_path()

        cmd = ["cargo", "run", "-p", "kernel"]
        if features:
            cmd.extend(features)
        p = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1,
            cwd=REPO,
        )
        matched = threading.Event()
        failed = threading.Event()
        tail: list[str] = []
        output_chunks: list[str] = []

        def reader() -> None:
            assert p.stdout is not None
            for line in p.stdout:
                output_chunks.append(line)
                tail.append(line)
                if len(tail) > 200:
                    tail.pop(0)
                _emit(line.rstrip("\n"))
                m = gate_re.search(line)
                if m:
                    ok = match_ok(m) if match_ok else m.group(1) == "true"
                    if ok:
                        matched.set()
                    else:
                        failed.set()
                    break
            try:
                if p.poll() is None:
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
                _emit(f"{label}: OK")
                return 0
            if failed.wait(timeout=0.0):
                cleanup()
                thread.join(timeout=5)
                print(f"{label}: gate reported ok=false", file=sys.stderr)
                print("".join(tail[-40:]), file=sys.stderr)
                return 1
            if p.poll() is not None:
                break

        cleanup()
        thread.join(timeout=5)
        if p.poll() is None:
            try:
                p.kill()
            except OSError:
                pass
        thread.join(timeout=2)

        last_tail = tail[-80:]
        last_output = "".join(output_chunks)
        if is_objcopy_lock_error(last_output) or is_bootimage_build_error(last_output):
            if attempt < attempts:
                continue
            print(f"{label}: bootimage build failed (file lock)", file=sys.stderr)
            print(last_output[-2000:], file=sys.stderr)
            return 1

        if attempt < attempts and not matched.is_set() and not failed.is_set():
            continue

    print(f"{label}: timeout after {timeout}s", file=sys.stderr)
    print("".join(last_tail), file=sys.stderr)
    return 1
