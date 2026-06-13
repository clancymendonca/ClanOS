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
    "smoke_shell_storage",
    "smoke_loader_security",
    "smoke_memory_layout",
    "smoke_userspace_bootstrap",
    "smoke_hw_paging",
    "smoke_sched_userspace",
    "smoke_dynamic_runtime",
    "smoke_fd_mmap",
    "smoke_vm_fork",
    "smoke_syscall_ring3",
    "smoke_path_exec",
    "smoke_smp_depth",
    "smoke_constitutional",
    "smoke_capabilities",
    "smoke_service_loader",
    "smoke_platform_brokers",
    "smoke_build_endpoints",
    "smoke_virtio_blk",
    "smoke_network_compat",
    "smoke_scheduler_epoch",
    "smoke_boundary",
    "boot_gate",
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
