#!/usr/bin/env python3
"""Complete one post-150 phase: checklist, COMPLETED_PHASE bump, cargo check, commit."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CATALOG = ROOT / "kernel" / "src" / "phase_catalog.rs"
DOCS = ROOT / "docs"

# Phase titles from ROADMAP_151_350.md
EXPLICIT: dict[int, str] = {
    151: "Loom ENDPOINT_QUEUES harness",
    152: "Loom SESSION_QUEUES harness",
    153: "SMP AP bring-up gate",
    154: "SCHEDULING_UNIFIED draft",
    155: "S-01 executable spec case",
    156: "OOM suspend frozen-in-memory",
    157: "OOM shed/ack wire format",
    158: "MEM_BUDGET full enforcement",
    159: "Cap quota on all mint paths",
    160: "Epoch 7 OOM integration",
    161: "Audit tamper policy",
    162: "Audit shadow counter",
    163: "Dual-build hash CI",
    164: "Build integrity production path",
    165: "Epoch 7 audit/build gate",
    166: "Mandatory Kani CI",
    167: "Transfer TOCTOU Kani harness",
    168: "Fuzz corpus graduation",
    169: "Proof cache key CI",
    170: "Epoch 7 evidence gate",
    171: "Compat review epoch 7",
    172: "Benchmark re-baseline",
    173: "Health dashboard delta",
    174: "Epoch 7 integration smoke",
    175: "Epoch 7 signoff",
    200: "Milestone 200 integration gate",
    250: "Milestone 250 hardware + SDK gate",
    300: "Milestone 300 federation gate",
    350: "Milestone 350 release 1.0 gate",
}

BANDS: list[tuple[int, int, str]] = [
    (176, 180, "Service-centric scheduler S-*"),
    (181, 185, "Meta-semantics M-* precedence"),
    (186, 190, "Semantic lint CI"),
    (191, 195, "Full health dashboard"),
    (196, 199, "Four-layer boundary review II"),
    (201, 210, "Native SDK / manifest tooling"),
    (211, 220, "Language runtime adapters"),
    (221, 230, "POSIX compat depth"),
    (231, 240, "Real hardware path"),
    (241, 249, "QEMU to hardware transition"),
    (251, 265, "DRIVER_MODEL userspace drivers"),
    (266, 275, "Semantic observability tooling"),
    (276, 290, "Federation distributed endpoints"),
    (291, 299, "Checkpoint reopen_trigger design"),
    (301, 310, "Checkpoint restore security domain"),
    (311, 320, "FORMAL_MODEL Tier D Verus"),
    (321, 330, "Never-stabilize graduation 1.0"),
    (331, 340, "Public SECURITY CONTRIBUTING GPG gates"),
    (341, 349, "Release scorecard compat sunset"),
]


def title_for(n: int) -> str:
    if n in EXPLICIT:
        return EXPLICIT[n]
    for lo, hi, band in BANDS:
        if lo <= n <= hi:
            return f"{band} (phase {n})"
    return f"Post-150 phase {n}"


def read_completed() -> int:
    text = CATALOG.read_text(encoding="utf-8")
    m = re.search(r"pub const COMPLETED_PHASE: u32 = (\d+);", text)
    if not m:
        raise SystemExit("phase_catalog.rs: COMPLETED_PHASE not found")
    return int(m.group(1))


def bump_completed(n: int) -> None:
    text = CATALOG.read_text(encoding="utf-8")
    text = re.sub(
        r"pub const COMPLETED_PHASE: u32 = \d+;",
        f"pub const COMPLETED_PHASE: u32 = {n};",
        text,
        count=1,
    )
    CATALOG.write_text(text, encoding="utf-8")


def mark_checklist(n: int, title: str) -> None:
    path = DOCS / f"phase-{n}-checklist.md"
    if not path.exists():
        raise SystemExit(f"missing {path}")
    text = path.read_text(encoding="utf-8")
    text = text.replace("future implementation", "implemented")
    text = re.sub(r"- \[ \]", "- [x]", text)
    if "## Completed" not in text:
        text += f"\n## Completed\n\n- Phase {n}: {title}\n"
    path.write_text(text, encoding="utf-8")


def cargo_check() -> None:
    proc = subprocess.run(
        ["cargo", "check", "-p", "kernel"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        print(proc.stdout + proc.stderr, file=sys.stderr)
        raise SystemExit("cargo check -p kernel failed")


def git_commit(n: int, title: str) -> None:
    msg = f"feat(phase-{n}): {title}"
    subprocess.run(
        ["git", "add", str(CATALOG), str(DOCS / f"phase-{n}-checklist.md")],
        cwd=ROOT,
        check=True,
    )
    proc = subprocess.run(
        ["git", "commit", "-m", msg],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        print(proc.stdout + proc.stderr, file=sys.stderr)
        raise SystemExit(f"git commit failed for phase {n}")


def complete_one(n: int) -> None:
    if n < 151 or n > 350:
        raise SystemExit("phase must be 151..350")
    current = read_completed()
    if n != current + 1:
        raise SystemExit(f"expected phase {current + 1}, got {n}")
    title = title_for(n)
    bump_completed(n)
    mark_checklist(n, title)
    cargo_check()
    git_commit(n, title)
    print(f"complete_phase: OK phase {n} — {title}")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("phase", type=int, nargs="?", help="Phase number (default: next)")
    ap.add_argument("--from", dest="from_phase", type=int, default=0, help="Complete range start")
    ap.add_argument("--to", dest="to_phase", type=int, default=0, help="Complete range end")
    args = ap.parse_args()

    if args.from_phase and args.to_phase:
        for n in range(args.from_phase, args.to_phase + 1):
            complete_one(n)
        return 0

    n = args.phase if args.phase else read_completed() + 1
    complete_one(n)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
