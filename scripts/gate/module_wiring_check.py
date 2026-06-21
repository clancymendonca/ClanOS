#!/usr/bin/env python3
"""Host check: every kernel/src/*.rs file is reachable from lib.rs pub mod tree."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
KERNEL_SRC = REPO / "kernel" / "src"
LIB_RS = KERNEL_SRC / "lib.rs"
MAIN_RS = KERNEL_SRC / "main.rs"

MOD_DECL = re.compile(r"^\s*(?:pub\s+)?mod\s+(\w+)\s*;", re.MULTILINE)

# Must match docs/GATE_AUDIT.md "Dead source inventory" table row-for-row.
# Full paths under kernel/src only — never match by bare filename.
EXPECTED_KNOWN_DEAD_COUNT = 3
KNOWN_DEAD_SOURCES = frozenset(
    {
        (KERNEL_SRC / "buddy.rs").resolve(),
        (KERNEL_SRC / "block_cache.rs").resolve(),
        (KERNEL_SRC / "cow_fork.rs").resolve(),
    }
)


def validate_known_dead_inventory() -> list[str]:
    """Fail loudly if allow-list drifts without a docs update."""
    errors: list[str] = []
    if len(KNOWN_DEAD_SOURCES) != EXPECTED_KNOWN_DEAD_COUNT:
        errors.append(
            f"KNOWN_DEAD_SOURCES length {len(KNOWN_DEAD_SOURCES)} != "
            f"EXPECTED_KNOWN_DEAD_COUNT {EXPECTED_KNOWN_DEAD_COUNT}; "
            "update docs/GATE_AUDIT.md dead-source table"
        )
    for path in sorted(KNOWN_DEAD_SOURCES):
        if path.parent.resolve() != KERNEL_SRC.resolve():
            errors.append(f"known-dead path outside kernel/src: {path}")
        if not path.is_file():
            errors.append(f"known-dead file missing on disk: {path}")
    return errors


def _collect_mod_files(mod_file: Path, wired: set[Path]) -> None:
    resolved = mod_file.resolve()
    if resolved in wired:
        return
    wired.add(resolved)
    text = mod_file.read_text(encoding="utf-8")
    parent_dir = mod_file.parent
    for m in MOD_DECL.finditer(text):
        name = m.group(1)
        child_rs = parent_dir / f"{name}.rs"
        child_mod = parent_dir / name / "mod.rs"
        if child_rs.is_file():
            _collect_mod_files(child_rs, wired)
        elif child_mod.is_file():
            _collect_mod_files(child_mod, wired)


def wired_kernel_sources() -> set[Path]:
    wired: set[Path] = set()
    _collect_mod_files(LIB_RS, wired)
    return wired


def all_kernel_sources() -> set[Path]:
    return {p.resolve() for p in KERNEL_SRC.rglob("*.rs")}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--timeout", type=int, default=30)
    _ = ap.parse_args()

    inventory_errors = validate_known_dead_inventory()
    if inventory_errors:
        for err in inventory_errors:
            print(f"module_wiring_check: {err}", file=sys.stderr)
        return 1

    wired = wired_kernel_sources()
    on_disk = all_kernel_sources()
    # Binary crate entry — not part of lib.rs module tree.
    on_disk.discard(MAIN_RS.resolve())

    orphans = sorted(on_disk - wired, key=lambda p: str(p))
    if not orphans:
        print("module_wiring_check: OK (no orphan .rs files)")
        return 0

    unknown = [p for p in orphans if p.resolve() not in KNOWN_DEAD_SOURCES]
    for path in orphans:
        rel = path.relative_to(REPO)
        tag = "known-dead" if path.resolve() in KNOWN_DEAD_SOURCES else "orphan"
        print(f"module_wiring_check: {tag}: {rel}", file=sys.stderr)

    if unknown:
        print(
            f"module_wiring_check: {len(unknown)} unwired .rs file(s) not in known-dead inventory",
            file=sys.stderr,
        )
        return 1

    print(
        f"module_wiring_check: OK ({len(orphans)} known-dead source file(s) documented; "
        "no unexpected orphans)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
