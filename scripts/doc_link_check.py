#!/usr/bin/env python3
"""Link stub + authoritative doc status header checker (epoch 0 CI)."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"
STUB = re.compile(r"\[CROSS-REF:.*?— TBD\]", re.I)
STATUS_HEADER = re.compile(r"^status:\s*\S+", re.M)
MIGRATING_STATUS = re.compile(r"^status:\s*migrating\b", re.M)
SUPERSEDED_BY = re.compile(r"status:\s*superseded-by:\s*(\S+)", re.I)

# Epoch-0 authoritative corpus — must carry `status:` in yaml frontmatter or header block.
AUTHORITATIVE_DOCS = [
    "CHARTER.md",
    "SECURITY.md",
    "docs/TEMPORAL_SEMANTICS.md",
    "docs/KERNEL_OBJECT_MODEL.md",
    "docs/FAULT_ESCALATION.md",
    "docs/RIGHTS_ALGEBRA.md",
    "docs/SCHEDULER_MODEL.md",
    "docs/THREAT_MODEL.md",
    "docs/PROOF_COVERAGE.md",
    "docs/KANI_SCOPE.md",
    "docs/UNSAFE_AUDIT.md",
    "docs/DESIGN_NORTH_STAR.md",
    "docs/ERROR_TAXONOMY.md",
    "docs/EPOCH_FAILURE_PROCEDURE.md",
    "docs/SUPPLY_CHAIN_POLICY.md",
    "docs/GENERATION_COUNTER.md",
    "docs/DEPENDENCY_POLICY.md",
    "docs/CAP_TRANSFER_PROTOCOL.md",
    "docs/MEMORY_SAFETY_BOUNDARY.md",
    "docs/LIVENESS_PROPERTIES.md",
    "docs/FUZZ_TARGETS.md",
    "docs/BUILD_INTEGRITY.md",
    "docs/AUDIT_SUBSYSTEM.md",
    "docs/WIRE_SCHEMA_REGISTRY.md",
    "docs/COMPAT_ISOLATION.md",
    "docs/COMPAT_SUNSET.md",
    "docs/IPC_VERSION_NEGOTIATION.md",
    "docs/ABI_NATIVE_SYSCALL.md",
    "docs/ABI_CLAN_RT.md",
    "docs/VIRTIO_SAFETY.md",
    "docs/ABI_COMPOSITOR_IPC.md",
    "docs/ARCHITECTURE_TARGETS.md",
    "docs/DECISION_LOG.md",
    "docs/PROTOCOL_CHANGELOG.md",
    "docs/PLAN_SUPERSESSION.md",
]


def check_superseded_pointers() -> list[str]:
    errors: list[str] = []
    for path in DOCS.glob("**/*.md"):
        text = path.read_text(encoding="utf-8")
        for target in SUPERSEDED_BY.findall(text):
            canon = ROOT / target
            if not canon.exists():
                errors.append(
                    f"{path.relative_to(ROOT)}: superseded-by target missing: {target}"
                )
    return errors


def check_status_headers() -> list[str]:
    errors: list[str] = []
    for rel in AUTHORITATIVE_DOCS:
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: authoritative doc missing")
            continue
        text = path.read_text(encoding="utf-8")
        head = text[:600]
        if not STATUS_HEADER.search(head):
            errors.append(f"{rel}: missing status: header in first 600 chars")
    return errors


def main() -> int:
    errors: list[str] = []
    for path in list(DOCS.glob("**/*.md")) + [ROOT / "CHARTER.md", ROOT / "SECURITY.md"]:
        if not path.exists():
            continue
        text = path.read_text(encoding="utf-8")
        head = text[:600]
        if MIGRATING_STATUS.search(head):
            continue
        if STUB.search(text):
            errors.append(f"{path.relative_to(ROOT)}: unresolved CROSS-REF TBD stub")
    errors.extend(check_superseded_pointers())
    errors.extend(check_status_headers())
    if errors:
        for e in errors:
            print(f"error: {e}", file=sys.stderr)
        return 1
    print("doc_link_check: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
