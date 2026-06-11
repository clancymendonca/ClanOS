#!/usr/bin/env python3
"""Kani tier-B gate — runs cargo kani when available, else validates harness registry."""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "kani_harness_registry.toml"


def host_target() -> str:
    proc = subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "host_target.py")],
        capture_output=True,
        text=True,
        check=True,
        cwd=ROOT,
    )
    return proc.stdout.strip()


def kani_available() -> bool:
    try:
        subprocess.run(
            ["cargo", "kani", "--version"],
            capture_output=True,
            check=True,
            cwd=ROOT,
        )
        return True
    except (subprocess.CalledProcessError, FileNotFoundError):
        return False


def count_registry_harnesses() -> int:
    text = REGISTRY.read_text(encoding="utf-8") if REGISTRY.exists() else ""
    return len(re.findall(r"^\[\[harnesses\]\]", text, re.M))


def run_kani() -> int:
    target = host_target()
    manifest = ROOT / "proof-rights" / "Cargo.toml"
    proc = subprocess.run(
        ["cargo", "kani", "--manifest-path", str(manifest), "--target", target],
        cwd=ROOT / "proof-rights",
        text=True,
    )
    return proc.returncode


def run_fallback_tests() -> int:
    proc = subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "rights_algebra_check.py")],
        cwd=ROOT,
        text=True,
    )
    return proc.returncode


def main() -> int:
    harnesses = count_registry_harnesses()
    if harnesses == 0:
        print("kani_gate: error — kani_harness_registry.toml has no [[harnesses]] entries")
        return 1

    if kani_available():
        print(f"kani_gate: running cargo kani -p proof-rights ({harnesses} registered harnesses)")
        code = run_kani()
        if code == 0:
            print("kani_gate: OK")
        return code

    print("kani_gate: cargo-kani not installed — running proptest fallback (tier A)")
    print("Install: cargo install cargo-kani")
    code = run_fallback_tests()
    if code == 0:
        print("kani_gate: OK (proptest fallback; install cargo-kani for tier-B CI)")
    return code


if __name__ == "__main__":
    raise SystemExit(main())
