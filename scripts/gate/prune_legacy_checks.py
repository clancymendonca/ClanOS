#!/usr/bin/env python3
"""Remove redundant legacy scope check wrapper scripts."""

from __future__ import annotations

from pathlib import Path

SCRIPTS = Path(__file__).resolve().parents[1]
KEEP = {
    "scope_checklist_spot_check.py",
}

removed = 0
for path in sorted(SCRIPTS.glob("scope*.py")):
    name = path.name
    if name in KEEP:
        continue
    if name.endswith("_check.py") or name.endswith("_milestone_check.py"):
        text = path.read_text(encoding="utf-8", errors="replace")
        if "Legacy scope" in text or "delegates to unified" in text or "delegates to" in text:
            path.unlink()
            removed += 1
            continue
        if "boot_gate_check.py" in text or "system_gate_check.py" in text:
            path.unlink()
            removed += 1

for obsolete in (
    "post150_milestone_check.py",
    "scope_smoke_host_check.py",
    "complete_scope.py",
    "boot_gate_redirect.py",
    "patch_remaining_docs.py",
    "preemption_telemetry.py",
):
    p = SCRIPTS / obsolete
    if p.exists():
        p.unlink()
        removed += 1
        print(f"removed {obsolete}")

print(f"prune_legacy_checks: removed {removed} scripts")
