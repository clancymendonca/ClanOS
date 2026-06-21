#!/usr/bin/env python3
"""Mechanical ADR-0003 Q4 execution-path verify enumeration check.

The table in docs/architecture/ADR/ADR-0003-loader-signed-exec-manifests.md § Q4
must stay in sync with REQUIRED_CALLS below. Any new program execution entry point
requires a row in both places.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
KERNEL = ROOT / "kernel" / "src"
ADR = ROOT / "docs" / "architecture" / "ADR" / "ADR-0003-loader-signed-exec-manifests.md"

# (source file under kernel/src/, function name, required verify helper substring)
REQUIRED_CALLS: tuple[tuple[str, str, str], ...] = (
    ("task/program_loader.rs", "resolve_program_for", "verify_system_signed_program"),
    ("task/program_loader.rs", "execute_trusted_manifest_elf", "verify_system_signed_program"),
    ("task/program_loader.rs", "execute_manifest_elf_gated", "verify_system_signed_program"),
    ("task/program_loader.rs", "execute_allowlisted_user_elf", "verify_system_signed_program"),
    ("task/program_loader.rs", "prepare_program_image", "verify_system_signed_program"),
    ("task/program_loader.rs", "validate_program_image", "verify_system_signed_program"),
    ("corpus_runner.rs", "execute_corpus_bytes", "verify_system_signed_elf_payload"),
)

# Documented exemptions — revisit when `hello` is signed.
EXEMPT: tuple[tuple[str, str, str], ...] = (
    (
        "task/program_loader.rs",
        "execute_minimal_user_elf_descriptor",
        "N/A until hello signed",
    ),
)

# Indirect coverage (must appear in ADR table; not grep-checked for verify call).
ADR_INDIRECT_ROWS: tuple[str, ...] = (
    "execute_corpus_elf",
    "userspace::run_program",
    "build_hw_page_table_program",
)


def fn_body(source: str, fn_name: str) -> str | None:
    pattern = rf"(?:pub\s+)?fn\s+{re.escape(fn_name)}\s*\("
    match = re.search(pattern, source)
    if not match:
        return None
    start = match.start()
    depth = 0
    in_fn = False
    for idx in range(match.end(), len(source)):
        ch = source[idx]
        if ch == "{":
            depth += 1
            in_fn = True
        elif ch == "}":
            depth -= 1
            if in_fn and depth == 0:
                return source[start : idx + 1]
    return None


def main() -> int:
    failures = 0

    for rel_path, fn_name, verify_call in REQUIRED_CALLS:
        path = KERNEL / rel_path
        if not path.is_file():
            print(f"test_loader_signed_exec_path_audit: FAIL missing {path}", file=sys.stderr)
            failures += 1
            continue
        source = path.read_text(encoding="utf-8")
        body = fn_body(source, fn_name)
        if body is None:
            print(
                f"test_loader_signed_exec_path_audit: FAIL fn {fn_name} not in {rel_path}",
                file=sys.stderr,
            )
            failures += 1
            continue
        if verify_call not in body:
            print(
                f"test_loader_signed_exec_path_audit: FAIL {rel_path}::{fn_name} "
                f"missing {verify_call}",
                file=sys.stderr,
            )
            failures += 1
        else:
            print(f"test_loader_signed_exec_path_audit: OK {rel_path}::{fn_name}")

    for rel_path, fn_name, reason in EXEMPT:
        path = KERNEL / rel_path
        if path.is_file() and fn_body(path.read_text(encoding="utf-8"), fn_name):
            print(f"test_loader_signed_exec_path_audit: note exempt {fn_name} ({reason})")

    if ADR.is_file():
        adr_text = ADR.read_text(encoding="utf-8")
        for _, fn_name, _ in REQUIRED_CALLS:
            if fn_name not in adr_text:
                print(
                    f"test_loader_signed_exec_path_audit: FAIL ADR table missing {fn_name}",
                    file=sys.stderr,
                )
                failures += 1
        for row in ADR_INDIRECT_ROWS:
            if row not in adr_text:
                print(
                    f"test_loader_signed_exec_path_audit: FAIL ADR table missing {row}",
                    file=sys.stderr,
                )
                failures += 1
        for _, fn_name, _ in EXEMPT:
            if fn_name not in adr_text:
                print(
                    f"test_loader_signed_exec_path_audit: FAIL ADR table missing exempt {fn_name}",
                    file=sys.stderr,
                )
                failures += 1
    else:
        print(f"test_loader_signed_exec_path_audit: FAIL missing {ADR}", file=sys.stderr)
        failures += 1

    # No direct SystemSigned handling outside program_loader / corpus_runner verify helpers.
    for path in KERNEL.rglob("*.rs"):
        rel = path.relative_to(KERNEL).as_posix()
        if rel in ("loader_signed_exec.rs", "signed_elf.rs"):
            continue
        text = path.read_text(encoding="utf-8")
        if "ProgramTrust::SystemSigned" not in text:
            continue
        if rel in ("task/program_loader.rs", "corpus_runner.rs"):
            continue
        print(
            "test_loader_signed_exec_path_audit: FAIL unexpected SystemSigned reference "
            f"in {rel} — add to ADR Q4 table or remove",
            file=sys.stderr,
        )
        failures += 1

    if failures:
        print(f"test_loader_signed_exec_path_audit: {failures} failure(s)", file=sys.stderr)
        return 1
    print("test_loader_signed_exec_path_audit: OK (enumeration mechanically checked)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
