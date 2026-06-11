#!/usr/bin/env python3
"""Return rustc host triple for host-only workspace crates (e.g. proof-rights)."""

from __future__ import annotations

import subprocess
import sys


def host_triple() -> str:
    out = subprocess.check_output(["rustc", "-vV"], text=True)
    for line in out.splitlines():
        if line.startswith("host:"):
            return line.split(":", 1)[1].strip()
    raise RuntimeError("rustc -vV did not report host triple")


def main() -> int:
    print(host_triple())
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
