#!/usr/bin/env python3
"""Supply-chain gate per SUPPLY_CHAIN_POLICY.md — cargo audit with triage path."""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TRIAGE_FILE = ROOT / "docs" / "SUPPLY_CHAIN_EXCEPTIONS.toml"


def cargo_audit_available() -> bool:
    try:
        subprocess.run(
            ["cargo", "audit", "--version"],
            capture_output=True,
            check=True,
            cwd=ROOT,
        )
        return True
    except (subprocess.CalledProcessError, FileNotFoundError):
        return False


def install_hint() -> str:
    return "Install: cargo install cargo-audit"


def run_audit() -> tuple[int, str]:
    proc = subprocess.run(
        ["cargo", "audit", "--json"],
        capture_output=True,
        text=True,
        cwd=ROOT,
    )
    out = proc.stdout + proc.stderr
    return proc.returncode, out


def main() -> int:
    audit_required = os.environ.get("AUDIT_REQUIRED", "").lower() in ("1", "true", "yes")
    if not cargo_audit_available():
        if audit_required:
            print("cargo_audit_check: AUDIT_REQUIRED but cargo-audit not installed", file=sys.stderr)
            return 1
        print("cargo_audit_check: SKIP — cargo-audit not installed")
        print(install_hint())
        print("Policy: advisory-only until cargo-audit present in CI image")
        return 0

    code, output = run_audit()
    if code == 0:
        print("cargo_audit_check: OK — no vulnerabilities reported")
        return 0

    if TRIAGE_FILE.exists():
        print("cargo_audit_check: FAIL — vulnerabilities found; triage file exists")
        print(f"Review {TRIAGE_FILE.relative_to(ROOT)} per SUPPLY_CHAIN_POLICY.md")
        print(output[:4000])
        return 1

    print("cargo_audit_check: FAIL — vulnerabilities found, no triage file")
    print(output[:4000])
    print(f"Create {TRIAGE_FILE.name} with documented exceptions or bump deps")
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
