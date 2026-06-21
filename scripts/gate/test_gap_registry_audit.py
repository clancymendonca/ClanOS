#!/usr/bin/env python3
"""Self-test for gap_registry_audit.py — fixture must fail; production must pass baseline."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
AUDIT = ROOT / "scripts" / "gate" / "gap_registry_audit.py"
BAD = Path(__file__).resolve().parent / "fixtures" / "gap_registry" / "bad_overclaimed.toml"
REGISTRY = ROOT / "gap_registry.toml"


def _run(extra: list[str]) -> int:
    proc = subprocess.run(
        [sys.executable, str(AUDIT), *extra],
        capture_output=True,
        text=True,
    )
    return proc.returncode


def main() -> int:
    failures = 0

    sys.path.insert(0, str(AUDIT.parent))
    try:
        import gap_registry_audit as gra  # type: ignore[import-not-found]
    finally:
        sys.path.pop(0)

    gaps = gra.parse_gaps(BAD.read_text(encoding="utf-8"))
    over, counts = gra.audit_gaps(gaps)
    if len(over) != 1 or counts.get("Overclaimed") != 1:
        print(
            f"test_gap_registry_audit: FAIL fixture classification "
            f"over={len(over)} counts={dict(counts)}",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_gap_registry_audit: OK fixture classified overclaimed")

    if _run(["--file", str(BAD), "--strict"]) != 1:
        print("test_gap_registry_audit: FAIL --strict on fixture should exit 1", file=sys.stderr)
        failures += 1
    else:
        print("test_gap_registry_audit: OK --strict rejects fixture")

    if _run(["--file", str(REGISTRY)]) != 0:
        print("test_gap_registry_audit: FAIL production registry baseline check", file=sys.stderr)
        failures += 1
    else:
        print("test_gap_registry_audit: OK production registry within baseline")

    prod_gaps = gra.parse_gaps(REGISTRY.read_text(encoding="utf-8"))
    n_stub = gra.count_milestone_150_stub(prod_gaps)
    if n_stub != gra.EXPECTED_MILESTONE_150_STUB_BASELINE:
        print(
            f"test_gap_registry_audit: FAIL milestone-150-stub count {n_stub} != "
            f"baseline {gra.EXPECTED_MILESTONE_150_STUB_BASELINE}",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_gap_registry_audit: OK milestone-150-stub baseline")

    if failures:
        print(f"test_gap_registry_audit: {failures} failure(s)", file=sys.stderr)
        return 1
    print("test_gap_registry_audit: OK (all cases)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
