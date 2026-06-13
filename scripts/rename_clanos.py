#!/usr/bin/env python3
"""One-shot Clan OS rebrand: replace legacy Ares identifiers in text files."""

from __future__ import annotations

import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]

SKIP_DIRS = {
    ".git",
    "target",
    "target-host",
    "proof-rights",
    "__pycache__",
    ".cursor/projects",
}

SKIP_FILES = {
    "scripts/rename_clanos.py",
    "Cargo.lock",
}

# Order matters: longest / most specific first.
REPLACEMENTS: list[tuple[str, str]] = [
    ("ABI_ARES_RT", "ABI_CLAN_RT"),
    ("AREOS DESKTOP", "CLANOS DESKTOP"),
    ("AREOS SHELL", "CLANOS SHELL"),
    ("AREOS", "CLANOS"),
    ("ares-copyin-ok", "clan-copyin-ok"),
    ("ares-info", "clan-info"),
    ("ares-exec-v1", "clan-exec-v1"),
    ("ARESFS1", "CLANFS1"),
    ("ares-rt-001", "clan-rt-001"),
    ("ares-rt-0.1.0", "clan-rt-0.1.0"),
    ("scripts/gate/ares_rt.py", "scripts/gate/clan_rt.py"),
    ("gate/ares_rt.py", "gate/clan_rt.py"),
    ("from gate.ares_rt import", "from gate.clan_rt import"),
    ("ares-native-v1", "clan-native-v1"),
    ("ares-semantics-v1", "clan-semantics-v1"),
    ("ares-semantics-v*", "clan-semantics-v*"),
    ("ares-abi-v1", "clan-abi-v1"),
    ("ares-abi-v*", "clan-abi-v*"),
    ("ARE_SEMANTICS_V1", "CLAN_SEMANTICS_V1"),
    ("ARE_ABI_V1", "CLAN_ABI_V1"),
    ("AresFS", "ClanFS"),
    ("AresError", "ClanError"),
    ("AresRtError", "ClanRtError"),
    ("aresos_full_os_build_b960e4a0", "clanos_full_os_build_b960e4a0"),
    ("aresos_build_151_350", "clanos_build_151_350"),
    ("aresos_full_os_build", "clanos_full_os_build"),
    ("ares-rt", "clan-rt"),
    ("ares_rt", "clan_rt"),
    ("aresos", "clanos"),
    ("AresOS", "ClanOS"),
    ("&image_bytes[120..124] == b\"ARES\" || &image_bytes[120..124] == b\"CLAN\"", "&image_bytes[120..124] == b\"CLAN\""),
    ("b\"ARES\"", "b\"CLAN\""),
]

TEXT_SUFFIXES = {
    ".rs",
    ".toml",
    ".md",
    ".mdc",
    ".py",
    ".json",
    ".txt",
    ".yml",
    ".yaml",
    ".ps1",
    ".sh",
    ".html",
    ".css",
    ".js",
    ".ts",
    ".tsx",
    ".jsx",
    ".lock",
}


def should_skip(path: Path) -> bool:
    rel = path.relative_to(REPO).as_posix()
    if rel in SKIP_FILES:
        return True
    for part in path.parts:
        if part in SKIP_DIRS:
            return True
    return path.suffix not in TEXT_SUFFIXES and path.name not in ("LICENSE", "CHARTER", "SECURITY")


def main() -> int:
    changed = 0
    for path in sorted(REPO.rglob("*")):
        if not path.is_file() or should_skip(path):
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        original = text
        for old, new in REPLACEMENTS:
            text = text.replace(old, new)
        if text != original:
            path.write_text(text, encoding="utf-8", newline="\n")
            changed += 1
            print(f"updated: {path.relative_to(REPO)}")
    print(f"rename_clanos: {changed} files updated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
