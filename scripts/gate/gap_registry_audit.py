#!/usr/bin/env python3
"""Gap registry honesty audit — minimum evidence for status=addressed rows."""

from __future__ import annotations

import argparse
import re
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_REGISTRY = ROOT / "gap_registry.toml"
sys.path.insert(0, str(ROOT / "scripts"))
from mark_epoch0_addressed import doc_paths_from_fix, parse_gaps  # noqa: E402

# gap_id -> (implementing_doc, code_path) with verified delivery (mark_delivered_gaps.py).
DELIVERED: dict[int, tuple[str, str]] = {
    3: ("docs/BUILD_INTEGRITY.md", "kernel/src/build_integrity.rs"),
    4: ("docs/AUDIT_SUBSYSTEM.md", "kernel/src/audit_wire.rs"),
    6: ("proof-rights/src/lib.rs", "proof-rights/src/lib.rs"),
    7: ("docs/ERROR_TAXONOMY.md", "kernel/src/service_scheduler.rs"),
    18: ("docs/PROOF_COVERAGE.md", "proof-rights/src/lib.rs"),
    40: ("docs/FUZZ_TARGETS.md", "docs/FUZZ_TARGETS.md"),
    57: ("docs/KANI_SCOPE.md", "kani_harness_registry.toml"),
}

# Baseline overclaimed count from first GAP_AUDIT (2026-06-20). CI fails if count rises.
# Lower only after explicit gap_registry.toml review (see docs/GAP_AUDIT.md).
EXPECTED_OVERCLAIMED_BASELINE = 204
# Rows with implementing_doc=milestone-150-stub from close_remaining_plan_gaps.py bulk close.
EXPECTED_MILESTONE_150_STUB_BASELINE = 202
MILESTONE_150_STUB = "milestone-150-stub"

DOC_PREFIXES = ("docs/", "CHARTER.md", "SECURITY.md")
CODE_PREFIXES = ("kernel/src/", "proof-rights/", "scripts/", "userland/")

PATH_IN_FIX = re.compile(
    r"(?:\([\`\"]?)?((?:docs/[^\s\)`\"]+\.md|kernel/src/[^\s\)`\"]+\.rs|"
    r"proof-rights/[^\s\)`\"]+|scripts/[^\s\)`\"]+\.py|userland/[^\s\)`\"]+))"
)


def _normalize_path(raw: str) -> str:
    return raw.strip("`\"' ")


def evidence_paths(gap: dict) -> tuple[list[str], list[str]]:
    """Return (doc_paths, code_paths) resolved relative to repo root."""
    docs: list[str] = []
    code: list[str] = []
    seen: set[str] = set()

    def add(path: str) -> None:
        path = _normalize_path(path)
        if not path or path in seen:
            return
        seen.add(path)
        if path.startswith(DOC_PREFIXES) or (
            path.endswith(".md") and "/" not in path
        ):
            docs.append(path)
        elif any(path.startswith(p) for p in CODE_PREFIXES) or path.endswith(
            (".rs", ".py", ".toml")
        ):
            code.append(path)

    fix = gap.get("fix") or ""
    for p in doc_paths_from_fix(fix):
        add(p)
    for m in PATH_IN_FIX.finditer(fix):
        add(m.group(1))

    impl = gap.get("implementing_doc")
    if impl:
        add(impl)

    return docs, code


def path_exists(rel: str) -> bool:
    return (ROOT / rel).is_file()


def classify_gap(gap: dict) -> str:
    status = gap.get("status")
    if status == "superseded":
        return "Superseded"
    if status == "wontfix":
        return "Wontfix"
    if status != "addressed":
        return "Other"

    gid = gap.get("id")
    docs, code = evidence_paths(gap)
    has_doc = any(path_exists(p) for p in docs)
    has_code = any(path_exists(p) for p in code)

    if gid in DELIVERED:
        doc, impl = DELIVERED[gid]
        if path_exists(doc) and path_exists(impl):
            return "Implemented"

    if not has_doc and not has_code:
        return "Overclaimed"
    if has_code and has_doc:
        return "Implemented"
    if has_code:
        return "Partial"
    if has_doc:
        return "DocOnly"
    return "Overclaimed"


def count_milestone_150_stub(gaps: list[dict]) -> int:
    return sum(
        1
        for g in gaps
        if g.get("status") == "addressed" and g.get("implementing_doc") == MILESTONE_150_STUB
    )


def audit_gaps(gaps: list[dict]) -> tuple[list[dict], Counter[str]]:
    """Return (overclaimed_gap_rows, class_histogram)."""
    counts: Counter[str] = Counter()
    overclaimed: list[dict] = []
    for g in gaps:
        cls = classify_gap(g)
        counts[cls] += 1
        if cls == "Overclaimed":
            overclaimed.append(g)
    return overclaimed, counts


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--file",
        type=Path,
        default=DEFAULT_REGISTRY,
        help="gap_registry.toml path (default: repo root)",
    )
    ap.add_argument(
        "--summary",
        action="store_true",
        help="Print class histogram only (exit 0)",
    )
    ap.add_argument(
        "--strict",
        action="store_true",
        help="Fail on any overclaimed row (not default baseline mode)",
    )
    args = ap.parse_args()
    gaps = parse_gaps(args.file.read_text(encoding="utf-8"))
    overclaimed, counts = audit_gaps(gaps)

    if args.summary:
        for cls, n in sorted(counts.items()):
            print(f"gap_registry_audit: {cls}: {n}")
        print(f"gap_registry_audit: total: {sum(counts.values())}")
        return 0

    n_over = len(overclaimed)
    n_stub = count_milestone_150_stub(gaps)
    if n_stub > EXPECTED_MILESTONE_150_STUB_BASELINE:
        print(
            f"gap_registry_audit: FAIL milestone-150-stub rows {n_stub} > baseline "
            f"{EXPECTED_MILESTONE_150_STUB_BASELINE}; see close_remaining_plan_gaps.py",
            file=sys.stderr,
        )
        return 1

    if args.strict and n_over:
        for g in overclaimed[:20]:
            print(
                f"gap_registry_audit: overclaimed gap {g.get('id')}: "
                f"{g.get('summary', '')[:72]!r}",
                file=sys.stderr,
            )
        if n_over > 20:
            print(f"gap_registry_audit: ... and {n_over - 20} more", file=sys.stderr)
        print(f"gap_registry_audit: FAIL {n_over} overclaimed gap(s)", file=sys.stderr)
        return 1

    if n_over > EXPECTED_OVERCLAIMED_BASELINE:
        print(
            f"gap_registry_audit: FAIL overclaimed {n_over} > baseline "
            f"{EXPECTED_OVERCLAIMED_BASELINE}; update gap_registry or baseline after review",
            file=sys.stderr,
        )
        return 1

    print(
        f"gap_registry_audit: OK (histogram: {dict(counts)}; "
        f"overclaimed={n_over}, baseline={EXPECTED_OVERCLAIMED_BASELINE}; "
        f"milestone-150-stub={n_stub}, stub_baseline={EXPECTED_MILESTONE_150_STUB_BASELINE})"
    )
    print(
        "gap_registry_audit: note: OK means debt did not exceed baseline — "
        f"{n_over} addressed rows still lack minimum evidence. "
        "Use --summary for class histogram; --strict for zero-debt mode."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
