#!/usr/bin/env python3
"""Spot-check scope-151..350 checklist stubs exist (gap 348)."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"


def main() -> int:
    missing: list[int] = []
    for scope in range(151, 351):
        path = DOCS / f"scope-{scope}-checklist.md"
        if not path.exists():
            missing.append(scope)
    if missing:
        print(
            f"scope_checklist_spot_check: missing {len(missing)} checklists "
            f"(first: {missing[:5]})",
            flush=True,
        )
        return 1
    print(f"scope_checklist_spot_check: OK (200 checklists 151-350)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
