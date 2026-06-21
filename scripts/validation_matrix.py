#!/usr/bin/env python3

import argparse
import os
import re
import signal
import subprocess
import sys
import time

from smoke_qemu import (
    cleanup_qemu_processes,
    ensure_qemu_on_path,
    is_objcopy_lock_error,
    wait_for_bootimage_unlock,
)

# Deprecated matrix check names → canonical (ADR-0001 unified validation gate).
CHECK_ALIASES: dict[str, str] = {
    "boot-gate-host-check": "gate-host-check",
    "boot-gate-check": "gate-check",
    "system-gate-host-check": "gate-host-check",
    "system-gate-check": "gate-check",
}


def run_command_with_retries(
    cmd: list[str], timeout: int | None = None, max_attempts: int = 4
) -> tuple[int, str]:
    last_code, last_output = 0, ""
    for attempt in range(1, max_attempts + 1):
        last_code, last_output = run_command(cmd, timeout=timeout)
        if last_code == 0 or not is_objcopy_lock_error(last_output):
            return last_code, last_output
        if attempt < max_attempts:
            print(
                f"Retry {attempt}/{max_attempts - 1}: llvm-objcopy lock; "
                "waiting 12s before retry..."
            )
            cleanup_qemu_processes()
            time.sleep(12)
    return last_code, last_output


def run_command(cmd: list[str], timeout: int | None = None) -> tuple[int, str]:
    ensure_qemu_on_path()
    process = subprocess.Popen(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        env=os.environ.copy(),
    )
    try:
        output, _ = process.communicate(timeout=timeout)
        return process.returncode or 0, output
    except subprocess.TimeoutExpired:
        process.send_signal(signal.SIGTERM)
        try:
            output, _ = process.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill()
            output, _ = process.communicate(timeout=5)
        cleanup_qemu_processes()
        return 124, output



