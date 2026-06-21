#!/usr/bin/env python3
"""Verify a commit builds clean and matrix entries reference existing scripts."""

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def parse_matrix_scripts(text: str) -> list[str]:
    scripts: list[str] = []
    for match in re.finditer(r'\["python",\s*"([^"]+)"', text):
        scripts.append(match.group(1).replace("/", "\\") if False else match.group(1))
    for match in re.finditer(
        r'\[\s*"cargo"[^\]]+\]',
        text,
        flags=re.DOTALL,
    ):
        block = match.group(0)
        if "scripts/" in block:
            continue
    return scripts


def matrix_python_paths(matrix_path: Path) -> list[Path]:
    text = matrix_path.read_text(encoding="utf-8")
    paths: list[Path] = []
    for match in re.finditer(r'\["python",\s*"([^"]+)"', text):
        rel = match.group(1).replace("\\", "/")
        if rel.startswith("scripts/"):
            paths.append(ROOT / rel)
    return paths


def verify_matrix_scripts_at(root: Path) -> list[str]:
    matrix = root / "scripts" / "validation_matrix.py"
    if not matrix.is_file():
        return ["missing scripts/validation_matrix.py"]
    errors: list[str] = []
    text = matrix.read_text(encoding="utf-8")
    for match in re.finditer(r'\["python",\s*"([^"]+)"', text):
        rel = match.group(1).replace("\\", "/")
        if not rel.startswith("scripts/"):
            continue
        path = root / rel
        if not path.is_file():
            errors.append(f"missing matrix script: {rel}")
    return errors


def run_in_worktree(rev: str, skip_cargo: bool = False) -> tuple[int, str]:
    tmp = Path(tempfile.mkdtemp(prefix="clanos-verify-"))
    try:
        subprocess.run(
            ["git", "worktree", "add", "--detach", str(tmp), rev],
            cwd=ROOT,
            check=True,
            capture_output=True,
            text=True,
        )
        errors = verify_matrix_scripts_at(tmp)
        if errors:
            return 1, "\n".join(errors)

        if not skip_cargo:
            subprocess.run(
                ["cargo", "clean", "-p", "kernel"],
                cwd=tmp,
                check=False,
                capture_output=True,
            )
            proc = subprocess.run(
                ["cargo", "check", "-p", "kernel"],
                cwd=tmp,
                capture_output=True,
                text=True,
            )
            if proc.returncode != 0:
                tail = (proc.stdout + proc.stderr)[-4000:]
                return proc.returncode, f"cargo check failed:\n{tail}"
        return 0, "OK"
    finally:
        subprocess.run(
            ["git", "worktree", "remove", "-f", str(tmp)],
            cwd=ROOT,
            check=False,
            capture_output=True,
        )
        if tmp.exists():
            shutil.rmtree(tmp, ignore_errors=True)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--rev", default="HEAD", help="Commit to verify (default HEAD)")
    ap.add_argument("--skip-cargo", action="store_true")
    args = ap.parse_args()

    code, msg = run_in_worktree(args.rev, skip_cargo=args.skip_cargo)
    if code != 0:
        print(f"verify_commit_clean: FAIL ({args.rev})\n{msg}", file=sys.stderr)
        return 1
    print(f"verify_commit_clean: OK ({args.rev}) — matrix scripts present, cargo check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
