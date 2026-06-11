#!/usr/bin/env python3
"""Build ares-rt demo and stage for QEMU FS install hook (epoch 2)."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
OUT = REPO / "target" / "userland-staging"
DEMO = OUT / "demo-hello.txt"


def host_target() -> str:
    import platform

    machine = platform.machine().lower()
    system = platform.system().lower()
    if system == "windows":
        return "x86_64-pc-windows-msvc"
    if system == "darwin":
        return "aarch64-apple-darwin" if "arm" in machine else "x86_64-apple-darwin"
    return "x86_64-unknown-linux-gnu"


def main() -> int:
    target = host_target()
    subprocess.check_call(
        [
            "cargo",
            "build",
            "-p",
            "ares-rt",
            "--bin",
            "demo-hello",
            "--target",
            target,
        ],
        cwd=REPO,
    )
    OUT.mkdir(parents=True, exist_ok=True)
    DEMO.write_text(
        "ares-rt staged artifact — replace with ring3 ELF install in later epoch\n",
        encoding="utf-8",
    )
    print(f"install_userland: staged {DEMO}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
