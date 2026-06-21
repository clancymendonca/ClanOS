#!/usr/bin/env python3
"""Host↔kernel trust-anchor byte equality — catches manual pubkey transcription errors.

Each epoch anchor TOML (`public_key_hex`) must match the embedded `[u8; 32]` array in the
corresponding kernel module. Run in validation_matrix before QEMU integration tests.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

ANCHORS: tuple[dict[str, str | Path], ...] = (
    {
        "label": "epoch-450 gate corpus (ADR-0002)",
        "toml": ROOT / "config" / "trust_anchor_epoch450.toml",
        "rust": ROOT / "kernel" / "src" / "signed_elf.rs",
        "const": "EPOCH450_PUBLIC_KEY",
    },
    {
        "label": "epoch-460 loader exec (ADR-0003)",
        "toml": ROOT / "config" / "trust_anchor_epoch460_loader.toml",
        "rust": ROOT / "kernel" / "src" / "loader_signed_exec.rs",
        "const": "EPOCH460_LOADER_PUBLIC_KEY",
    },
)


def load_toml_pubkey_hex(path: Path) -> str:
    text = path.read_text(encoding="utf-8")
    m = re.search(r'^public_key_hex\s*=\s*"([^"]+)"', text, re.M)
    if not m:
        raise ValueError(f"missing public_key_hex in {path}")
    hex_val = m.group(1).lower()
    if len(hex_val) != 64 or not re.fullmatch(r"[0-9a-f]+", hex_val):
        raise ValueError(f"invalid public_key_hex in {path}")
    return hex_val


def load_rust_embedded_pubkey(path: Path, const_name: str) -> bytes:
    text = path.read_text(encoding="utf-8")
    m = re.search(
        rf"pub const {re.escape(const_name)}: \[u8; 32\] = \[(.*?)\];",
        text,
        re.S,
    )
    if not m:
        raise ValueError(f"missing pub const {const_name} in {path}")
    tokens = re.findall(r"0x[0-9a-fA-F]{2}", m.group(1))
    if len(tokens) != 32:
        raise ValueError(
            f"{const_name} in {path}: expected 32 byte literals, got {len(tokens)}"
        )
    return bytes(int(t, 16) for t in tokens)


def main() -> int:
    failures = 0
    for entry in ANCHORS:
        label = str(entry["label"])
        toml_path = Path(entry["toml"])
        rust_path = Path(entry["rust"])
        const_name = str(entry["const"])
        try:
            toml_hex = load_toml_pubkey_hex(toml_path)
            rust_bytes = load_rust_embedded_pubkey(rust_path, const_name)
        except ValueError as exc:
            print(f"test_anchor_embed_match: FAIL {label}: {exc}", file=sys.stderr)
            failures += 1
            continue
        if rust_bytes.hex() != toml_hex:
            print(
                f"test_anchor_embed_match: FAIL {label}: kernel embed "
                f"{rust_bytes.hex()} != toml {toml_hex} "
                f"({const_name} in {rust_path.name})",
                file=sys.stderr,
            )
            failures += 1
        else:
            print(f"test_anchor_embed_match: OK {label} ({const_name} == toml anchor)")
    if failures:
        print(f"test_anchor_embed_match: {failures} failure(s)", file=sys.stderr)
        return 1
    print("test_anchor_embed_match: OK (all anchors)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
