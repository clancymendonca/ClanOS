#!/usr/bin/env python3
"""CAP_REGISTRY.toml object_kind values must match kernel ObjectKind enum."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def parse_registry_kinds(text: str) -> set[str]:
    return set(re.findall(r'object_kind = "([^"]+)"', text))


def parse_kernel_object_kinds(text: str) -> set[str]:
    block = re.search(r"pub enum ObjectKind \{([^}]+)\}", text, re.S)
    if not block:
        return kinds
    return set(re.findall(r"(\w+)", block.group(1)))


def main() -> int:
    reg = (ROOT / "docs" / "CAP_REGISTRY.toml").read_text(encoding="utf-8")
    kernel = (ROOT / "kernel" / "src" / "kernel_object.rs").read_text(encoding="utf-8")
    reg_kinds = parse_registry_kinds(reg)
    kernel_kinds = parse_kernel_object_kinds(kernel)
    missing_in_reg = kernel_kinds - reg_kinds
    extra_in_reg = reg_kinds - kernel_kinds
    if missing_in_reg or extra_in_reg:
        if missing_in_reg:
            print(f"cap_registry_sync: kernel kinds missing from registry: {sorted(missing_in_reg)}", file=sys.stderr)
        if extra_in_reg:
            print(f"cap_registry_sync: registry kinds missing from kernel: {sorted(extra_in_reg)}", file=sys.stderr)
        return 1
    print(f"cap_registry_sync: OK ({len(reg_kinds)} kinds synced)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
