#!/usr/bin/env python3
"""Host-side boot gate validation when QEMU is unavailable."""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GATE = ROOT / "kernel" / "src" / "boot_gate.rs"
GATES = [
    "eval_shell_storage",
    "eval_loader_security",
    "eval_memory_layout",
    "eval_userspace_bootstrap",
    "exec_phase21_to_30_smokes",
    "exec_phase31_to_40_smokes",
    "exec_phase41_to_50_smokes",
    "exec_phase51_to_60_smokes",
    "exec_phase61_to_70_smokes",
    "exec_phase71_to_80_smokes",
    "exec_phase81_to_90_smokes",
    "exec_phase91_to_100_smokes",
    "exec_phase101_to_110_smokes",
    "exec_phase111_to_120_smokes",
    "exec_phase121_smoke",
    "exec_phase122_to_130_smokes",
    "exec_phase131_to_140_smokes",
    "exec_phase201_virtio_smoke",
    "exec_epoch4_network_smokes",
    "exec_epoch5_scheduler_smokes",
    "exec_milestone150",
    "run_boot_gate",
]


def main() -> int:
    text = GATE.read_text(encoding="utf-8")
    version = re.search(r'BOOT_GATE_VERSION: &str = "([^"]+)"', text)
    if not version:
        print("gate/boot_host: BOOT_GATE_VERSION not found", file=sys.stderr)
        return 1
    for fn in GATES:
        if fn not in text:
            print(f"gate/boot_host: missing {fn}", file=sys.stderr)
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
    print(f"gate/boot_host: OK (version={version.group(1)}, cargo check, gate fns present)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
