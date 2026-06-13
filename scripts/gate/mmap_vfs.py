#!/usr/bin/env python3
"""Host check: mmap uses VFS read path (not hardcoded /bin/hello only)."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
MMAP = REPO / "kernel" / "src" / "mmap.rs"
DEMAND = REPO / "kernel" / "src" / "demand_paging.rs"
VFS = REPO / "kernel" / "src" / "vfs.rs"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=30)
    _ = ap.parse_args()
    mmap_text = MMAP.read_text(encoding="utf-8")
    demand_text = DEMAND.read_text(encoding="utf-8")
    vfs_text = VFS.read_text(encoding="utf-8")
    if "read_bytes_for" not in mmap_text:
        print("gate/mmap_vfs: mmap missing vfs read", file=sys.stderr)
        return 1
    if "file_mapping_at" not in demand_text:
        print("gate/mmap_vfs: demand paging missing vma lookup", file=sys.stderr)
        return 1
    if "FILE_BACKED_PATH" in demand_text:
        print("gate/mmap_vfs: legacy global file path still present", file=sys.stderr)
        return 1
    if "smoke_mmap_vfs" not in mmap_text:
        print("gate/mmap_vfs: missing smoke", file=sys.stderr)
        return 1
    if "read_bytes_for" not in vfs_text:
        print("gate/mmap_vfs: vfs missing read_bytes_for", file=sys.stderr)
        return 1
    print("gate/mmap_vfs: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
