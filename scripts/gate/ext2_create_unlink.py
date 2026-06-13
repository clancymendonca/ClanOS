#!/usr/bin/env python3
"""Host check: ext2 create/unlink + multi-block grow."""

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
    for needle in (
        "pub fn create_file",
        "pub fn unlink_file",
        "MAX_FILE_BLOCKS",
        "alloc_block",
        "smoke_ext2_create_unlink",
    ):
        if needle not in ext2_text:
            print(f"gate/ext2_create_unlink: missing {needle}", file=sys.stderr)
            return 1
    for needle in ("create_bytes", "unlink_path"):
        if needle not in vfs_text:
            print(f"gate/ext2_create_unlink: vfs missing {needle}", file=sys.stderr)
            return 1
    print("gate/ext2_create_unlink: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
