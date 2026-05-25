#!/usr/bin/env python3
import argparse
import os
import re
import signal
import subprocess
import sys
from pathlib import Path

PHASE_RE = re.compile(
    r"Phase120-CapCompat:\s+cap_table=(true|false),\s+rights=(true|false),\s+"
    r"grant=(true|false),\s+broker=(true|false),\s+compat=(true|false),\s+ok=(true|false)"
)

REPO_ROOT = Path(__file__).resolve().parents[1]


def cleanup():
    if os.name == "nt":
        subprocess.run(
            ["taskkill", "/IM", "qemu-system-x86_64.exe", "/F"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )


def run_semantic_lint() -> int:
    return subprocess.call(
        [sys.executable, str(REPO_ROOT / "scripts" / "semantic_lint.py")],
        cwd=REPO_ROOT,
    )


def run_kernel(timeout: int) -> tuple[int, str]:
    p = subprocess.Popen(
        [
            "cargo",
            "run",
            "-p",
            "kernel",
            "--features",
            "preemption",
            "--",
            "-serial",
            "stdio",
            "-display",
            "none",
            "-no-reboot",
        ],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        env=os.environ.copy(),
        cwd=REPO_ROOT,
    )
    try:
        out, _ = p.communicate(timeout=timeout)
        return p.returncode or 0, out
    except subprocess.TimeoutExpired:
        p.send_signal(signal.SIGTERM)
        try:
            out, _ = p.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            p.kill()
            out, _ = p.communicate(timeout=5)
        cleanup()
        return 124, out


def ok(output: str) -> bool:
    for line in output.splitlines():
        m = PHASE_RE.search(line)
        if m:
            return all(v == "true" for v in m.groups())
    return False


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--timeout", type=int, default=300)
    parser.add_argument("--skip-lint", action="store_true")
    args = parser.parse_args()

    if not args.skip_lint:
        if run_semantic_lint() != 0:
            print("phase120: semantic_lint failed", file=sys.stderr)
            return 1

    cleanup()
    _code, output = run_kernel(args.timeout)
    print(output[-8000:])
    if not ok(output):
        print("phase120: Phase120-CapCompat line missing or not ok", file=sys.stderr)
        return 1
    print("phase120_cap_integration_check: ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
