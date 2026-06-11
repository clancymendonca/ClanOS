#!/usr/bin/env python3
"""Host-side milestone validation when QEMU is unavailable."""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CATALOG = ROOT / "kernel" / "src" / "phase_catalog.rs"
MILESTONES = [
    ("Phase175-Epoch7", "phase175_epoch7_smoke"),
    ("Phase200-Milestone", "phase200_milestone_smoke"),
    ("Phase250-Milestone", "phase250_milestone_smoke"),
    ("Phase300-Milestone", "phase300_milestone_smoke"),
    ("Phase350-Milestone", "phase350_milestone_smoke"),
]


def completed_phase() -> int:
    text = CATALOG.read_text(encoding="utf-8")
    m = re.search(r"pub const COMPLETED_PHASE: u32 = (\d+);", text)
    if not m:
        raise SystemExit("COMPLETED_PHASE not found")
    return int(m.group(1))


def main() -> int:
    phase = completed_phase()
    if phase < 350:
        print(f"phase_smoke_host_check: COMPLETED_PHASE={phase}, need 350", file=sys.stderr)
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
    post150 = (ROOT / "kernel" / "src" / "post150.rs").read_text(encoding="utf-8")
    for _label, fn in MILESTONES:
        if fn not in post150:
            print(f"phase_smoke_host_check: missing {fn}", file=sys.stderr)
            return 1
    print(f"phase_smoke_host_check: OK (COMPLETED_PHASE={phase}, cargo check, milestone fns present)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
