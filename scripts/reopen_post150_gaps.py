#!/usr/bin/env python3
"""Re-open post-150 wontfix gaps with epoch assignments per plan gap 350."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
from mark_epoch0_addressed import emit_gaps, parse_gaps  # noqa: E402

# gap_id -> target epoch for when field
EPOCH_MAP: dict[int, int] = {
    33: 13,   # Verus triggers -> formal epoch
    37: 13,   # checkpoint suspend
    75: 14,   # never stabilize 1.0
    78: 13,   # checkpoint threats
    101: 13,  # Verus N+2
    110: 14,  # new cap kind post-150
    113: 13,  # checkpoint owner
    132: 13,  # fuzz->Verus
    138: 13,  # Tier D
    139: 13,  # Verus intractable
    147: 14,  # health JSON stabilize
    166: 13,  # Verus runway
    182: 14,  # semver vs never-stabilize
    194: 13,  # toolchain risk
    235: 12,  # service restart identity
    239: 10,  # userspace allocator
    252: 12,  # network isolation
    253: 11,  # storage access
    256: 8,   # Kani cache
    267: 12,  # R-cascade SMP
    277: 14,  # multitenancy
    278: 14,  # deterministic replay
    280: 12,  # external interop
    305: 10,  # boot attestation
    324: 12,  # distributed revocation
    330: 14,  # a11y
    341: 14,  # post-150 inventory
}


def epoch_when(n: int) -> str:
    return f"Epoch {n}"


def main() -> int:
    path = ROOT / "gap_registry.toml"
    gaps = parse_gaps(path.read_text(encoding="utf-8"))
    reopened = 0
    for g in gaps:
        if g.get("status") != "wontfix":
            continue
        gid = g.get("id")
        epoch = EPOCH_MAP.get(gid)
        if epoch is None:
            # Default: assign by keywords in when/fix
            blob = f"{g.get('when', '')} {g.get('fix', '')}".lower()
            if "epoch 10" in blob or "hardware" in blob or "security.md" in blob:
                epoch = 10
            elif "epoch 11" in blob or "driver" in blob:
                epoch = 11
            elif "federation" in blob or "distributed" in blob:
                epoch = 12
            elif "formal" in blob or "verus" in blob or "tier d" in blob or "checkpoint" in blob:
                epoch = 13
            elif "1.0" in blob or "milestone" in blob or "release" in blob:
                epoch = 14
            else:
                epoch = 7
        g["status"] = "open"
        g["when"] = epoch_when(epoch)
        g["implementing_doc"] = f"epoch-{epoch}-target"
        reopened += 1
    # Close gap 350 (lifecycle script delivered)
    for g in gaps:
        if g.get("id") == 350 and g.get("status") == "open":
            g["status"] = "addressed"
            g["implementing_doc"] = "scripts/reopen_post150_gaps.py"
    path.write_text(emit_gaps(gaps), encoding="utf-8")
    open_count = sum(1 for g in gaps if g.get("status") == "open")
    print(f"reopen_post150_gaps: reopened {reopened} wontfix -> open; {open_count} open total")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
