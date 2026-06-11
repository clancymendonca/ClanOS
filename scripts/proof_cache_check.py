#!/usr/bin/env python3
"""Proof cache key gate (gap #256) — harness registry + lockfile present."""

from __future__ import annotations

import hashlib
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FILES = [
    ROOT / "kani_harness_registry.toml",
    ROOT / "Cargo.lock",
    ROOT / "proof-rights" / "Cargo.toml",
]


def main() -> int:
    h = hashlib.sha256()
    for path in FILES:
        if not path.exists():
            print(f"proof_cache_check: missing {path.name}", file=sys.stderr)
            return 1
        h.update(path.read_bytes())
    print(f"proof_cache_check: OK (cache_key={h.hexdigest()[:16]})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
