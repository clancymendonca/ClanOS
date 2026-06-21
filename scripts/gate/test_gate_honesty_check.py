#!/usr/bin/env python3
"""Negative verification for gate_honesty_check.py — fixtures must pass/fail as expected."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LINTER = ROOT / "scripts" / "gate" / "gate_honesty_check.py"
FIXTURES = Path(__file__).resolve().parent / "fixtures" / "gate_honesty"
GATE_RS = ROOT / "kernel" / "src" / "validation_gate.rs"


def _run(part: str, path: Path) -> int:
    proc = subprocess.run(
        [sys.executable, str(LINTER), "--part", part, "--file", str(path)],
        capture_output=True,
        text=True,
    )
    return proc.returncode


def main() -> int:
    cases: list[tuple[str, str, Path, int]] = [
        ("part A rejects trivial stub", "a", FIXTURES / "bad_stub.rs", 1),
        ("part B rejects smoke_ok shadowing", "b", FIXTURES / "bad_shadow.rs", 1),
        ("part B accepts accumulated smoke_ok", "b", FIXTURES / "good_shadow.rs", 0),
        ("part A accepts production gate file", "a", GATE_RS, 0),
        ("part B accepts production gate file", "b", GATE_RS, 0),
    ]
    failures = 0
    for label, part, path, want in cases:
        got = _run(part, path)
        if got != want:
            print(
                f"test_gate_honesty_check: FAIL {label}: want exit {want}, got {got}",
                file=sys.stderr,
            )
            failures += 1
        else:
            print(f"test_gate_honesty_check: OK {label}")
    if failures:
        print(f"test_gate_honesty_check: {failures} failure(s)", file=sys.stderr)
        return 1
    print("test_gate_honesty_check: OK (all cases)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
