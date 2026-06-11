#!/usr/bin/env python3
"""Spot-check phase-151..350 checklist stubs exist (gap 348)."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"


def main() -> int:
    missing: list[int] = []
    for phase in range(151, 351):
        path = DOCS / f"phase-{phase}-checklist.md"
        if not path.exists():
            missing.append(phase)
    if missing:
        print(
            f"phase_checklist_spot_check: missing {len(missing)} checklists "
            f"(first: {missing[:5]})",
            flush=True,
        )
        return 1
    print(f"phase_checklist_spot_check: OK (200 checklists 151-350)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
