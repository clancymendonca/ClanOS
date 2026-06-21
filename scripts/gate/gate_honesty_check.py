#!/usr/bin/env python3
"""Gate honesty linter — trivial stub bodies (part A) and smoke_ok shadowing (part B)."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GATE_RS = ROOT / "kernel" / "src" / "validation_gate.rs"

# Serial-emitted smoke_* in validation_gate.rs (compat + late phase).
SERIAL_SMOKE_FNS = (
    "smoke_compat_signal",
    "smoke_posix_compat",
    "smoke_compat_runtime",
    "smoke_compat_fd_vm",
    "smoke_storage_depth",
)

TRIVIAL_BODY = re.compile(
    r"pub fn (?P<name>smoke_\w+)\(\) -> bool \{\s*(?:true|return true;)\s*\}",
    re.MULTILINE,
)

RUN_SMOKE_FN = re.compile(r"fn (run_\w+_smokes?)\(\) -> bool \{")

LET_SMOKE_OK = re.compile(r"let (?P<mut>mut )?smoke_ok =")

# Single-assignment runners (no shadowing rule).
SHADOW_ALLOWLIST = frozenset(
    {
        "run_constitutional_smokes",
        "run_capabilities_smokes",
        "run_virtio_blk_smoke",
        "run_network_compat_smokes",
        "run_scheduler_epoch_smokes",
        "run_boundary_smoke",
        "run_service_loader_smoke",
    }
)


def _extract_fn_body(text: str, fn_name: str) -> str | None:
    pat = re.compile(rf"fn {re.escape(fn_name)}\(\) -> bool \{{", re.MULTILINE)
    m = pat.search(text)
    if not m:
        return None
    start = m.end()
    depth = 1
    i = start
    while i < len(text) and depth > 0:
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
        i += 1
    return text[start : i - 1]


def check_trivial_stubs(text: str) -> list[str]:
    errors: list[str] = []
    for m in TRIVIAL_BODY.finditer(text):
        name = m.group("name")
        if name in SERIAL_SMOKE_FNS or name.startswith("smoke_compat_") or name.startswith("smoke_posix"):
            errors.append(f"trivial stub body: {name} at offset {m.start()}")
    return errors


def check_smoke_ok_shadowing(text: str) -> list[str]:
    errors: list[str] = []
    for m in RUN_SMOKE_FN.finditer(text):
        fn_name = m.group(1)
        if fn_name in SHADOW_ALLOWLIST:
            continue
        body = _extract_fn_body(text, fn_name)
        if body is None:
            continue
        assigns = list(LET_SMOKE_OK.finditer(body))
        if not assigns:
            continue
        bare = [a for a in assigns if not a.group("mut")]
        mut_init = any(a.group("mut") for a in assigns)
        if len(bare) > 1:
            errors.append(
                f"smoke_ok shadowing: {fn_name} has {len(bare)} "
                f"non-mut 'let smoke_ok =' bindings"
            )
            continue
        if mut_init and bare:
            errors.append(
                f"smoke_ok shadowing: {fn_name} mixes 'let mut smoke_ok =' "
                f"with non-mut rebind"
            )
            continue
        if len(assigns) <= 1:
            continue
        accum = body.count("smoke_ok &=") + body.count("smoke_ok = smoke_ok &")
        if accum < len(assigns) - 1:
            errors.append(
                f"smoke_ok shadowing: {fn_name} has {len(assigns)} "
                f"'let smoke_ok =' bindings but only {accum} accumulations"
            )
    return errors


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--file",
        type=Path,
        default=GATE_RS,
        help="Rust source to lint (default: validation_gate.rs)",
    )
    ap.add_argument(
        "--part",
        choices=("a", "b", "all"),
        default="all",
        help="a=trivial stubs, b=smoke_ok shadowing, all=both",
    )
    args = ap.parse_args()
    text = args.file.read_text(encoding="utf-8")
    errors: list[str] = []
    if args.part in ("a", "all"):
        errors.extend(check_trivial_stubs(text))
    if args.part in ("b", "all"):
        errors.extend(check_smoke_ok_shadowing(text))
    if errors:
        for e in errors:
            print(f"gate_honesty_check: {e}", file=sys.stderr)
        return 1
    parts = []
    if args.part in ("a", "all"):
        parts.append("stubs")
    if args.part in ("b", "all"):
        parts.append("shadowing")
    print(f"gate_honesty_check: OK ({', '.join(parts)})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
