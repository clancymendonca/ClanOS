#!/usr/bin/env python3
"""Preemption latency validation."""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path

_SCRIPTS = Path(__file__).resolve().parents[1]
if str(_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS))

from preemption.telemetry import collect_samples  # noqa: E402

LATENCY_RE = re.compile(r"ClanOS-Preemption: name=latency\s+(.*)")
KV_RE = re.compile(
    r"(ticks|quantum|req|pts|backlog|max_backlog|est_ms|max_est_ms)=([0-9]+(?:\.[0-9]+)?)"
)


@dataclass
class Sample:
    ticks: int
    quantum: int
    req: int
    pts: int
    backlog: int
    max_backlog: int
    est_ms: int
    max_est_ms: int


def parse_sample(line: str) -> Sample | None:
    match = LATENCY_RE.search(line)
    if not match:
        return None

    values = {k: v for k, v in KV_RE.findall(match.group(1))}
    required = {"ticks", "quantum", "req", "pts", "backlog", "max_backlog", "est_ms", "max_est_ms"}
    if not required.issubset(values):
        return None

    return Sample(
        ticks=int(float(values["ticks"])),
        quantum=int(float(values["quantum"])),
        req=int(float(values["req"])),
        pts=int(float(values["pts"])),
        backlog=int(float(values["backlog"])),
        max_backlog=int(float(values["max_backlog"])),
        est_ms=int(float(values["est_ms"])),
        max_est_ms=int(float(values["max_est_ms"])),
    )


def validate(samples: list[Sample], max_latency_ms: int) -> tuple[bool, list[str]]:
    errors: list[str] = []
    if len(samples) < 2:
        return False, ["not enough latency samples captured"]

    first = samples[0]
    last = samples[-1]

    if last.req <= first.req:
        errors.append("reschedule request counter did not advance")

    if last.pts <= first.pts:
        errors.append("reschedule point counter did not advance")

    observed_max = max(sample.est_ms for sample in samples)
    if observed_max > max_latency_ms:
        errors.append(f"estimated preemption latency too high: {observed_max}ms > {max_latency_ms}ms")

    return len(errors) == 0, errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--boot-wait", type=int, default=150)
    parser.add_argument("--duration", type=int, default=120)
    parser.add_argument("--min-samples", type=int, default=5)
    parser.add_argument("--max-latency-ms", type=int, default=300)
    args = parser.parse_args()

    print(
        f"Starting preemption latency check (boot_wait={args.boot_wait}s, duration={args.duration}s after first sample)"
    )

    try:
        samples, output_tail = collect_samples(
            parse_sample,
            boot_wait=args.boot_wait,
            duration=args.duration,
        )
    except KeyboardInterrupt:
        print("Interrupted by user")
        return 130

    if len(samples) < args.min_samples:
        print(
            f"FAIL: captured only {len(samples)} latency samples, need at least {args.min_samples}"
        )
        if output_tail:
            print("Last captured output lines:")
            for line in output_tail[-20:]:
                print(f"  {line}")
        return 1

    ok, errors = validate(samples, args.max_latency_ms)
    first = samples[0]
    last = samples[-1]
    observed_max_est_ms = max(sample.est_ms for sample in samples)

    print(
        "Summary: "
        f"samples={len(samples)}, "
        f"req={first.req}->{last.req}, "
        f"pts={first.pts}->{last.pts}, "
        f"backlog={first.backlog}->{last.backlog}, "
        f"est_ms={first.est_ms}->{last.est_ms}, "
        f"max_est_ms={observed_max_est_ms}"
    )

    if not ok:
        print("FAIL:")
        for error in errors:
            print(f"  - {error}")
        return 1

    print(f"PASS: preemption latency check passed (max_est_ms <= {args.max_latency_ms})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
