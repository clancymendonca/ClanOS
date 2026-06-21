#!/usr/bin/env python3
"""Prove seed migration rollback path — deliberate break then revert (ADR-0003 § Seed migration).

Exercises builtin-alias (manifest field tamper) and elf64-image (ELF payload tamper) paths.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ANCHOR = ROOT / "config" / "trust_anchor_epoch460_loader.toml"
ALLOWLIST = ROOT / "config" / "loader_digest_only_allowlist.toml"
SEED_DIR = ROOT / "config" / "loader_signed_seed"
SAMPLE_ELF = SEED_DIR / "fixtures" / "sample_elf_fixture.bin"
BUILTIN_PROGRAM = "demo-hello"
ELF_PROGRAM = "tickprobe"
EXT2_PROGRAM = "ring3-io-demo-ext2"
RING3_ELF = SEED_DIR / "fixtures" / "ring3_io_demo.bin"
# digest-only remaining after demo-hello…tickprobe + ring3-io-demo-ext2 migrated.
EXPECTED_DIGEST_ONLY_REMAINING = 2

sys.path.insert(0, str(Path(__file__).resolve().parent))
import loader_signed_exec_lib as lsel  # noqa: E402


def load_allowlist_names() -> list[str]:
    text = ALLOWLIST.read_text(encoding="utf-8")
    return re.findall(r'name\s*=\s*"([^"]+)"', text)


def rollback_builtin_manifest(name: str, entry: str, description: str) -> str:
    return (
        f"clan-exec-v1\n"
        f"name={name}\n"
        f"kind=builtin-alias\n"
        f"entry={entry}\n"
        f"requires=execute\n"
        f"trust=system\n"
        f"owner=admin\n"
        f"description={description}\n"
    )


def rollback_elf_manifest(
    name: str,
    entry: str,
    image: str,
    description: str,
) -> str:
    return (
        f"clan-exec-v1\n"
        f"name={name}\n"
        f"kind=elf64-image\n"
        f"entry={entry}\n"
        f"image={image}\n"
        f"requires=execute\n"
        f"trust=system\n"
        f"owner=admin\n"
        f"description={description}\n"
    )


def prove_builtin_alias_rollback(anchor: lsel.TrustAnchor, failures: int) -> int:
    signed_path = SEED_DIR / f"{BUILTIN_PROGRAM}.signed.manifest"
    if not signed_path.is_file():
        print(
            f"test_loader_seed_migration_rollback: FAIL missing {signed_path}",
            file=sys.stderr,
        )
        return failures + 1

    signed = signed_path.read_text(encoding="utf-8")
    ok, msg = lsel.verify_signed_builtin_alias(signed, anchor)
    if not ok:
        print(
            f"test_loader_seed_migration_rollback: FAIL signed {BUILTIN_PROGRAM}: {msg}",
            file=sys.stderr,
        )
        failures += 1
    else:
        print(f"test_loader_seed_migration_rollback: OK signed {BUILTIN_PROGRAM} verifies")

    broken = signed.replace("entry=demo-hello", "entry=demo-evil", 1)
    ok, msg = lsel.verify_signed_builtin_alias(broken, anchor)
    if ok:
        print(
            "test_loader_seed_migration_rollback: FAIL builtin tampered entry should not verify",
            file=sys.stderr,
        )
        failures += 1
    elif "signature verify failed" not in msg and "digest payload mismatch" not in msg:
        print(
            f"test_loader_seed_migration_rollback: FAIL builtin tampered entry wrong reason: {msg}",
            file=sys.stderr,
        )
        failures += 1
    else:
        print(f"test_loader_seed_migration_rollback: OK builtin rejects tampered entry ({msg})")

    rb = rollback_builtin_manifest(BUILTIN_PROGRAM, BUILTIN_PROGRAM, "clan-rt demo")
    if "trust=system-signed" in rb or "digest=sha256:" in rb or "sig=ed25519:" in rb:
        print(
            "test_loader_seed_migration_rollback: FAIL builtin rollback shape",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_loader_seed_migration_rollback: OK builtin rollback manifest is digest-only trust=system")

    ok, msg = lsel.verify_signed_builtin_alias(rb, anchor)
    if ok:
        print(
            "test_loader_seed_migration_rollback: FAIL builtin rollback must not pass signed verify",
            file=sys.stderr,
        )
        failures += 1
    else:
        print(
            f"test_loader_seed_migration_rollback: OK builtin rollback rejected by signed path ({msg})"
        )

    return failures


def prove_elf64_image_rollback(anchor: lsel.TrustAnchor, failures: int) -> int:
    signed_path = SEED_DIR / f"{ELF_PROGRAM}.signed.manifest"
    if not signed_path.is_file():
        print(
            f"test_loader_seed_migration_rollback: FAIL missing {signed_path}",
            file=sys.stderr,
        )
        return failures + 1
    if not SAMPLE_ELF.is_file():
        print(
            f"test_loader_seed_migration_rollback: FAIL missing {SAMPLE_ELF}",
            file=sys.stderr,
        )
        return failures + 1

    elf_bytes = SAMPLE_ELF.read_bytes()
    signed = signed_path.read_text(encoding="utf-8")
    ok, msg = lsel.verify_signed_manifest(signed, elf_bytes, anchor)
    if not ok:
        print(
            f"test_loader_seed_migration_rollback: FAIL signed {ELF_PROGRAM}: {msg}",
            file=sys.stderr,
        )
        failures += 1
    else:
        print(f"test_loader_seed_migration_rollback: OK signed {ELF_PROGRAM} verifies against ELF payload")

    # Deliberate break: tampered ELF payload bytes (not manifest field).
    tampered_elf = bytearray(elf_bytes)
    tampered_elf[0] ^= 0x01
    ok, msg = lsel.verify_signed_manifest(signed, bytes(tampered_elf), anchor)
    if ok:
        print(
            "test_loader_seed_migration_rollback: FAIL elf tampered payload should not verify",
            file=sys.stderr,
        )
        failures += 1
    elif "digest payload mismatch" not in msg:
        print(
            f"test_loader_seed_migration_rollback: FAIL elf tampered payload wrong reason: {msg}",
            file=sys.stderr,
        )
        failures += 1
    else:
        print(f"test_loader_seed_migration_rollback: OK elf rejects tampered payload ({msg})")

    rb = rollback_elf_manifest(
        ELF_PROGRAM,
        "0x400000",
        "/bin/tickprobe.elf",
        "Tick probe ELF fixture",
    )
    rollback_path = SEED_DIR / f"{ELF_PROGRAM}.rollback.manifest"
    rollback_path.write_text(rb, encoding="utf-8", newline="\n")
    if "trust=system-signed" in rb or "digest=sha256:" in rb or "sig=ed25519:" in rb:
        print(
            "test_loader_seed_migration_rollback: FAIL elf rollback shape",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_loader_seed_migration_rollback: OK elf rollback manifest is digest-only trust=system")

    ok, msg = lsel.verify_signed_manifest(rb, elf_bytes, anchor)
    if ok:
        print(
            "test_loader_seed_migration_rollback: FAIL elf rollback must not pass signed verify",
            file=sys.stderr,
        )
        failures += 1
    else:
        print(
            f"test_loader_seed_migration_rollback: OK elf rollback rejected by signed path ({msg})"
        )

    return failures


def prove_ext2_image_path_rollback(anchor: lsel.TrustAnchor, failures: int) -> int:
    """Confirm /ext2/… image= uses same SHA256(ELF bytes) digest, not a third variant."""
    signed_path = SEED_DIR / f"{EXT2_PROGRAM}.signed.manifest"
    if not signed_path.is_file():
        print(
            f"test_loader_seed_migration_rollback: FAIL missing {signed_path}",
            file=sys.stderr,
        )
        return failures + 1
    if not RING3_ELF.is_file():
        print(
            f"test_loader_seed_migration_rollback: FAIL missing {RING3_ELF}",
            file=sys.stderr,
        )
        return failures + 1

    elf_bytes = RING3_ELF.read_bytes()
    signed = signed_path.read_text(encoding="utf-8")
    if "image=/ext2/ring3-io-demo.elf" not in signed:
        print(
            "test_loader_seed_migration_rollback: FAIL ext2 manifest missing /ext2/ image path",
            file=sys.stderr,
        )
        failures += 1

    ok, msg = lsel.verify_signed_manifest(signed, elf_bytes, anchor)
    if not ok:
        print(
            f"test_loader_seed_migration_rollback: FAIL signed {EXT2_PROGRAM}: {msg}",
            file=sys.stderr,
        )
        failures += 1
    else:
        print(
            f"test_loader_seed_migration_rollback: OK signed {EXT2_PROGRAM} "
            "(/ext2/ image path, ELF-byte digest) verifies"
        )

    tampered_elf = bytearray(elf_bytes)
    tampered_elf[0] ^= 0x01
    ok, msg = lsel.verify_signed_manifest(signed, bytes(tampered_elf), anchor)
    if ok:
        print(
            "test_loader_seed_migration_rollback: FAIL ext2 elf tampered payload should not verify",
            file=sys.stderr,
        )
        failures += 1
    elif "digest payload mismatch" not in msg:
        print(
            f"test_loader_seed_migration_rollback: FAIL ext2 tampered payload wrong reason: {msg}",
            file=sys.stderr,
        )
        failures += 1
    else:
        print(f"test_loader_seed_migration_rollback: OK ext2 rejects tampered payload ({msg})")

    rb = rollback_elf_manifest(
        EXT2_PROGRAM,
        "0x400000",
        "/ext2/ring3-io-demo.elf",
        "Ring-3 I/O demo from ext2",
    )
    if "trust=system-signed" in rb or "digest=sha256:" in rb or "sig=ed25519:" in rb:
        print(
            "test_loader_seed_migration_rollback: FAIL ext2 rollback shape",
            file=sys.stderr,
        )
        failures += 1
    else:
        print("test_loader_seed_migration_rollback: OK ext2 rollback manifest is digest-only trust=system")

    ok, msg = lsel.verify_signed_manifest(rb, elf_bytes, anchor)
    if ok:
        print(
            "test_loader_seed_migration_rollback: FAIL ext2 rollback must not pass signed verify",
            file=sys.stderr,
        )
        failures += 1
    else:
        print(
            f"test_loader_seed_migration_rollback: OK ext2 rollback rejected by signed path ({msg})"
        )

    return failures


def main() -> int:
    failures = 0
    anchor = lsel.load_trust_anchor(ANCHOR)

    failures = prove_builtin_alias_rollback(anchor, failures)
    failures = prove_elf64_image_rollback(anchor, failures)
    failures = prove_ext2_image_path_rollback(anchor, failures)

    names = load_allowlist_names()
    for prog in (BUILTIN_PROGRAM, ELF_PROGRAM, EXT2_PROGRAM):
        if prog in names:
            print(
                f"test_loader_seed_migration_rollback: FAIL {prog} still on digest-only allowlist",
                file=sys.stderr,
            )
            failures += 1
    if len(names) != EXPECTED_DIGEST_ONLY_REMAINING:
        print(
            "test_loader_seed_migration_rollback: FAIL allowlist count "
            f"{len(names)} expected {EXPECTED_DIGEST_ONLY_REMAINING} digest-only remaining",
            file=sys.stderr,
        )
        failures += 1
    else:
        names_restored = names + [EXT2_PROGRAM]
        print(
            "test_loader_seed_migration_rollback: OK re-add to allowlist restores "
            f"digest-only staging ({len(names)} digest-only remaining -> "
            f"{len(names_restored)} after simulated rollback)"
        )

    if failures:
        print(f"test_loader_seed_migration_rollback: {failures} failure(s)", file=sys.stderr)
        return 1
    print(
        "test_loader_seed_migration_rollback: OK "
        "(builtin + elf + ext2-image-path rollback paths proven)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
