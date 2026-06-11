#!/usr/bin/env python3
"""Verify every mapped path in config/README.md exists on disk."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
README = ROOT / "config" / "README.md"

# Markdown links: [`label`](../relative/path) — path chars only (avoids doc examples)
LINK = re.compile(r"\]\(\.\./([A-Za-z0-9_./-]+)\)")


def main() -> int:
    if not README.exists():
        print("config_readme_check: config/README.md missing", file=sys.stderr)
        return 1

    text = README.read_text(encoding="utf-8")
    errors: list[str] = []
    checked: set[str] = set()

    for rel in LINK.findall(text):
        rel = rel.strip()
        if rel in checked:
            continue
        checked.add(rel)
        if "create at" in rel.lower():
            continue
        target = (ROOT / rel).resolve()
        try:
            target.relative_to(ROOT.resolve())
        except ValueError:
            errors.append(f"config/README.md: path escapes repo: {rel}")
            continue
        if not target.exists():
            errors.append(f"config/README.md: mapped path missing: {rel}")

    if errors:
        for e in errors:
            print(f"error: {e}", file=sys.stderr)
        return 1

    print(f"config_readme_check: OK ({len(checked)} paths)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
