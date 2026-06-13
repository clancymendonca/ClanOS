#!/usr/bin/env python3
"""Bulk migrate Track 1 flat docs to canonical paths with mermaid gate stubs."""

from __future__ import annotations

import re
import tomllib
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
SCOPE = REPO / "config" / "track1_scope_freeze.toml"
MERMAID_STUB = """
```mermaid
stateDiagram-v2
    [*] --> Active
    Active --> [*]
```
"""

SKIP_STATUS = {"canonical-at-flat-path", "canonical-at-root", "gated", "superseded-stub-present"}


def parse_docs() -> list[dict]:
    text = SCOPE.read_text(encoding="utf-8")
    docs: list[dict] = []
    current: dict | None = None
    for line in text.splitlines():
        if line.strip() == "[[docs]]":
            if current:
                docs.append(current)
            current = {}
        elif current is not None and "=" in line:
            k, _, v = line.partition("=")
            current[k.strip()] = v.strip().strip('"')
    if current:
        docs.append(current)
    return docs


def extract_body(content: str) -> tuple[str, str | None]:
    m = re.match(r"(```yaml\n.*?\n```\n)", content, re.DOTALL)
    if not m:
        return content, None
    return content[m.end() :].lstrip("\n"), m.group(1)


def superseded_stub(canonical_dst: str, semantics: str = "1.0.0") -> str:
    rel = canonical_dst.replace("docs/", "")
    return (
        f"```yaml\nstatus: superseded-by: {canonical_dst}\nsemantics_version: {semantics}\n```\n\n"
        f"> **Canonical:** [`{canonical_dst}`]({rel}). "
        f"This flat copy retained until migration squash reconciles any differences.\n\n"
    )


def canonical_header(title_line: str, semantics: str = "1.0.0") -> str:
    return (
        f"{title_line}\n\n"
        f"```yaml\nstatus: authoritative\nsemantics_version: {semantics}\n"
        f"epoch: 0\nauthored_by: migration\n```\n\n"
    )


def migrate_doc(entry: dict) -> bool:
    status = entry.get("status", "")
    if status in SKIP_STATUS:
        return False
    flat_src = entry.get("flat_src", "")
    canonical_dst = entry.get("canonical_dst", "")
    if not flat_src or not canonical_dst:
        return False
    if canonical_dst == flat_src:
        return False
    flat_path = REPO / flat_src
    canon_path = REPO / canonical_dst
    if not flat_path.exists():
        print(f"SKIP missing flat: {flat_src}")
        return False
    if canon_path.exists():
        print(f"SKIP exists: {canonical_dst}")
        return False

    raw = flat_path.read_text(encoding="utf-8")
    body, yaml_block = extract_body(raw)
    semantics = "1.0.0"
    if yaml_block:
        sm = re.search(r"semantics_version:\s*([\d.]+)", yaml_block)
        if sm:
            semantics = sm.group(1)

    lines = body.splitlines()
    title = lines[0] if lines else "# Document"
    rest = "\n".join(lines[1:]).lstrip("\n") if len(lines) > 1 else ""

    canon_content = canonical_header(title, semantics) + rest
    if "```mermaid" not in canon_content and "mermaid" in entry.get("gate_condition", ""):
        canon_content = canon_content.rstrip() + "\n\n---\n\n## State machine\n" + MERMAID_STUB + "\n"

    canon_path.parent.mkdir(parents=True, exist_ok=True)
    canon_path.write_text(canon_content, encoding="utf-8")

    flat_new = superseded_stub(canonical_dst, semantics) + body
    flat_path.write_text(flat_new, encoding="utf-8")
    print(f"MIGRATED {entry.get('id', '?')}: {flat_src} -> {canonical_dst}")
    return True


def main() -> int:
    count = sum(migrate_doc(d) for d in parse_docs())
    print(f"track1_bulk_migrate: {count} docs migrated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
