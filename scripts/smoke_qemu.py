#!/usr/bin/env python3
"""Shared helpers for QEMU serial smoke checks."""

from __future__ import annotations

import os
import shutil
import signal
import subprocess
import time
from pathlib import Path

DEFAULT_SMOKE_TIMEOUT = 120
OBJCOPY_RETRY_SLEEP_S = 12
WINDOWS_ARTIFACT_UNLOCK_S = 5


def is_objcopy_lock_error(output: str) -> bool:
    lowered = output.lower()
    return "llvm-objcopy" in lowered and "permission denied" in lowered


def is_bootimage_build_error(output: str) -> bool:
    lowered = output.lower()
    return "bootimage" in lowered and (
        "permission denied" in lowered or "failed to run" in lowered
    )


def wait_for_bootimage_unlock() -> None:
    """Release Windows file locks on bootimage artifacts before the next cargo run."""
    cleanup_qemu_processes()
    if os.name == "nt":
        time.sleep(WINDOWS_ARTIFACT_UNLOCK_S)

# QEMU serial/display/no-reboot come from [package.metadata.bootimage] run-command.
KERNEL_FEATURES = ["--features", "preemption"]
KERNEL_BUILD_CMD = ["cargo", "build", "-p", "kernel", *KERNEL_FEATURES]
KERNEL_CMD = ["cargo", "run", "-p", "kernel", *KERNEL_FEATURES]


def ensure_preemption_kernel_built() -> None:
    """Rebuild kernel with preemption after non-preemption gate smokes may have clobbered artifacts."""
    wait_for_bootimage_unlock()
    subprocess.run(
        KERNEL_BUILD_CMD,
        check=True,
        env=os.environ.copy(),
    )


def _qemu_candidate_dirs() -> list[Path]:
    dirs: list[Path] = []
    if os.name == "nt":
        pf = os.environ.get("ProgramFiles", r"C:\Program Files")
        pf86 = os.environ.get("ProgramFiles(x86)", r"C:\Program Files (x86)")
        local = os.environ.get("LOCALAPPDATA", "")
        dirs.extend(
            [
                Path(pf) / "qemu",
                Path(pf86) / "qemu",
                Path(local) / "Programs" / "QEMU",
            ]
        )
        winget_root = Path(local) / "Microsoft" / "WinGet" / "Packages"
        if winget_root.is_dir():
            dirs.extend(winget_root.glob("*QEMU*"))
    else:
        dirs.extend(
            [
                Path("/usr/bin"),
                Path("/usr/local/bin"),
            ]
        )
    return dirs


def find_qemu_bin_dir() -> Path | None:
    """Return directory containing `qemu-system-x86_64` when not already on PATH."""
    if shutil.which("qemu-system-x86_64"):
        return None
    exe_name = "qemu-system-x86_64.exe" if os.name == "nt" else "qemu-system-x86_64"
    for base in _qemu_candidate_dirs():
        direct = base / exe_name
        if direct.is_file():
            return base
        if base.is_dir():
            for hit in base.rglob(exe_name):
                return hit.parent
    return None


def ensure_qemu_on_path() -> bool:
    """Prepend a discovered QEMU install dir to PATH (Windows winget/NSIS layouts)."""
    if shutil.which("qemu-system-x86_64"):
        return True
    qdir = find_qemu_bin_dir()
    if qdir is None:
        return False
    os.environ["PATH"] = str(qdir) + os.pathsep + os.environ.get("PATH", "")
    return shutil.which("qemu-system-x86_64") is not None


def cleanup_qemu_processes() -> None:
    if os.name == "nt":
        subprocess.run(
            ["taskkill", "/IM", "qemu-system-x86_64.exe", "/F"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )
    else:
        subprocess.run(
            ["pkill", "-9", "qemu-system-x86_64"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )


def run_kernel(timeout: int = DEFAULT_SMOKE_TIMEOUT) -> tuple[int, str]:
    cleanup_qemu_processes()
    ensure_qemu_on_path()
    ensure_preemption_kernel_built()
    process = subprocess.Popen(
        KERNEL_CMD,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        env=os.environ.copy(),
    )
    try:
        output, _ = process.communicate(timeout=timeout)
        return process.returncode or 0, output
    except subprocess.TimeoutExpired:
        process.send_signal(signal.SIGTERM)
        try:
            output, _ = process.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill()
            output, _ = process.communicate(timeout=5)
        cleanup_qemu_processes()
        return 124, output


def kernel_exit_ok(code: int) -> bool:
    return code in (0, 124)
