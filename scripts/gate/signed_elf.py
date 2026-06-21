#!/usr/bin/env python3
"""Host check: ADR-0002 signed ELF gate corpus + trust anchor (host reference; kernel path in signed_elf.rs)."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ANCHOR = ROOT / "config" / "trust_anchor_epoch450.toml"
CORPUS = ROOT / "config" / "signed_elf_test_corpus"

sys.path.insert(0, str(Path(__file__).resolve().parent))
import signed_elf_lib as sel  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--timeout", type=int, default=30)
    ap.add_argument("--anchor", type=Path, default=ANCHOR)
    ap.add_argument("--corpus", type=Path, default=CORPUS)
    args = ap.parse_args()

    if not args.anchor.is_file():
        print(f"gate/signed_elf: missing {args.anchor}", file=sys.stderr)
        return 1
    ok, msg = sel.verify_corpus_dir(args.corpus, args.anchor)
    if not ok:
        print(f"gate/signed_elf: FAIL {msg}", file=sys.stderr)
        return 1
    anchor = sel.load_trust_anchor(args.anchor)
    print(
        f"gate/signed_elf: OK (epoch={anchor.epoch}, corpus={args.corpus.name}, "
        "host verify)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
