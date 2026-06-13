#!/usr/bin/env python3
"""Host-side system gate validation when QEMU is unavailable."""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GATE = ROOT / "kernel" / "src" / "system_gate.rs"
GATES = [
    "integrity_gate",
    "scheduling_gate",
    "hardware_gate",
    "federation_gate",
    "release_gate",
    "desktop_gate",
    "functional_gate",
    "system_gate",
]


def main() -> int:
    text = GATE.read_text(encoding="utf-8")
    version = re.search(r'SYSTEM_GATE_VERSION: &str = "([^"]+)"', text)
    if not version:
        print("gate/system_host: SYSTEM_GATE_VERSION not found", file=sys.stderr)
        return 1
    for fn in GATES:
        if fn not in text:
            print(f"gate/system_host: missing {fn}", file=sys.stderr)
            return 1
    proc = subprocess.run(
        ["cargo", "check", "-p", "kernel"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        print(proc.stdout + proc.stderr, file=sys.stderr)
        return 1
    print(f"gate/system_host: OK (version={version.group(1)}, cargo check, gate fns present)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
