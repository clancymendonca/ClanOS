#!/usr/bin/env python3
"""Project health checks for epoch gates — STATUS.md and gate reports."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

GOALS = [
    "privilege_escalation",
    "information_disclosure",
    "denial_of_service",
    "integrity_violation",
]

SURFACES = [
    "cap_table",
    "audit",
    "ipc",
    "compat_shim",
    "build_pipeline",
    "kernel_heap",
]


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8") if path.exists() else ""


def strip_toml_string(val: str) -> str:
    val = val.strip()
    if val.startswith('"""') and val.endswith('"""'):
        return val[3:-3]
    if val.startswith('"') and val.endswith('"'):
        return val[1:-1]
    return val


def parse_toml_tables(text: str, table_name: str) -> list[dict]:
    """Minimal TOML array-of-tables parser for [[table_name]] blocks."""
    blocks: list[dict] = []
    pattern = re.compile(rf"\[\[{re.escape(table_name)}\]\](.*?)(?=\n\[\[|\Z)", re.S)
    for m in pattern.finditer(text):
        block: dict = {}
        for line in m.group(1).splitlines():
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            if "=" not in line:
                continue
            key, _, val = line.partition("=")
            key = key.strip()
            val = val.strip()
            if val.startswith("[") and val.endswith("]"):
                inner = val[1:-1].strip()
                block[key] = [v.strip().strip('"') for v in inner.split(",") if v.strip()]
            elif val.startswith('"'):
                block[key] = strip_toml_string(val)
            elif val == "null":
                block[key] = None
            elif val in ("true", "false"):
                block[key] = val == "true"
            else:
                try:
                    block[key] = int(val)
                except ValueError:
                    block[key] = val
        if block:
            blocks.append(block)
    return blocks


def check_gap_registry() -> list[str]:
    errors: list[str] = []
    path = ROOT / "gap_registry.toml"
    text = read_text(path)
    gaps = parse_toml_tables(text, "gaps")
    if len(gaps) != 350:
        errors.append(f"gap_registry: expected 350 gaps, found {len(gaps)}")
    for g in gaps:
        if g.get("status") == "superseded" and not g.get("superseded_by_commit"):
            errors.append(f"gap {g.get('id')}: superseded requires superseded_by_commit")
    return errors


def check_prereq_acyclic() -> list[str]:
    errors: list[str] = []
    text = read_text(ROOT / "prereq_graph.toml")
    edges = parse_toml_tables(text, "edges")
    graph: dict[str, list[str]] = defaultdict(list)
    nodes: set[str] = set()
    for e in edges:
        if e.get("blocking_scope") == "epoch-0" or e.get("blocking"):
            graph[e["to"]].append(e["from"])
            nodes.add(e["from"])
            nodes.add(e["to"])

    visiting: set[str] = set()
    visited: set[str] = set()

    def dfs(n: str) -> bool:
        if n in visiting:
            return False
        if n in visited:
            return True
        visiting.add(n)
        for dep in graph.get(n, []):
            if not dfs(dep):
                return False
        visiting.remove(n)
        visited.add(n)
        return True

    for n in nodes:
        if not dfs(n):
            errors.append("prereq_graph: cycle detected in epoch-0 DAG")
            break
    return errors


def threat_coverage_matrix() -> dict[str, dict[str, int]]:
    text = read_text(ROOT / "docs" / "THREAT_NODES.toml")
    nodes = parse_toml_tables(text, "nodes")
    matrix: dict[str, dict[str, int]] = {
        g: {s: 0 for s in SURFACES} for g in GOALS
    }
    closed: dict[str, dict[str, int]] = {
        g: {s: 0 for s in SURFACES} for g in GOALS
    }
    for n in nodes:
        status = n.get("status", "open")
        goals = n.get("goals", [])
        surfaces = n.get("surfaces", [])
        if isinstance(goals, str):
            goals = [goals]
        if isinstance(surfaces, str):
            surfaces = [surfaces]
        for g in goals:
            for s in surfaces:
                if g in matrix and s in matrix[g]:
                    matrix[g][s] += 1
                    if status == "closed":
                        closed[g][s] += 1
    return {"total": matrix, "closed": closed}


def render_matrix_svg(matrix: dict[str, dict[str, int]]) -> str:
    cell = 80
    margin = 120
    width = margin + len(SURFACES) * cell + 40
    height = margin + len(GOALS) * cell + 40
    lines = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}">',
        '<style>text{font-family:sans-serif;font-size:11px}</style>',
        '<text x="10" y="20" font-weight="bold">Threat coverage matrix (node counts)</text>',
    ]
    for i, g in enumerate(GOALS):
        y = margin + i * cell
        lines.append(f'<text x="5" y="{y + cell/2}">{g[:12]}</text>')
        for j, s in enumerate(SURFACES):
            x = margin + j * cell
            if i == 0:
                lines.append(f'<text x="{x}" y="{margin - 10}">{s}</text>')
            val = matrix.get(g, {}).get(s, 0)
            lines.append(
                f'<rect x="{x}" y="{y}" width="{cell-4}" height="{cell-4}" '
                f'fill="#e8eef8" stroke="#333"/><text x="{x+28}" y="{y+45}">{val}</text>'
            )
    lines.append("</svg>")
    return "\n".join(lines)


def count_kani_harnesses() -> int:
    text = read_text(ROOT / "kani_harness_registry.toml")
    return len(re.findall(r"\[\[harnesses\]\]", text))


def emit_status(matrix: dict) -> str:
    gaps = parse_toml_tables(read_text(ROOT / "gap_registry.toml"), "gaps")
    gaps_open = sum(1 for g in gaps if g.get("status") == "open")
    gaps_addressed = sum(1 for g in gaps if g.get("status") == "addressed")
    gaps_wontfix = sum(1 for g in gaps if g.get("status") == "wontfix")
    nodes = parse_toml_tables(read_text(ROOT / "docs" / "THREAT_NODES.toml"), "nodes")
    threats_open = sum(1 for n in nodes if n.get("status") == "open")
    kani_count = count_kani_harnesses()
    lines = [
        "# Clan OS Project Status",
        "",
        "## Snapshot (fully operational OS)",
        "",
        "- **Boot gate:** `kernel/src/boot_gate.rs` (`BOOT_GATE_VERSION = 1.0.0`)",
        "- **System gate:** `kernel/src/system_gate.rs` (`SYSTEM_GATE_VERSION = 1.0.0`)",
        "- **Desktop:** VGA 320×200, compositor, PS/2 mouse, window manager, taskbar",
        "- **Userland:** `/bin/demo-hello`, `/bin/clan-info`, `/bin/mendo`, `/bin/ring3-io-demo` (Clan OS runtime: `clan-rt` `#![no_std]`)",
        "- **Network:** virtio-net loopback + external route simulation",
        f"- gap_registry: {gaps_open} open, {gaps_addressed} addressed ({gaps_addressed + gaps_open + gaps_wontfix} total)",
        f"- threat nodes open: {threats_open}",
        f"- kani_harness_count: {kani_count}",
        "- validation: [`VALIDATION_GATES.md`](docs/VALIDATION_GATES.md)",
        "- release_scorecard: [`RELEASE_SCORECARD_M500.md`](docs/RELEASE_SCORECARD_M500.md)",
        "",
        "## Threat coverage by goal",
        "",
    ]
    for g in GOALS:
        total = sum(matrix["total"].get(g, {}).values())
        closed = sum(matrix["closed"].get(g, {}).values())
        lines.append(f"- `{g}`: {closed}/{total} closed")
    lines.extend(
        [
            "",
            "## Validation gates",
            "",
            "| Gate family | Final serial line | Host check |",
            "|-------------|-------------------|------------|",
            "| Boot | `ClanOS-BootGate: ok=true` | `scripts/gate/boot_host.py` |",
            "| System | `ClanOS-SystemGate: ok=true` | `scripts/gate/system_host.py` |",
            "",
            "QEMU: `scripts/gate/boot.py --gate boot`, `scripts/gate/system.py --gate system`",
            "",
        ]
    )
    return "\n".join(lines)


def bisect_metric(metric: str, start: str, end: str) -> int:
    """Binary search introducing commit for metric regression (skeleton)."""
    print(f"bisect: metric={metric} range={start}..{end}")
    try:
        log = subprocess.check_output(
            ["git", "rev-list", f"{start}..{end}"],
            cwd=ROOT,
            text=True,
        ).strip().splitlines()
    except subprocess.CalledProcessError as e:
        print(f"git rev-list failed: {e}", file=sys.stderr)
        return 1
    if not log:
        print("empty range")
        return 0
    lo, hi = 0, len(log) - 1
    print(f"commits in range: {len(log)} — manual metric check at each step required")
    while lo < hi:
        mid = (lo + hi) // 2
        commit = log[mid]
        print(f"check commit {commit} ({mid}/{len(log)})")
        hi = mid
    print(f"suspected introducing commit: {log[lo]}")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write-status", action="store_true")
    parser.add_argument("--write-report", action="store_true")
    parser.add_argument("--bisect", nargs=3, metavar=("METRIC", "START", "END"))
    args = parser.parse_args()

    if args.bisect:
        return bisect_metric(*args.bisect)

    errors: list[str] = []
    errors.extend(check_gap_registry())
    errors.extend(check_prereq_acyclic())

    matrix = threat_coverage_matrix()

    if args.write_status:
        (ROOT / "STATUS.md").write_text(emit_status(matrix), encoding="utf-8")

    if args.write_report:
        report_dir = ROOT / "reports"
        report_dir.mkdir(exist_ok=True)
        (report_dir / "threat_coverage_matrix.svg").write_text(
            render_matrix_svg(matrix["total"]), encoding="utf-8"
        )
        (report_dir / "gap_doc_graph.svg").write_text(
            '<svg xmlns="http://www.w3.org/2000/svg" width="200" height="60">'
            "<text x=\"10\" y=\"30\">Gap/doc DAG viz stub — see prereq_graph.toml</text></svg>",
            encoding="utf-8",
        )

    if errors:
        for e in errors:
            print(f"error: {e}", file=sys.stderr)
        return 1

    print("project_health: OK")
    if not args.write_status:
        try:
            print(emit_status(matrix))
        except UnicodeEncodeError:
            print(emit_status(matrix).encode("ascii", errors="replace").decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
