#!/usr/bin/env python3

import argparse
import pathlib
import subprocess
import sys


def run(cmd: list[str]) -> int:
    print("Running:", " ".join(cmd))
    return subprocess.run(cmd, check=False).returncode


def main() -> int:
    parser = argparse.ArgumentParser(description="Phase 6+ smoke checks for shell/storage/syscall baseline.")
    parser.add_argument("--skip-build", action="store_true", help="Skip cargo check")
    args = parser.parse_args()

    root = pathlib.Path(__file__).resolve().parents[1]
    required = [
        root / "kernel/src/task/userspace.rs",
        root / "kernel/src/storage.rs",
        root / "kernel/src/syscall.rs",
        root / "docs/phase-6-checklist.md",
    ]
    missing = [str(path) for path in required if not path.exists()]
    if missing:
        print("FAIL: missing required files:")
        for item in missing:
            print(" -", item)
        return 1

    if not args.skip_build:
        rc = run(["cargo", "check", "-p", "kernel"])
        if rc != 0:
            print("FAIL: cargo check failed")
            return rc

    print("PASS: phase6 smoke checks passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
