#!/usr/bin/env python3
"""Host check: ADR-0003 loader signed exec gate corpus (loader_signed_exec_lib only)."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ANCHOR = ROOT / "config" / "trust_anchor_epoch460_loader.toml"
CORPUS = ROOT / "config" / "loader_signed_exec"

sys.path.insert(0, str(Path(__file__).resolve().parent))
import loader_signed_exec_lib as lsel  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--timeout", type=int, default=30)
    ap.add_argument("--anchor", type=Path, default=ANCHOR)
    ap.add_argument("--corpus", type=Path, default=CORPUS)
    args = ap.parse_args()

    if not args.anchor.is_file():
        print(f"gate/loader_signed_exec: missing {args.anchor}", file=sys.stderr)
        return 1
    ok, msg = lsel.verify_corpus_dir(args.corpus, args.anchor)
    if not ok:
        print(f"gate/loader_signed_exec: FAIL {msg}", file=sys.stderr)
        return 1
    anchor = lsel.load_trust_anchor(args.anchor)
    print(
        f"gate/loader_signed_exec: OK (epoch={anchor.epoch}, corpus={args.corpus.name}, "
        "host verify)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
