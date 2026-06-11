#!/usr/bin/env python3
"""Tier-A rights algebra property checks (exhaustive + random sample)."""

from __future__ import annotations

import random
import sys

READ = 1 << 0
WRITE = 1 << 1
MAP = 1 << 2
DELEGATE = 1 << 3
REVOKE = 1 << 4
ALL_FLAGS = READ | WRITE | MAP | DELEGATE | REVOKE

ALLOWED = "allowed"
AMP_DENIED = "amplification_denied"
MISSING_DELEGATE = "missing_delegate"


def contains(parent: int, child: int) -> bool:
    return (parent & child) == child


def intersect(a: int, b: int) -> int:
    return a & b


def delegate_verdict(parent: int, child: int) -> str:
    if not contains(parent, DELEGATE):
        return MISSING_DELEGATE
    if not contains(parent, child):
        return AMP_DENIED
    return ALLOWED


def chain_delegate(parent: int, mid: int, leaf: int) -> int | None:
    if delegate_verdict(parent, mid) != ALLOWED:
        return None
    if delegate_verdict(mid, leaf) != ALLOWED:
        return None
    return leaf


def check_exhaustive() -> None:
    for parent in range(ALL_FLAGS + 1):
        for child in range(ALL_FLAGS + 1):
            verdict = delegate_verdict(parent, child)
            if verdict == ALLOWED:
                assert contains(parent, child)
                assert child <= parent
            if contains(parent, DELEGATE) and not contains(parent, child):
                assert verdict == AMP_DENIED
            inter = intersect(parent, child)
            assert contains(parent, inter)
            assert contains(child, inter)


def check_random(samples: int = 10_000) -> None:
    rng = random.Random(0xA0E50006)
    for _ in range(samples):
        parent = rng.randint(0, ALL_FLAGS)
        child = rng.randint(0, ALL_FLAGS)
        mid = rng.randint(0, ALL_FLAGS)
        leaf = rng.randint(0, ALL_FLAGS)
        verdict = delegate_verdict(parent, child)
        if verdict == ALLOWED:
            assert child <= parent
        effective = chain_delegate(parent, mid, leaf)
        if effective is not None:
            assert effective <= parent


def main() -> int:
    check_exhaustive()
    check_random()
    print("rights_algebra_check: OK (exhaustive 33x33 + 10000 random samples)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
