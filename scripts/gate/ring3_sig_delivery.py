#!/usr/bin/env python3
"""Host check: sig-demo ring-3 ELF builds and wires signal syscalls."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
ELF = REPO / "target" / "x86_64-unknown-none" / "release" / "sig-demo"
MAX_IMAGE_SIZE = 32768


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=180)
    args = ap.parse_args()
    r = subprocess.run(
        [
            "cargo",
            "build",
            "-p",
            "sig-demo",
            "--release",
            "--target",
            "x86_64-unknown-none",
        ],
        cwd=REPO,
        capture_output=True,
        text=True,
        timeout=args.timeout,
    )
    if r.returncode != 0:
        print(r.stdout + r.stderr, file=sys.stderr)
        print("gate/ring3_sig_delivery: FAIL (build)", file=sys.stderr)
        return 1
    if not ELF.is_file():
        print(f"gate/ring3_sig_delivery: missing ELF at {ELF}", file=sys.stderr)
        return 1
    if ELF.stat().st_size > MAX_IMAGE_SIZE:
        print("gate/ring3_sig_delivery: ELF exceeds MAX_IMAGE_SIZE", file=sys.stderr)
        return 1
    main_rs = REPO / "userland" / "sig-demo" / "src" / "main.rs"
    text = main_rs.read_text(encoding="utf-8")
    for needle in ("sys_sigaction", "sys_kill", "sys_sigreturn", "sigusr1_handler"):
        if needle not in text:
            print(f"gate/ring3_sig_delivery: missing {needle}", file=sys.stderr)
            return 1
    ring3 = REPO / "userland" / "src" / "ring3_syscall.rs"
    if "sys_sigreturn" not in ring3.read_text(encoding="utf-8"):
        print("gate/ring3_sig_delivery: missing sys_sigreturn stub", file=sys.stderr)
        return 1
    loader = REPO / "kernel" / "src" / "task" / "program_loader.rs"
    if "ring3_sig_delivery_smoke" not in loader.read_text(encoding="utf-8"):
        print("gate/ring3_sig_delivery: smoke not wired", file=sys.stderr)
        return 1
    print("gate/ring3_sig_delivery: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
