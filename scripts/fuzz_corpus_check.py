#!/usr/bin/env python3
"""Fuzz corpus hash gate per FUZZ_TARGETS.md (stub graduation)."""

from __future__ import annotations

import hashlib
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CORPUS_DIR = ROOT / "fuzz" / "corpus"
HASH_FILE = ROOT / "fuzz" / "corpus.sha256"


def corpus_hash() -> str:
    h = hashlib.sha256()
    if not CORPUS_DIR.exists():
        return h.hexdigest()
    for path in sorted(CORPUS_DIR.rglob("*")):
        if path.is_file():
            h.update(path.name.encode())
            h.update(path.read_bytes())
    return h.hexdigest()


def main() -> int:
    if not HASH_FILE.exists():
        print("fuzz_corpus_check: missing fuzz/corpus.sha256 (run fuzz init)", file=sys.stderr)
        return 1
    expected = HASH_FILE.read_text(encoding="utf-8").strip().split()[0]
    actual = corpus_hash()
    if expected != actual:
        print(f"fuzz_corpus_check: hash mismatch expected={expected[:16]} actual={actual[:16]}", file=sys.stderr)
        return 1
    print("fuzz_corpus_check: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
