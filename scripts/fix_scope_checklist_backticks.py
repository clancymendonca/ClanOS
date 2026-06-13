#!/usr/bin/env python3
"""Fix missing closing backticks on gate CLI lines in scope checklists."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "docs"
PAT = re.compile(
    r"^(- \[x\] `)(python scripts/gate/(?:boot|system)\.py --gate \S+ --timeout \d+)$"
)


def main() -> int:
    fixed = 0
    for path in sorted(ROOT.glob("scope-*-checklist.md")):
        lines_out: list[str] = []
        changed = False
        for line in path.read_text(encoding="utf-8").splitlines(keepends=True):
            body = line.rstrip("\r\n")
            suffix = line[len(body) :]
            m = PAT.match(body)
            if m:
                lines_out.append(m.group(1) + m.group(2) + "`" + suffix)
                changed = True
                fixed += 1
            else:
                lines_out.append(line)
        if changed:
            path.write_text("".join(lines_out), encoding="utf-8")
    print(f"fix_scope_checklist_backticks: {fixed} lines")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
