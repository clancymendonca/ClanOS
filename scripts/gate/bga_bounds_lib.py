"""BGA LFB map bound rule — host mirror of kernel/src/bga.rs (ADR-0004 Q4)."""

from __future__ import annotations

BUFFER_BYTES = 1024 * 768 * 4


def map_bytes_rule(computed_size: int, bar_size: int) -> int | None:
    if computed_size <= 0 or bar_size <= 0:
        return None
    return min(computed_size, bar_size)


def dual_probe_fail_closed(bga_id: int, mode13_ok: bool, expected_id: int = 0xB0C5) -> bool:
    """Returns True when init must fail closed (desktop gates false)."""
    if bga_id == expected_id:
        return False
    return not mode13_ok


def init_display_outcome(bga_id: int, bga_path_ok: bool, mode13_ok: bool) -> str:
    if bga_path_ok and bga_id == 0xB0C5:
        return "bga"
    if bga_id != 0xB0C5 and mode13_ok:
        return "mode13h_fallback"
    return "fail_closed"
