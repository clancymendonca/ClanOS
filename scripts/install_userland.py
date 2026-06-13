#!/usr/bin/env python3
"""Build Clan OS runtime demo and Mendo; stage for QEMU FS install hook."""

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
            "clan-rt",
            "--bin",
            "demo-hello",
            "--target",
            target,
        ],
        cwd=REPO,
    )
    subprocess.check_call(
        [
            "cargo",
            "build",
            "-p",
            "mendo",
            "--release",
            "--target",
            "x86_64-unknown-none",
        ],
        cwd=REPO,
    )
    OUT.mkdir(parents=True, exist_ok=True)
    DEMO.write_text(
        "clan-rt staged artifact — Clan OS host demo; ring-3 ELFs use /bin/mendo.elf\n",
        encoding="utf-8",
    )
    mendo_elf = (
        REPO / "target" / "x86_64-unknown-none" / "release" / "mendo"
    )
    staged_mendo = OUT / "mendo.elf"
    if mendo_elf.is_file():
        staged_mendo.write_bytes(mendo_elf.read_bytes())
        print(f"install_userland: staged {staged_mendo} ({staged_mendo.stat().st_size} bytes)")
    print(f"install_userland: staged {DEMO}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
