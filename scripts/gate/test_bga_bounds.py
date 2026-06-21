#!/usr/bin/env python3
"""ADR-0004 BGA bound + dual-probe negatives — must fail before positives."""

from __future__ import annotations

import sys
from pathlib import Path

LIB_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(LIB_DIR))

import bga_bounds_lib as bbl  # noqa: E402


def main() -> int:
    failures = 0

    negative_map_cases = (
        ("computed zero", 0, 4_194_304),
        ("bar zero", bbl.BUFFER_BYTES, 0),
        ("both zero", 0, 0),
    )
    for label, computed, bar in negative_map_cases:
        if bbl.map_bytes_rule(computed, bar) is not None:
            print(f"test_bga_bounds: FAIL {label} should return None", file=sys.stderr)
            failures += 1
        else:
            print(f"test_bga_bounds: OK rejects {label}")

    if bbl.map_bytes_rule(bbl.BUFFER_BYTES, bbl.BUFFER_BYTES + 4096) != bbl.BUFFER_BYTES:
        print(
            "test_bga_bounds: FAIL bar larger than computed should min to computed",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_bga_bounds: OK bar larger than computed")

    smaller_bar = bbl.BUFFER_BYTES - 4096
    if bbl.map_bytes_rule(bbl.BUFFER_BYTES, smaller_bar) != smaller_bar:
        print(
            "test_bga_bounds: FAIL bar smaller than computed should min to bar",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_bga_bounds: OK bar smaller than computed")

    if not bbl.dual_probe_fail_closed(0x0000, False):
        print(
            "test_bga_bounds: FAIL dual-probe (bad id, mode13 false) must fail closed",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_bga_bounds: OK dual-probe fail closed")

    if bbl.dual_probe_fail_closed(0x0000, True):
        print(
            "test_bga_bounds: FAIL mode13 fallback should not fail closed",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_bga_bounds: OK mode13 fallback allowed")

    if bbl.init_display_outcome(0xB0C5, True, False) != "bga":
        print("test_bga_bounds: FAIL BGA path outcome", file=sys.stderr)
        failures += 1
    else:
        print("test_bga_bounds: OK BGA path outcome")

    if bbl.init_display_outcome(0x0000, False, True) != "mode13h_fallback":
        print("test_bga_bounds: FAIL mode13 fallback outcome", file=sys.stderr)
        failures += 1
    else:
        print("test_bga_bounds: OK mode13 fallback outcome")

    if bbl.init_display_outcome(0x0000, False, False) != "fail_closed":
        print("test_bga_bounds: FAIL dual-probe outcome", file=sys.stderr)
        failures += 1
    else:
        print("test_bga_bounds: OK dual-probe outcome")

    if failures:
        print(f"test_bga_bounds: {failures} failure(s)", file=sys.stderr)
        return 1
    print("test_bga_bounds: all checks passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
