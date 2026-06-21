#!/usr/bin/env python3
"""ADR-0003 Q3 sunset enforcement — hard fail on stale digest-only allowlist."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
POLICY = ROOT / "config" / "loader_signing_policy.toml"
ALLOWLIST = ROOT / "config" / "loader_digest_only_allowlist.toml"
ARCH = ROOT / "architecture_state.toml"
SEED_MIGRATION_TOTAL = 16


def _toml_int(text: str, key: str) -> int | None:
    m = re.search(rf"^{re.escape(key)}\s*=\s*(\d+)", text, re.M)
    return int(m.group(1)) if m else None


def _toml_bool(text: str, key: str) -> bool | None:
    m = re.search(rf"^{re.escape(key)}\s*=\s*(true|false)", text, re.M)
    if not m:
        return None
    return m.group(1) == "true"


def load_allowlist_names(path: Path) -> list[str]:
    text = path.read_text(encoding="utf-8")
    return re.findall(r'name\s*=\s*"([^"]+)"', text)


def main() -> int:
    errors: list[str] = []

    if not POLICY.is_file():
        errors.append(f"missing {POLICY}")
    if not ALLOWLIST.is_file():
        errors.append(f"missing {ALLOWLIST}")
    if not ARCH.is_file():
        errors.append(f"missing {ARCH}")

    if errors:
        for e in errors:
            print(f"loader_signing_sunset_check: {e}", file=sys.stderr)
        return 1

    policy = POLICY.read_text(encoding="utf-8")
    arch = ARCH.read_text(encoding="utf-8")
    sunset_scope = _toml_int(policy, "sunset_scope")
    current_scope = _toml_int(policy, "current_scope")
    grace = _toml_bool(arch, "loader_digest_only_grace")
    names = load_allowlist_names(ALLOWLIST)

    if sunset_scope is None or current_scope is None:
        errors.append("loader_signing_policy.toml missing sunset_scope or current_scope")
    if grace is None:
        errors.append("architecture_state.toml missing loader_digest_only_grace flag")

    if errors:
        for e in errors:
            print(f"loader_signing_sunset_check: {e}", file=sys.stderr)
        return 1

    # Locked ADR-0003 Q3 triggers (numeric, gate-checkable).
    if current_scope >= sunset_scope and names:
        errors.append(
            f"current_scope={current_scope} >= sunset_scope={sunset_scope} "
            f"but digest-only allowlist has {len(names)} entries"
        )
    if grace is False and names:
        errors.append(
            f"loader_digest_only_grace=false but allowlist has {len(names)} entries"
        )

    if errors:
        for e in errors:
            print(f"loader_signing_sunset_check: {e}", file=sys.stderr)
        return 1

    digest_only_remaining = len(names)
    seeds_signed = SEED_MIGRATION_TOTAL - digest_only_remaining

    print(
        f"loader_signing_sunset_check: OK "
        f"(current_scope={current_scope}, sunset_scope={sunset_scope}, "
        f"digest_only_remaining={digest_only_remaining}, "
        f"seeds_signed={seeds_signed} of {SEED_MIGRATION_TOTAL}, grace={grace})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
