#!/usr/bin/env python3
"""Mark gap_registry gaps addressed when Epoch 0 fix docs exist on disk."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def parse_gaps(text: str) -> list[dict]:
    gaps: list[dict] = []
    current: dict | None = None
    for line in text.splitlines():
        if line.strip() == "[[gaps]]":
            if current:
                gaps.append(current)
            current = {}
            continue
        if current is not None and "=" in line:
            key, _, val = line.partition("=")
            key = key.strip()
            val = val.strip()
            if val.startswith('"""'):
                current[key] = val.strip('"')
            elif val == "null":
                current[key] = None
            elif val.startswith('"') and val.endswith('"'):
                current[key] = val[1:-1]
            else:
                try:
                    current[key] = int(val)
                except ValueError:
                    current[key] = val
    if current:
        gaps.append(current)
    return gaps


def doc_paths_from_fix(fix: str) -> list[str]:
    paths = re.findall(r"\((docs/[^)]+\.md)\)", fix)
    paths += re.findall(r"\[`([^`]+)`\]\((docs/[^)]+)\)", fix)
    # flatten tuples from second pattern
    flat: list[str] = []
    for p in paths:
        if isinstance(p, tuple):
            flat.append(p[1])
        else:
            flat.append(p)
    # root-level CHARTER etc
    for name in ("CHARTER.md", "SECURITY.md", "gap_registry.toml", "prereq_graph.toml"):
        if name in fix:
            flat.append(name)
    return list(dict.fromkeys(flat))


def emit_gaps(gaps: list[dict]) -> str:
    header = '''# AresOS gap registry — canonical lifecycle tracking
# Updated by scripts/mark_epoch0_addressed.py

[schema]
version = "1.0.0"
status_values = [
  "open",
  "addressed",
  "wontfix",
  "split-into",
  "superseded",
]
split_into_max_depth = 3

# superseded: gap moot due to design decision (not wontfix risk acceptance)
# superseded_by_commit must reference a commit touching DECISION_LOG or KERNEL_OBJECT_MODEL

'''
    blocks = []
    for g in gaps:
        blocks.append("[[gaps]]")
        for key in (
            "id",
            "summary",
            "fix",
            "when",
            "status",
            "addressing_commit",
            "implementing_doc",
            "split_into",
            "superseded_by_commit",
        ):
            val = g.get(key)
            if key == "split_into":
                blocks.append(f'{key} = {val if val else "[]"}')
            elif val is None:
                blocks.append(f"{key} = null")
            elif isinstance(val, int):
                blocks.append(f"{key} = {val}")
            else:
                blocks.append(f'{key} = """{val}"""')
        blocks.append("")
    return header + "\n".join(blocks)


def main() -> int:
    path = ROOT / "gap_registry.toml"
    text = path.read_text(encoding="utf-8")
    gaps = parse_gaps(text)
    marked = 0
    for g in gaps:
        when = g.get("when", "")
        if "Epoch 0" not in when and "epoch 0" not in when.lower():
            continue
        docs = doc_paths_from_fix(g.get("fix", ""))
        if not docs:
            continue
        if all((ROOT / d).exists() for d in docs):
            if g.get("status") == "open":
                g["status"] = "addressed"
                g["implementing_doc"] = docs[0]
                marked += 1
    path.write_text(emit_gaps(gaps), encoding="utf-8")
    open_count = sum(1 for g in gaps if g.get("status") == "open")
    print(f"marked {marked} gaps addressed; {open_count} still open")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
