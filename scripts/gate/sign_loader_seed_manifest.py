#!/usr/bin/env python3
"""Sign a seed /bin/* clan-exec-v1 manifest for ADR-0003 migration (host only)."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ANCHOR = ROOT / "config" / "trust_anchor_epoch460_loader.toml"
SEED_DIR = ROOT / "config" / "loader_signed_seed"

sys.path.insert(0, str(Path(__file__).resolve().parent))
import loader_signed_exec_lib as lsel  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--name", required=True)
    ap.add_argument("--kind", choices=("builtin-alias", "elf64-image"), required=True)
    ap.add_argument("--entry", required=True)
    ap.add_argument("--image", default="")
    ap.add_argument("--description", default="")
    ap.add_argument("--owner", default="admin")
    ap.add_argument(
        "--out",
        type=Path,
        help="Write signed manifest (default: config/loader_signed_seed/<name>.signed.manifest)",
    )
    ap.add_argument(
        "--elf",
        type=Path,
        help="ELF file for kind=elf64-image digest (required for elf64-image)",
    )
    args = ap.parse_args()

    if args.kind == "elf64-image":
        if not args.image or not args.elf or not args.elf.is_file():
            print("sign_loader_seed_manifest: --image and --elf required for elf64-image", file=sys.stderr)
            return 1
        digest_payload = args.elf.read_bytes()
    else:
        digest_payload = lsel.builtin_alias_digest_payload(args.name, args.entry)

    digest_hex = lsel.sha256_hex(digest_payload)
    manifest = lsel.ExecManifest(
        name=args.name,
        kind=args.kind,
        entry=args.entry,
        image=args.image or None,
        requires="execute",
        digest_hex=digest_hex,
        trust=lsel.TRUST_SIGNED,
        owner=args.owner,
        description=args.description or None,
        signature_hex=None,
    )
    sk = lsel.epoch460_loader_dev_private_key()
    signed = lsel.sign_manifest(manifest, sk)

    out = args.out or (SEED_DIR / f"{args.name}.signed.manifest")
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(signed, encoding="utf-8", newline="\n")

    anchor = lsel.load_trust_anchor(ANCHOR)
    if args.kind == "builtin-alias":
        ok, msg = lsel.verify_signed_builtin_alias(signed, anchor)
    else:
        ok, msg = lsel.verify_signed_manifest(signed, digest_payload, anchor)
    if not ok:
        print(f"sign_loader_seed_manifest: FAIL self-verify: {msg}", file=sys.stderr)
        return 1
    print(f"sign_loader_seed_manifest: OK wrote {out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
