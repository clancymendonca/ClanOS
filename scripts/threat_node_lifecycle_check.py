#!/usr/bin/env python3
"""Threat node lifecycle: no open nodes at milestone gate; depends_on respected."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def parse_nodes(text: str) -> list[dict]:
    nodes: list[dict] = []
    for block in re.split(r"\[\[nodes\]\]", text)[1:]:
        node: dict = {}
        for line in block.splitlines():
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            if line.startswith("depends_on = ["):
                inner = line[line.index("[") + 1 : line.index("]")]
                node["depends_on"] = [x.strip().strip('"') for x in inner.split(",") if x.strip()]
                continue
            if "=" not in line:
                continue
            key, _, val = line.partition("=")
            key = key.strip()
            val = val.strip().strip('"')
            node[key] = val
        if node.get("id"):
            nodes.append(node)
    return nodes


def main() -> int:
    text = (ROOT / "docs" / "THREAT_NODES.toml").read_text(encoding="utf-8")
    nodes = parse_nodes(text)
    by_id = {n["id"]: n for n in nodes}
    errors: list[str] = []
    open_nodes = [n for n in nodes if n.get("status") == "open"]
    if open_nodes:
        errors.append(f"{len(open_nodes)} open threat nodes: {[n['id'] for n in open_nodes]}")
    for n in nodes:
        if n.get("status") != "closed":
            continue
        for dep in n.get("depends_on", []):
            dep_node = by_id.get(dep)
            if dep_node and dep_node.get("status") != "closed":
                errors.append(f"{n['id']} closed but depends_on {dep} is {dep_node.get('status')}")
    if errors:
        for e in errors:
            print(f"threat_node_lifecycle_check: {e}", file=sys.stderr)
        return 1
    print(f"threat_node_lifecycle_check: OK ({len(nodes)} nodes, 0 open)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
