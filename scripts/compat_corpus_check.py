#!/usr/bin/env python3
"""Fixed compat test corpus denominator per COMPAT_SUNSET.md."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CORPUS = ROOT / "compat_test_corpus.toml"


def main() -> int:
    if not CORPUS.exists():
        print("compat_corpus_check: missing compat_test_corpus.toml", file=sys.stderr)
        return 1
    text = CORPUS.read_text(encoding="utf-8")
    scenarios = text.count("[[scenarios]]")
    if scenarios < 3:
        print(f"compat_corpus_check: need >=3 scenarios, found {scenarios}", file=sys.stderr)
        return 1
    print(f"compat_corpus_check: OK ({scenarios} scenarios)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
