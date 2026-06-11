#!/usr/bin/env python3
"""Stub link + heading checker for epoch-0 docs — expand at epoch 0 gate CI."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"
STUB = re.compile(r"\[CROSS-REF:.*?— TBD\]", re.I)


def main() -> int:
    errors: list[str] = []
    for path in list(DOCS.glob("**/*.md")) + [ROOT / "CHARTER.md", ROOT / "SECURITY.md"]:
        if not path.exists():
            continue
        text = path.read_text(encoding="utf-8")
        if STUB.search(text):
            errors.append(f"{path.relative_to(ROOT)}: unresolved CROSS-REF TBD stub")
    if errors:
        for e in errors:
            print(f"error: {e}", file=sys.stderr)
        return 1
    print("doc_link_check: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
