#!/usr/bin/env python3
"""Capture QEMU VGA screendump and validate 1024×768 BGA RGB desktop (ADR-0004)."""

from __future__ import annotations

import argparse
import os
import shutil
import socket
import subprocess
import sys
import time
from pathlib import Path
from typing import IO, TextIO

_SCRIPTS = Path(__file__).resolve().parents[1]
if str(_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS))

from smoke_qemu import cleanup_qemu_processes, ensure_qemu_on_path

REPO = Path(__file__).resolve().parents[1]
IMG = REPO / "target" / "x86_64-unknown-none" / "debug" / "bootimage-kernel.bin"
OUT = REPO / "target" / "desktop_screendump.ppm"
SERIAL_LOG = REPO / "target" / "desktop_boot_serial.log"
BOOT_ATTEMPTS = 3
# Measured on pinned gate corpus (2026-06-21): full validation gate + shell ready ~24s
# (5 consecutive screendump launches: min 23.8s, max 24.4s). Retries cover transient
# QEMU/monitor failures — not slow boot — so each attempt gets the full `--timeout`.
MEASURED_BOOT_READY_P99_S = 45

EXPECTED_W = 1024
EXPECTED_H = 768
TASKBAR_H = 54

READY_MARKERS = (
    "ClanOS shell ready",
)


def cleanup() -> None:
    cleanup_qemu_processes()


def screendump_via_monitor(port: int, out_path: Path, timeout: float = 30.0) -> bool:
    dump_arg = out_path.relative_to(REPO).as_posix()
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        try:
            with socket.create_connection(("127.0.0.1", port), timeout=2) as sock:
                sock.sendall(f"screendump {dump_arg}\n".encode())
                time.sleep(1.0)
                if out_path.exists() and out_path.stat().st_size > 1000:
                    return True
        except OSError:
            time.sleep(0.5)
    return out_path.exists() and out_path.stat().st_size > 1000


