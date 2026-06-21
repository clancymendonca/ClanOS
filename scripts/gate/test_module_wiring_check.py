#!/usr/bin/env python3
"""Self-test for module_wiring_check.py — inventory size and path discipline."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CHECK = ROOT / "scripts" / "gate" / "module_wiring_check.py"
GATE_AUDIT = ROOT / "docs" / "GATE_AUDIT.md"


def main() -> int:
    failures = 0

    # Import inventory constants without running main().
    sys.path.insert(0, str(CHECK.parent))
    try:
        import module_wiring_check as mwc  # type: ignore[import-not-found]
    finally:
        sys.path.pop(0)

    if len(mwc.KNOWN_DEAD_SOURCES) != mwc.EXPECTED_KNOWN_DEAD_COUNT:
        print(
            "test_module_wiring_check: FAIL inventory length mismatch",
            file=sys.stderr,
        )
        failures += 1
    else:
        print(
            f"test_module_wiring_check: OK inventory length == "
            f"{mwc.EXPECTED_KNOWN_DEAD_COUNT}"
        )

    for err in mwc.validate_known_dead_inventory():
        print(f"test_module_wiring_check: FAIL {err}", file=sys.stderr)
        failures += 1

    audit_text = GATE_AUDIT.read_text(encoding="utf-8")
    dead_section = audit_text.split("## Dead source inventory", 1)[-1].split("##", 1)[0]
    table_rows = [
        line
        for line in dead_section.splitlines()
        if line.startswith("| `kernel/src/")
    ]
    if len(table_rows) != mwc.EXPECTED_KNOWN_DEAD_COUNT:
        print(
            f"test_module_wiring_check: FAIL GATE_AUDIT.md lists {len(table_rows)} "
            f"dead-source rows; expected {mwc.EXPECTED_KNOWN_DEAD_COUNT}",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_module_wiring_check: OK GATE_AUDIT.md row count matches inventory")

    proc = subprocess.run(
        [sys.executable, str(CHECK)],
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        print(
            "test_module_wiring_check: FAIL module_wiring_check.py exited "
            f"{proc.returncode}",
            file=sys.stderr,
        )
        if proc.stdout:
            print(proc.stdout, file=sys.stderr)
        if proc.stderr:
            print(proc.stderr, file=sys.stderr)
        failures += 1
    else:
        print("test_module_wiring_check: OK production wiring check")

    if failures:
        print(f"test_module_wiring_check: {failures} failure(s)", file=sys.stderr)
        return 1
    print("test_module_wiring_check: OK (all cases)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
