#!/usr/bin/env python3
"""Host check: ext2 write path exists and is bounded."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
EXT2 = REPO / "kernel" / "src" / "ext2.rs"
VFS = REPO / "kernel" / "src" / "vfs.rs"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=30)
    _ = ap.parse_args()
    ext2_text = EXT2.read_text(encoding="utf-8")
    vfs_text = VFS.read_text(encoding="utf-8")
    if "pub fn write_file" not in ext2_text:
        print("gate/ext2_write: missing write_file", file=sys.stderr)
        return 1
    if "smoke_ext2_write" not in ext2_text:
        print("gate/ext2_write: missing smoke", file=sys.stderr)
        return 1
    if "WRITABLE_FILES" not in ext2_text:
        print("gate/ext2_write: missing allowlist", file=sys.stderr)
        return 1
    if "ext2::write_file" not in vfs_text:
        print("gate/ext2_write: vfs not wired", file=sys.stderr)
        return 1
    print("gate/ext2_write: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
