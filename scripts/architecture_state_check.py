#!/usr/bin/env python3
"""Deferred threat reopen triggers vs architecture_state.toml flags."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

TRIGGER_MAP = {
    "has_real_hardware_target": "T-physical-access",
    "has_persisted_cap_state": "T-checkpoint-persisted-caps",
}


def parse_flags(text: str) -> dict[str, bool]:
    flags: dict[str, bool] = {}
    block = re.search(r"\[flags\](.*?)(?:\n\[|\Z)", text, re.S)
    if not block:
        return flags
    for line in block.group(1).splitlines():
        if "=" not in line:
            continue
        key, _, val = line.partition("=")
        flags[key.strip()] = val.strip() == "true"
    return flags


def parse_deferred_nodes(text: str) -> dict[str, str | None]:
    nodes: dict[str, str | None] = {}
    for m in re.finditer(
        r'\[\[nodes\]\].*?id = "([^"]+)".*?status = "deferred".*?(?:reopen_trigger = "([^"]*)"|reopen_trigger = null)',
        text,
        re.S,
    ):
        nodes[m.group(1)] = m.group(2) or None
    return nodes


def main() -> int:
    arch = (ROOT / "architecture_state.toml").read_text(encoding="utf-8")
    threats = (ROOT / "docs" / "THREAT_NODES.toml").read_text(encoding="utf-8")
    flags = parse_flags(arch)
    deferred = parse_deferred_nodes(threats)
    errors: list[str] = []
    for flag, node_id in TRIGGER_MAP.items():
        if flags.get(flag) and node_id in deferred:
            errors.append(f"{flag}=true but {node_id} still deferred without re-eval commit")
    if errors:
        for e in errors:
            print(f"architecture_state_check: {e}", file=sys.stderr)
        return 1
    print("architecture_state_check: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
