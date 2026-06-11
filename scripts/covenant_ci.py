#!/usr/bin/env python3
"""Enforce plan covenant CI rules (aresos_full_os_build plan lines 1614-1636)."""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

SCRIPTS = [
    "doc_link_check.py",
    "project_health.py",
    "cap_registry_sync.py",
    "epoch_signoff_check.py",
    "architecture_state_check.py",
    "compat_corpus_check.py",
    "fuzz_corpus_check.py",
    "unsafe_boundary_check.py",
    "count_ipc_bridge.py",
    "threat_node_lifecycle_check.py",
    "gap_registry_epoch_warn.py",
    "loom_gate.py",
    "transfer_toctou_check.py",
    "kani_gate.py",
    "phase_checklist_spot_check.py",
    "phase_smoke_host_check.py",
    "proof_cache_check.py",
    "release_scorecard_check.py",
]


def run_script(name: str) -> tuple[bool, str]:
    path = ROOT / "scripts" / name
    if not path.exists():
        return False, f"missing script {name}"
    proc = subprocess.run(
        [sys.executable, str(path)],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    out = (proc.stdout + proc.stderr).strip()
    return proc.returncode == 0, out or f"{name}: exit {proc.returncode}"


def check_charter() -> tuple[bool, str]:
    path = ROOT / "CHARTER.md"
    if not path.exists():
        return False, "CHARTER.md missing"
    return True, "CHARTER.md present"


def main() -> int:
    errors: list[str] = []
    ok, msg = check_charter()
    if not ok:
        errors.append(msg)
    else:
        print(msg)

    for name in SCRIPTS:
        passed, out = run_script(name)
        if passed:
            print(out.splitlines()[-1] if out else f"{name}: OK")
        else:
            errors.append(f"{name}: {out}")

    if errors:
        print("covenant_ci: FAIL", file=sys.stderr)
        for e in errors:
            print(f"  - {e}", file=sys.stderr)
        return 1
    print("covenant_ci: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