def main() -> int:
    parser = argparse.ArgumentParser(description="Run Clan OS validation matrix with practical thresholds.")
    parser.add_argument("--soak-duration", type=int, default=60)
    parser.add_argument("--latency-duration", type=int, default=60)
    parser.add_argument("--max-latency-ms", type=int, default=300)
    parser.add_argument("--max-fairness-score", type=float, default=1.10)
    parser.add_argument("--smoke-timeout", type=int, default=180)
    parser.add_argument("--boot-wait", type=int, default=150, help="Seconds to wait for preemption telemetry after boot")
    parser.add_argument(
        "--from-check",
        type=str,
        default="",
        help="Skip checks before this name (e.g. gate-check)",
    )
    args = parser.parse_args()

    checks: list[tuple[str, list[str], int | None]] = [
        ("epoch0-doc-link", ["python", "scripts/doc_link_check.py"], None),
        ("epoch0-project-health", ["python", "scripts/project_health.py"], None),
        ("epoch0-proof-rights", ["python", "scripts/proof_rights_test.py"], 120),
        ("epoch0-kani-gate", ["python", "scripts/kani_gate.py"], 600),
        ("epoch0-cargo-audit", ["python", "scripts/cargo_audit_check.py"], 180),
        ("cargo-check", ["cargo", "check", "-p", "kernel"], None),
        (
            "preemption-integration",
            [
                "cargo",
                "test",
                "-p",
                "kernel",
                "--features",
                "preemption",
                "--test",
                "preemption_integration",
            ],
            900,
        ),
        (
            "gate-host-check",
            ["python", "scripts/gate/gate_host.py"],
            None,
        ),
        (
            "gate-check",
            [
                "python",
                "scripts/gate/run.py",
                "--gate",
                "all",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "compat-subsystems-host-check",
            ["python", "scripts/gate/compat_subsystems.py"],
            180,
        ),
        (
            "gate-posix-server-host-check",
            ["python", "scripts/gate/posix_server.py"],
            60,
        ),
        (
            "gate-signal-skeleton-host-check",
            ["python", "scripts/gate/signal_skeleton.py"],
            60,
        ),
        (
            "gate-honesty-check-part-a",
            ["python", "scripts/gate/gate_honesty_check.py", "--part", "a"],
            30,
        ),
        (
            "gate-honesty-check-part-b",
            ["python", "scripts/gate/gate_honesty_check.py", "--part", "b"],
            30,
        ),
        (
            "module-wiring-check",
            ["python", "scripts/gate/module_wiring_check.py"],
            30,
        ),
        (
            "module-wiring-self-test",
            ["python", "scripts/gate/test_module_wiring_check.py"],
            30,
        ),
        (
            "gate-honesty-self-test",
            ["python", "scripts/gate/test_gate_honesty_check.py"],
            30,
        ),
        (
            "gap-registry-audit",
            ["python", "scripts/gate/gap_registry_audit.py"],
            60,
        ),
        (
            "gap-registry-audit-self-test",
            ["python", "scripts/gate/test_gap_registry_audit.py"],
            30,
        ),
        (
            "close-remaining-plan-gaps-self-test",
            ["python", "scripts/gate/test_close_remaining_plan_gaps.py"],
            30,
        ),
        (
            "gate-signed-elf-host-check",
            ["python", "scripts/gate/signed_elf.py"],
            30,
        ),
        (
            "gate-signed-elf-self-test",
            ["python", "scripts/gate/test_signed_elf.py"],
            30,
        ),
        (
            "trust-anchor-embed-match",
            ["python", "scripts/gate/test_anchor_embed_match.py"],
            30,
        ),
        (
            "gate-loader-signed-exec-host-check",
            ["python", "scripts/gate/loader_signed_exec.py"],
            30,
        ),
        (
            "gate-loader-signed-exec-self-test",
            ["python", "scripts/gate/test_loader_signed_exec.py"],
            60,
        ),
        (
            "loader-signing-sunset-check",
            ["python", "scripts/gate/loader_signing_sunset_check.py"],
            30,
        ),
        (
            "loader-signing-sunset-self-test",
            ["python", "scripts/gate/test_loader_signing_sunset_check.py"],
            30,
        ),
        (
            "loader-seed-migration-rollback-proof",
            ["python", "scripts/gate/test_loader_seed_migration_rollback.py"],
            30,
        ),
        (
            "loader-signed-exec-kernel-integration",
            [
                "cargo",
                "test",
                "-p",
                "kernel",
                "--test",
                "loader_signed_exec_integration",
            ],
            300,
        ),
        (
            "signed-elf-kernel-integration",
            [
                "cargo",
                "test",
                "-p",
                "kernel",
                "--test",
                "signed_elf_integration",
            ],
            300,
        ),
        ("semantic-lint", ["python", "scripts/semantic_lint.py"], None),
        ("covenant-ci", ["python", "scripts/covenant_ci.py"], 120),
        ("loom-gate", ["python", "scripts/loom_gate.py"], None),
        ("transfer-toctou-check", ["python", "scripts/transfer_toctou_check.py"], None),
        (
            "preemption-soak-check",
            [
                "python",
                "scripts/preemption/soak.py",
                "--duration",
                str(args.soak_duration),
                "--boot-wait",
                str(args.boot_wait),
                "--min-samples",
                "2",
                "--max-score",
                str(args.max_fairness_score),
            ],
            None,
        ),
        (
            "preemption-latency-check",
            [
                "python",
                "scripts/preemption/latency.py",
                "--duration",
                str(args.latency_duration),
                "--boot-wait",
                str(args.boot_wait),
                "--min-samples",
                "2",
                "--max-latency-ms",
                str(args.max_latency_ms),
            ],
            None,
        ),
    ]

    if args.from_check:
        args.from_check = CHECK_ALIASES.get(args.from_check, args.from_check)
        names = [name for name, _, _ in checks]
        if args.from_check not in names:
            print(f"Unknown --from-check name: {args.from_check}")
            print("Known checks:", ", ".join(names))
            return 2
        start_idx = names.index(args.from_check)
        checks = checks[start_idx:]
        print(f"Resuming from {args.from_check} ({len(checks)} checks)")

    preemption_timeout = args.boot_wait + max(args.soak_duration, args.latency_duration) + 180

    any_failed = False
    cleanup_qemu_processes()
    if not ensure_qemu_on_path():
        print(
            "WARN: qemu-system-x86_64 not on PATH and not found in common install dirs; "
            "QEMU checks will fail (install: winget install SoftwareFreedomConservancy.QEMU)"
        )
    print("Validation matrix start")
    for name, cmd, timeout in checks:
        if name in ("preemption-soak-check", "preemption-latency-check"):
            timeout = preemption_timeout
        if name == "gate-check":
            timeout = max(timeout or 0, args.smoke_timeout, 480)
        print(f"\n=== {name} ===")
        print("Command:", " ".join(cmd))
        start = time.time()
        code, output = run_command_with_retries(cmd, timeout=timeout)
        elapsed = time.time() - start
        print(output[-4000:])
        cleanup_qemu_processes()
        if name == "preemption-integration" and os.name == "nt":
            print("Waiting for bootimage artifacts to unlock (Windows)...")
            time.sleep(20)
            wait_for_bootimage_unlock()
        else:
            wait_for_bootimage_unlock()

        if code != 0:
            print(f"FAIL: {name} exited with {code} in {elapsed:.1f}s")
            any_failed = True
        else:
            print(f"PASS: {name} in {elapsed:.1f}s")

    if any_failed:
        print("\nValidation matrix: FAIL")
        return 1
    print("\nValidation matrix: PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
