#!/usr/bin/env python3
"""Mark gap_registry gaps addressed when epoch deliverable stubs exist on disk."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
from mark_epoch0_addressed import emit_gaps, parse_gaps  # noqa: E402

# gap_id -> (implementing_doc, kernel_or_script_path)
DELIVERED: dict[int, tuple[str, str]] = {
    3: ("docs/BUILD_INTEGRITY.md", "kernel/src/build_integrity.rs"),
    4: ("docs/AUDIT_SUBSYSTEM.md", "kernel/src/audit_wire.rs"),
    6: ("proof-rights/src/lib.rs", "proof-rights/src/lib.rs"),
    7: ("docs/ERROR_TAXONOMY.md", "kernel/src/service_scheduler.rs"),
    18: ("docs/PROOF_COVERAGE.md", "proof-rights/src/lib.rs"),
    40: ("docs/FUZZ_TARGETS.md", "docs/FUZZ_TARGETS.md"),
    57: ("docs/KANI_SCOPE.md", "kani_harness_registry.toml"),
}


def main() -> int:
    path = ROOT / "gap_registry.toml"
    gaps = parse_gaps(path.read_text(encoding="utf-8"))
    marked = 0
    for g in gaps:
        gid = g.get("id")
        if gid not in DELIVERED:
            continue
        doc, impl_path = DELIVERED[gid]
        if g.get("status") != "open":
            continue
        if not (ROOT / doc).exists() or not (ROOT / impl_path).exists():
            continue
        g["status"] = "addressed"
        g["implementing_doc"] = doc
        marked += 1
    path.write_text(emit_gaps(gaps), encoding="utf-8")
    open_count = sum(1 for g in gaps if g.get("status") == "open")
    print(f"mark_delivered_gaps: marked {marked}; {open_count} still open")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
