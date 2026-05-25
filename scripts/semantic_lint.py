#!/usr/bin/env python3
"""Architecture-preservation lint for post-100 constitutional docs (phases 101-110)."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REQUIRED_DOCS = [
    "AXIOMS.md",
    "NATIVE_MODEL.md",
    "KERNEL_OBJECT_MODEL.md",
    "RIGHTS_ALGEBRA.md",
    "TEMPORAL_SEMANTICS.md",
    "SEMANTIC_SPECS.md",
    "SEMANTIC_JURISDICTION.md",
    "SEMANTIC_LINT.md",
    "SEMANTIC_OBSERVABILITY.md",
    "ABI_SYSCALL.md",
    "ABI_MEMORY.md",
    "ABI_IPC.md",
    "ABI_ASYNC.md",
    "ABI_RUNTIME.md",
    "ABI_DRIVER.md",
    "ABI_SECURITY.md",
    "ABI_STABILITY.md",
    "NATIVE_DEVELOPER_EXPERIENCE.md",
    "ROADMAP_POST100.md",
]

GATE_DOCS = {
    "G1": "KERNEL_OBJECT_MODEL.md",
    "G2": "RIGHTS_ALGEBRA.md",
    "G3": "ABI_IPC.md",
    "G4": "NATIVE_MODEL.md",
    "G5": ("SEMANTIC_SPECS.md", "TEMPORAL_SEMANTICS.md"),
}

SPEC_ID_RE = re.compile(r"\b[RETM]-\d{2}\b")

ABI_TABLE_ID_RE = re.compile(r"^\|\s*(\d+)\s*\|", re.MULTILINE)

ALLOWED_BLOCK_RE = re.compile(
    r"pub const ALLOWED_HW_SYSCALLS.*?&\[(.*?)\];",
    re.DOTALL,
)

SYSCALL_ENUM_RE = re.compile(r"(\w+)\s*=\s*(\d+)", re.MULTILINE)


def parse_syscall_enum(syscall_rs: Path) -> dict[str, int]:
    text = syscall_rs.read_text(encoding="utf-8")
    names: dict[str, int] = {}
    in_enum = False
    for line in text.splitlines():
        if "pub enum SyscallId" in line:
            in_enum = True
            continue
        if in_enum and line.strip().startswith("}"):
            break
        m = SYSCALL_ENUM_RE.search(line)
        if in_enum and m:
            names[m.group(1)] = int(m.group(2))
    return names


def parse_allowed_hw_ids(user_syscall_hw: Path, syscall_rs: Path) -> set[int]:
    text = user_syscall_hw.read_text(encoding="utf-8")
    m = ALLOWED_BLOCK_RE.search(text)
    if not m:
        raise RuntimeError("ALLOWED_HW_SYSCALLS block not found")
    enum_map = parse_syscall_enum(syscall_rs)
    ids: set[int] = set()
    for name in re.findall(r"SyscallId::(\w+)", m.group(1)):
        if name not in enum_map:
            raise RuntimeError(f"unknown SyscallId::{name}")
        ids.add(enum_map[name])
    return ids


def parse_abi_doc_ids(abi_syscall_md: Path) -> set[int]:
    text = abi_syscall_md.read_text(encoding="utf-8")
    start = text.find("## Allowlisted hardware syscalls")
    if start < 0:
        return set()
    chunk = text[start:]
    end = chunk.find("\n---\n", 10)
    if end > 0:
        chunk = chunk[:end]
    ids: set[int] = set()
    in_table = False
    for line in chunk.splitlines():
        if "| ID |" in line and "Name" in line:
            in_table = True
            continue
        if not in_table:
            continue
        if line.strip().startswith("|---"):
            continue
        m = re.match(r"\|\s*(\d+)\s*\|", line.strip())
        if m:
            ids.add(int(m.group(1)))
        elif line.strip() and not line.strip().startswith("|"):
            break
    return ids


def check_required_files(docs: Path, errors: list[str]) -> None:
    for name in REQUIRED_DOCS:
        if not (docs / name).is_file():
            errors.append(f"missing required doc: docs/{name}")


def check_axioms_referenced(docs: Path, errors: list[str]) -> None:
    skip = {"AXIOMS.md", "SEMANTIC_LINT.md"}
    for name in REQUIRED_DOCS:
        if name in skip:
            continue
        path = docs / name
        if "AXIOMS.md" not in path.read_text(encoding="utf-8"):
            errors.append(f"{name}: does not reference AXIOMS.md")


def check_spec_cases(docs: Path, errors: list[str]) -> None:
    specs = docs / "SEMANTIC_SPECS.md"
    text = specs.read_text(encoding="utf-8")
    ids = set(SPEC_ID_RE.findall(text))
    for prefix in ("R", "E", "T"):
        if not any(x.startswith(f"{prefix}-") for x in ids):
            errors.append(f"SEMANTIC_SPECS.md: missing {prefix}-* cases")
    if "linkage matrix" not in text.lower():
        errors.append("SEMANTIC_SPECS.md: missing law ↔ case linkage matrix")


def check_gate_docs(docs: Path, errors: list[str]) -> None:
    for gate, target in GATE_DOCS.items():
        if isinstance(target, tuple):
            for t in target:
                if not (docs / t).is_file():
                    errors.append(f"{gate}: missing docs/{t}")
        elif not (docs / target).is_file():
            errors.append(f"{gate}: missing docs/{target}")


def check_abi_allowlist(repo: Path, errors: list[str]) -> None:
    doc_ids = parse_abi_doc_ids(repo / "docs" / "ABI_SYSCALL.md")
    kernel_ids = parse_allowed_hw_ids(
        repo / "kernel" / "src" / "user_syscall_hw.rs",
        repo / "kernel" / "src" / "syscall.rs",
    )
    if doc_ids != kernel_ids:
        missing_doc = kernel_ids - doc_ids
        missing_kernel = doc_ids - kernel_ids
        if missing_doc:
            errors.append(f"ABI_SYSCALL.md missing IDs in kernel allowlist: {sorted(missing_doc)}")
        if missing_kernel:
            errors.append(f"kernel allowlist missing IDs in ABI_SYSCALL.md: {sorted(missing_kernel)}")


def print_minimization_audit() -> None:
    rows = [
        ("Constitutional", "AXIOMS", "10 axioms"),
        ("Ontology", "KERNEL_OBJECT_MODEL", "1 model + 8 kinds"),
        ("Rights", "RIGHTS_ALGEBRA", "6 operations + 5 revocation modes"),
        ("Temporal", "TEMPORAL_SEMANTICS", "6 visibility domains + meta outline"),
        ("IPC", "ABI_IPC", "7 guarantee areas"),
        ("Async", "ABI_ASYNC", "4 primitives"),
    ]
    print("Minimization audit (phase 110):")
    for layer, doc, count in rows:
        print(f"  {layer:14} {doc:22} {count}")


def main() -> int:
    parser = argparse.ArgumentParser(description="Semantic lint for phases 101-110")
    parser.add_argument(
        "--repo-root",
        type=Path,
        default=Path(__file__).resolve().parents[1],
    )
    args = parser.parse_args()
    repo = args.repo_root
    docs = repo / "docs"
    errors: list[str] = []

    check_required_files(docs, errors)
    check_axioms_referenced(docs, errors)
    check_spec_cases(docs, errors)
    check_gate_docs(docs, errors)
    try:
        check_abi_allowlist(repo, errors)
    except RuntimeError as exc:
        errors.append(str(exc))

    print_minimization_audit()

    if errors:
        print("semantic_lint: FAILED", file=sys.stderr)
        for err in errors:
            print(f"  - {err}", file=sys.stderr)
        return 1

    print("semantic_lint: ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
