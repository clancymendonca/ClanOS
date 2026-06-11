#!/usr/bin/env python3
"""Epoch 4 network smoke — virtio-net, compat sockets, functional broker (QEMU)."""

import os
import re
import subprocess
import sys
from pathlib import Path

PHASE_RE = re.compile(
    r"Phase404-Network:.*?tcp=(\d+).*?udp=(\d+).*?select=(\d+).*?ok=(true|false)"
)
REPO = Path(__file__).resolve().parents[1]


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
        cwd=REPO,
    )
    try:
        out, _ = p.communicate(timeout=300)
    except subprocess.TimeoutExpired:
        p.kill()
        cleanup()
        return 1
    for line in out.splitlines():
        m = PHASE_RE.search(line)
        if m and m.group(4) == "true":
            tcp, udp, sel = int(m.group(1)), int(m.group(2)), int(m.group(3))
            if tcp > 0 and udp > 0 and sel > 0:
                print("phase404_network_check: OK")
                return 0
    print(out[-3000:], file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