def analyze_ppm(path: Path) -> tuple[bool, str]:
    data = path.read_bytes()
    if not data.startswith(b"P6"):
        return False, "not P6 PPM"
    header_end = data.find(b"\n255\n")
    if header_end < 0:
        return False, "malformed PPM header"
    header = data[3:header_end].decode("ascii", errors="replace")
    parts = header.split()
    if len(parts) < 2:
        return False, "missing width/height"
    file_w, file_h = int(parts[0]), int(parts[1])
    if file_w < EXPECTED_W or file_h < EXPECTED_H:
        return False, f"expected>={EXPECTED_W}x{EXPECTED_H} got {file_w}x{file_h}"

    pixels = data[header_end + 5 :]
    width = EXPECTED_W
    height = EXPECTED_H
    stride = file_w * 3

    def rgb_at(x: int, y: int) -> tuple[int, int, int]:
        i = y * stride + x * 3
        return pixels[i], pixels[i + 1], pixels[i + 2]

    # Sample desktop chrome + window regions (layout from desktop_shell.rs, ~3× scale).
    probe_points = [
        (512, 384),  # desktop center
        (512, height - TASKBAR_H // 2),  # taskbar
        (120, 120),  # console window body
        (600, 150),  # files window body
        (120, 72),   # console title bar
    ]
    probe_colors = [rgb_at(x, y) for x, y in probe_points]
    distinct = len(set(probe_colors))
    if distinct < 3:
        return False, f"insufficient color variance distinct={distinct} probes={probe_colors}"

    center = probe_colors[0]
    taskbar = probe_colors[1]
    if center == taskbar:
        return False, "taskbar/desktop indistinguishable"

    def col_score(x: int) -> float:
        diffs = 0
        samples_n = 0
        for y in range(1, height - 1):
            i = y * stride + x * 3
            prev = pixels[i - stride : i - stride + 3]
            cur = pixels[i : i + 3]
            nxt = pixels[i + stride : i + stride + 3]
            for a, b in zip(cur, prev):
                diffs += abs(a - b)
            for a, b in zip(cur, nxt):
                diffs += abs(a - b)
            samples_n += 6
        return diffs / max(samples_n, 1)

    scores = [col_score(x) for x in range(width)]
    avg = sum(scores) / len(scores)
    hot = sum(1 for s in scores if s > avg * 2.5)
    ok = hot < width * 0.15
    return ok, (
        f"surface={width}x{height} distinct={distinct} "
        f"columns_high_vertical_delta={hot}/{width} avg_delta={avg:.2f}"
    )


def wait_for_boot(
    proc: subprocess.Popen[bytes],
    serial_out: IO[str],
    timeout: int,
) -> bool:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        serial_out.flush()
        text = SERIAL_LOG.read_text(encoding="utf-8", errors="replace")
        if any(marker in text for marker in READY_MARKERS):
            return True
        if proc.poll() is not None:
            return any(marker in text for marker in READY_MARKERS)
        time.sleep(0.25)
    serial_out.flush()
    text = SERIAL_LOG.read_text(encoding="utf-8", errors="replace")
    return any(marker in text for marker in READY_MARKERS)


def launch_qemu(monitor_port: int) -> tuple[subprocess.Popen[bytes], TextIO]:
    if not ensure_qemu_on_path():
        raise FileNotFoundError("qemu-system-x86_64 not found on PATH")
    qemu = shutil.which("qemu-system-x86_64")
    if qemu is None:
        raise FileNotFoundError("qemu-system-x86_64 not found on PATH")
    drive_path = IMG.relative_to(REPO).as_posix()
    monitor = f"tcp:127.0.0.1:{monitor_port},server,nowait"
    cmd = [
        qemu,
        "-drive",
        f"format=raw,file={drive_path}",
        "-serial",
        "stdio",
        "-display",
        "none",
        "-vga",
        "std",
        "-machine",
        "pc",
        "-monitor",
        monitor,
        "-no-reboot",
    ]
    serial_out = SERIAL_LOG.open("w", encoding="utf-8", buffering=1)
    proc = subprocess.Popen(
        cmd,
        stdout=serial_out,
        stderr=subprocess.STDOUT,
        cwd=REPO,
        env=os.environ.copy(),
    )
    return proc, serial_out


def boot_qemu_with_retries(
    base_port: int, timeout: int
) -> tuple[tuple[subprocess.Popen[bytes], int], float] | None:
    if timeout < MEASURED_BOOT_READY_P99_S:
        print(
            f"desktop_screendump_check: warn timeout={timeout}s below measured p99 "
            f"boot budget {MEASURED_BOOT_READY_P99_S}s",
            file=sys.stderr,
        )
    per_attempt = timeout
    for attempt in range(BOOT_ATTEMPTS):
        if attempt:
            cleanup()
            time.sleep(3.0)
        SERIAL_LOG.unlink(missing_ok=True)
        monitor_port = base_port + attempt
        proc, serial_out = launch_qemu(monitor_port)
        attempt_start = time.monotonic()
        try:
            if wait_for_boot(proc, serial_out, per_attempt):
                boot_elapsed = time.monotonic() - attempt_start
                return (proc, monitor_port), boot_elapsed
        finally:
            serial_out.close()
        try:
            proc.kill()
        except OSError:
            pass
    return None


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=120)
    ap.add_argument("--monitor-port", type=int, default=55555)
    args = ap.parse_args()

    if not IMG.exists():
        print(f"desktop_screendump_check: missing {IMG}", file=sys.stderr)
        return 2

    cleanup()
    time.sleep(3.0)
    OUT.unlink(missing_ok=True)
    SERIAL_LOG.unlink(missing_ok=True)

    boot = boot_qemu_with_retries(args.monitor_port, args.timeout)
    if boot is None:
        tail = ""
        if SERIAL_LOG.exists():
            tail = SERIAL_LOG.read_text(encoding="utf-8", errors="replace")[-4000:]
        cleanup()
        print("desktop_screendump_check: boot timeout", file=sys.stderr)
        if tail:
            print(tail, file=sys.stderr)
        return 1

    (proc, monitor_port), boot_elapsed = boot

    time.sleep(2.0)
    dump_path = REPO / "target" / "desktop_screendump.ppm"
    if not screendump_via_monitor(monitor_port, dump_path):
        cleanup()
        proc.kill()
        print("desktop_screendump_check: screendump failed", file=sys.stderr)
        return 1

    ok, detail = analyze_ppm(dump_path)
    cleanup()
    try:
        proc.kill()
    except OSError:
        pass

    if ok:
        print(
            f"desktop_screendump_check: OK ({detail} boot_elapsed={boot_elapsed:.1f}s)"
        )
        print(f"screendump: {dump_path}")
        return 0
    print(f"desktop_screendump_check: FAIL ({detail})", file=sys.stderr)
    print(f"screendump: {dump_path}", file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
