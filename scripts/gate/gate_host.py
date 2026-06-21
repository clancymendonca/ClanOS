#!/usr/bin/env python3
"""Host-side unified validation gate check (no QEMU)."""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GATE = ROOT / "kernel" / "src" / "validation_gate.rs"
# Early-phase smokes (historically "boot gate" inventory).
EARLY_PHASE_SMOKES = [
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
]
# Late-phase smokes (historically "system gate" inventory).
LATE_PHASE_SMOKES = [
    "integrity_gate",
    "scheduling_gate",
    "hardware_gate",
    "federation_gate",
    "release_gate",
    "desktop_gate",
    "functional_gate",
    "smoke_compat_runtime",
    "smoke_compat_fd_vm",
    "smoke_compat_signal",
    "smoke_storage_depth",
    "smoke_posix_compat",
    "system_gate",
    "run_validation_gate",
]


def main() -> int:
    text = GATE.read_text(encoding="utf-8")
    version = re.search(r'VALIDATION_GATE_VERSION: &str = "([^"]+)"', text)
    if not version:
        print("gate/gate_host: VALIDATION_GATE_VERSION not found", file=sys.stderr)
        return 1
    for fn in EARLY_PHASE_SMOKES + LATE_PHASE_SMOKES:
        if fn not in text:
            print(f"gate/gate_host: missing {fn}", file=sys.stderr)
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
    print(
        f"gate/gate_host: OK (version={version.group(1)}, cargo check, gate fns present)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
