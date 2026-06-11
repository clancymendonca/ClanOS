#!/usr/bin/env python3
"""Validate epoch_signoffs/ manifests exist for epochs 0-14."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SIGNOFFS = ROOT / "epoch_signoffs"


def parse_signoff(path: Path) -> dict:
    text = path.read_text(encoding="utf-8")
    out: dict = {}
    for line in text.splitlines():
        if "=" not in line or line.strip().startswith("#"):
            continue
        key, _, val = line.partition("=")
        key = key.strip()
        val = val.strip().strip('"')
        out[key] = val
    return out


def main() -> int:
    errors: list[str] = []
    for epoch in range(15):
        path = SIGNOFFS / f"epoch-{epoch}.toml"
        if not path.exists():
            errors.append(f"missing {path.name}")
            continue
        data = parse_signoff(path)
        if "epoch" not in data and "phase" not in data:
            errors.append(f"{path.name}: missing epoch/phase field")
    if errors:
        for e in errors:
            print(f"epoch_signoff_check: {e}", file=sys.stderr)
        return 1
    print("epoch_signoff_check: OK (epochs 0-14)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
