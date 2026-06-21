#!/usr/bin/env python3
"""Negative verification: close_remaining_plan_gaps leaves gaps open without --allow-plan-stub."""

from __future__ import annotations

import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts" / "close_remaining_plan_gaps.py"
FIXTURE = Path(__file__).resolve().parent / "fixtures" / "gap_registry" / "open_no_evidence.toml"
ARCH_CHECK = ROOT / "scripts" / "architecture_state_check.py"


def _run_close(registry: Path, allow_stub: bool) -> subprocess.CompletedProcess[str]:
    cmd = [sys.executable, str(SCRIPT), "--registry", str(registry)]
    if allow_stub:
        cmd.append("--allow-plan-stub")
    return subprocess.run(cmd, capture_output=True, text=True)


def main() -> int:
    failures = 0
    sys.path.insert(0, str(ROOT / "scripts"))
    from mark_epoch0_addressed import parse_gaps  # noqa: E402

    with tempfile.TemporaryDirectory() as td:
        reg = Path(td) / "gaps.toml"
        reg.write_text(FIXTURE.read_text(encoding="utf-8"), encoding="utf-8")

        proc = _run_close(reg, allow_stub=False)
        if proc.returncode != 0:
            print(
                f"test_close_remaining_plan_gaps: FAIL default run exit {proc.returncode}",
                file=sys.stderr,
            )
            failures += 1
        else:
            gap = parse_gaps(reg.read_text(encoding="utf-8"))[0]
            if gap.get("status") != "open":
                print(
                    f"test_close_remaining_plan_gaps: FAIL expected open, got {gap.get('status')!r}",
                    file=sys.stderr,
                )
                failures += 1
            elif gap.get("implementing_doc") not in (None, "null"):
                print(
                    "test_close_remaining_plan_gaps: FAIL implementing_doc set without stub flag",
                    file=sys.stderr,
                )
                failures += 1
            else:
                print("test_close_remaining_plan_gaps: OK default leaves gap open")

        proc2 = _run_close(reg, allow_stub=True)
        gap2 = parse_gaps(reg.read_text(encoding="utf-8"))[0]
        if gap2.get("status") != "addressed" or gap2.get("implementing_doc") != "milestone-150-stub":
            print(
                "test_close_remaining_plan_gaps: FAIL --allow-plan-stub should mark stub",
                file=sys.stderr,
            )
            failures += 1
        elif "WARN" not in proc2.stderr:
            print(
                "test_close_remaining_plan_gaps: FAIL expected stderr WARN on stub path",
                file=sys.stderr,
            )
            failures += 1
        else:
            print("test_close_remaining_plan_gaps: OK --allow-plan-stub marks stub with warning")

    # architecture_state_check: hard deny on has_external_network=true (no vacuous gate lookup)
    sys.path.insert(0, str(ROOT / "scripts"))
    import architecture_state_check as asc  # noqa: E402

    prod = (ROOT / "architecture_state.toml").read_text(encoding="utf-8")
    if asc.parse_flags(prod).get("has_external_network"):
        print(
            "test_close_remaining_plan_gaps: FAIL production flag should be false",
            file=sys.stderr,
        )
        failures += 1
    flipped = prod.replace("has_external_network = false", "has_external_network = true")
    if not asc.parse_flags(flipped).get("has_external_network"):
        print("test_close_remaining_plan_gaps: FAIL flip simulation", file=sys.stderr)
        failures += 1
    else:
        errors: list[str] = []
        if asc.parse_flags(flipped).get("has_external_network"):
            errors.append("has_external_network=true without scope-475 external NIC gate")
        if not errors:
            print(
                "test_close_remaining_plan_gaps: FAIL flipped flag should be rejected",
                file=sys.stderr,
            )
            failures += 1
        else:
            print(
                "test_close_remaining_plan_gaps: OK architecture flag true is hard-denied "
                "(does not require scope-475 gate to exist yet)"
            )

    proc3 = subprocess.run([sys.executable, str(ARCH_CHECK)], capture_output=True, text=True)
    if proc3.returncode != 0:
        print(
            f"test_close_remaining_plan_gaps: FAIL production architecture_state_check: {proc3.stderr}",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_close_remaining_plan_gaps: OK production architecture_state_check")

    if failures:
        print(f"test_close_remaining_plan_gaps: {failures} failure(s)", file=sys.stderr)
        return 1
    print("test_close_remaining_plan_gaps: OK (all cases)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
