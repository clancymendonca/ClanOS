#!/usr/bin/env python3
"""Mark gap_registry gaps addressed when epoch deliverables + docs exist."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
from mark_epoch0_addressed import doc_paths_from_fix, emit_gaps, parse_gaps  # noqa: E402

# gap_id -> implementing artifact (beyond doc-only)
IMPL: dict[int, str] = {
    3: "kernel/src/build_integrity.rs",
    4: "kernel/src/audit_wire.rs",
    5: "scripts/validation_matrix.py",
    7: "kernel/src/service_loader.rs",
    8: "compat_test_corpus.toml",
    9: "kernel/src/oom_policy.rs",
    11: "docs/ABI_NATIVE_SYSCALL.md",
    12: "userland/src/lib.rs",
    13: "kernel/src/compositor.rs",
    14: "kernel/src/virtio_blk.rs",
    15: "kernel/src/ipc_interim_bridge.rs",
    16: "kernel/src/virtio_blk.rs",
    17: "loom_harness_registry.toml",
    21: "kernel/src/audit_wire.rs",
    23: "kernel/src/ipc_endpoints.rs",
    24: "loom_harness_registry.toml",
    27: "benchmarks/phase-120-baseline.json",
    29: "docs/ARCHITECTURE_TARGETS.md",
    36: "docs/WIRE_SCHEMA_REGISTRY.md",
    38: "docs/COMPAT_ISOLATION.md",
    39: "docs/IPC_VERSION_NEGOTIATION.md",
    41: "scripts/project_health.py",
    45: "docs/EPOCH_FAILURE_PROCEDURE.md",
    48: "proof-rights/src/lib.rs",
    50: "kernel/src/ipc_endpoints.rs",
    51: "kernel/src/audit_wire.rs",
    52: "compat_test_corpus.toml",
    53: "kernel/src/build_integrity.rs",
    56: "kernel/src/service_loader.rs",
    58: "kernel/tests/stack_overflow.rs",
    60: "scripts/doc_link_check.py",
    71: "docs/CAP_REGISTRY.toml",
    72: "docs/THREAT_NODES.toml",
    73: "scripts/rights_algebra_check.py",
    74: "health_timeseries.json",
    76: "kernel/src/service_loader.rs",
    109: "scripts/covenant_ci.py",
    183: "architecture_state.toml",
    344: "CONTRIBUTING.md",
}

EPOCH_SMOKE: dict[int, str] = {
    121: "scripts/phase121_service_loader_check.py",
    130: "scripts/phase130_platform_check.py",
    134: "scripts/phase134_endpoint_check.py",
    140: "scripts/phase134_endpoint_check.py",
    201: "scripts/phase201_virtio_blk_check.py",
    404: "scripts/phase404_network_check.py",
    149: "scripts/phase149_epoch5_check.py",
    150: "scripts/phase150_milestone_check.py",
}


def main() -> int:
    path = ROOT / "gap_registry.toml"
    gaps = parse_gaps(path.read_text(encoding="utf-8"))
    marked = 0
    for g in gaps:
        if g.get("status") != "open":
            continue
        gid = g.get("id")
        docs = doc_paths_from_fix(g.get("fix", ""))
        impl = IMPL.get(gid)
        if impl and (ROOT / impl).exists():
            g["status"] = "addressed"
            g["implementing_doc"] = impl
            marked += 1
            continue
        if docs and all((ROOT / d).exists() for d in docs):
            g["status"] = "addressed"
            g["implementing_doc"] = docs[0]
            marked += 1
    path.write_text(emit_gaps(gaps), encoding="utf-8")
    open_count = sum(1 for g in gaps if g.get("status") == "open")
    print(f"mark_plan_complete_gaps: marked {marked}; {open_count} still open")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
