#!/usr/bin/env python3

import argparse
import os
import re
import signal
import subprocess
import sys
import time

PHASE6_SMOKE_RE = re.compile(
    r"Phase6-Smoke:\s+mounted=(true|false),\s+list_ok=(true|false),\s+cat_ok=(true|false),\s+run_ok=(true|false)"
)


def is_objcopy_lock_error(output: str) -> bool:
    lowered = output.lower()
    return "llvm-objcopy" in lowered and "permission denied" in lowered


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


def cleanup_qemu_processes() -> None:
    if os.name != "nt":
        return
    subprocess.run(
        ["taskkill", "/IM", "qemu-system-x86_64.exe", "/F"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    )


def check_phase6_smoke_output(text: str) -> bool:
    for line in text.splitlines():
        match = PHASE6_SMOKE_RE.search(line)
        if not match:
            continue
        mounted, list_ok, cat_ok, run_ok = match.groups()
        return all(v == "true" for v in (mounted, list_ok, cat_ok, run_ok))
    return False


def main() -> int:
    parser = argparse.ArgumentParser(description="Run AresOS validation matrix with practical thresholds.")
    parser.add_argument("--soak-duration", type=int, default=60)
    parser.add_argument("--latency-duration", type=int, default=60)
    parser.add_argument("--max-latency-ms", type=int, default=300)
    parser.add_argument("--max-fairness-score", type=float, default=1.10)
    parser.add_argument("--smoke-timeout", type=int, default=180)
    parser.add_argument("--boot-wait", type=int, default=150, help="Seconds to wait for Phase5 telemetry after boot")
    parser.add_argument(
        "--from-check",
        type=str,
        default="",
        help="Skip checks before this name (e.g. phase14-frame-check)",
    )
    args = parser.parse_args()

    checks: list[tuple[str, list[str], int | None]] = [
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
            "phase6-smoke-qemu",
            ["cargo", "run", "-p", "kernel", "--features", "preemption", "--", "-serial", "stdio", "-display", "none", "-no-reboot"],
            args.smoke_timeout,
        ),
        (
            "phase7-storage-check",
            [
                "python",
                "scripts/phase7_storage_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase8-device-check",
            [
                "python",
                "scripts/phase8_device_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase9-loader-check",
            [
                "python",
                "scripts/phase9_loader_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase10-security-check",
            [
                "python",
                "scripts/phase10_security_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase11-image-check",
            [
                "python",
                "scripts/phase11_image_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase12-load-plan-check",
            [
                "python",
                "scripts/phase12_load_plan_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase13-mapping-stub-check",
            [
                "python",
                "scripts/phase13_mapping_stub_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase14-frame-check",
            [
                "python",
                "scripts/phase14_frame_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase15-frame-backing-check",
            [
                "python",
                "scripts/phase15_frame_backing_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase16-page-table-check",
            [
                "python",
                "scripts/phase16_page_table_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase17-user-context-check",
            [
                "python",
                "scripts/phase17_user_context_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase18-ring3-check",
            [
                "python",
                "scripts/phase18_ring3_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase19-syscall-return-check",
            [
                "python",
                "scripts/phase19_syscall_return_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase20-user-elf-check",
            [
                "python",
                "scripts/phase20_user_elf_check.py",
                "--timeout",
                str(args.smoke_timeout),
            ],
            None,
        ),
        (
            "phase21-hw-page-table-check",
            ["python", "scripts/phase21_hw_page_table_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase22-cr3-check",
            ["python", "scripts/phase22_cr3_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase23-iretq-check",
            ["python", "scripts/phase23_iretq_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase24-user-trap-check",
            ["python", "scripts/phase24_user_trap_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase25-syscall-hw-check",
            ["python", "scripts/phase25_syscall_hw_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase26-copyin-check",
            ["python", "scripts/phase26_copyin_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase27-reloc-check",
            ["python", "scripts/phase27_reloc_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase28-hw-hello-check",
            ["python", "scripts/phase28_hw_hello_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase29-allowlist-check",
            ["python", "scripts/phase29_allowlist_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase30-cr3-switch-check",
            ["python", "scripts/phase30_cr3_switch_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase31-sched-cr3-check",
            ["python", "scripts/phase31_sched_cr3_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase32-user-frame-check",
            ["python", "scripts/phase32_user_frame_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase33-multi-elf-check",
            ["python", "scripts/phase33_multi_elf_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase34-exit-wait-check",
            ["python", "scripts/phase34_exit_wait_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase35-syscall-table-check",
            ["python", "scripts/phase35_syscall_table_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase36-storage-copyin-check",
            ["python", "scripts/phase36_storage_copyin_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase37-manifest-elf-check",
            ["python", "scripts/phase37_manifest_elf_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase38-demand-zero-check",
            ["python", "scripts/phase38_demand_zero_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase39-dynamic-check",
            ["python", "scripts/phase39_dynamic_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase40-integration-check",
            ["python", "scripts/phase40_integration_check.py", "--timeout", str(max(args.smoke_timeout, 180))],
            None,
        ),
        (
            "phase41-shared-lib-check",
            ["python", "scripts/phase41_shared_lib_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase42-dyn-reloc-check",
            ["python", "scripts/phase42_dyn_reloc_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase43-trust-exec-check",
            ["python", "scripts/phase43_trust_exec_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase44-user-path-check",
            ["python", "scripts/phase44_user_path_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase45-file-fd-check",
            ["python", "scripts/phase45_file_fd_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase46-fd-io-check",
            ["python", "scripts/phase46_fd_io_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase47-file-demand-check",
            ["python", "scripts/phase47_file_demand_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase48-wx-policy-check",
            ["python", "scripts/phase48_wx_policy_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase49-smp-check",
            ["python", "scripts/phase49_smp_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase50-integration-check",
            ["python", "scripts/phase50_integration_check.py", "--timeout", str(max(args.smoke_timeout, 180))],
            None,
        ),
        (
            "phase51-proc-fd-check",
            ["python", "scripts/phase51_proc_fd_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase52-fd-dup-check",
            ["python", "scripts/phase52_fd_dup_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase53-mprotect-check",
            ["python", "scripts/phase53_mprotect_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase54-mmap-check",
            ["python", "scripts/phase54_mmap_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase55-write-path-check",
            ["python", "scripts/phase55_write_path_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase56-multi-shlib-check",
            ["python", "scripts/phase56_multi_shlib_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase57-plt-reloc-check",
            ["python", "scripts/phase57_plt_reloc_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase58-digest-trust-check",
            ["python", "scripts/phase58_digest_trust_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase59-runqueue-check",
            ["python", "scripts/phase59_runqueue_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase60-integration-check",
            ["python", "scripts/phase60_integration_check.py", "--timeout", str(max(args.smoke_timeout, 180))],
            None,
        ),
        (
            "phase61-chdir-check",
            ["python", "scripts/phase61_chdir_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase62-munmap-check",
            ["python", "scripts/phase62_munmap_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase63-vma-check",
            ["python", "scripts/phase63_vma_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase64-forklite-check",
            ["python", "scripts/phase64_forklite_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase65-ring3-syscall-check",
            ["python", "scripts/phase65_ring3_syscall_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase66-fcntl-check",
            ["python", "scripts/phase66_fcntl_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase67-lazy-plt-check",
            ["python", "scripts/phase67_lazy_plt_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase68-tlb-shootdown-check",
            ["python", "scripts/phase68_tlb_shootdown_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase69-ap-idle-check",
            ["python", "scripts/phase69_ap_idle_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase70-integration-check",
            ["python", "scripts/phase70_integration_check.py", "--timeout", str(max(args.smoke_timeout, 180))],
            None,
        ),
        (
            "phase71-sysret-check",
            ["python", "scripts/phase71_sysret_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase72-ring3-chdir-check",
            ["python", "scripts/phase72_ring3_chdir_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase73-munmap-len-check",
            ["python", "scripts/phase73_munmap_len_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase74-waitlite-check",
            ["python", "scripts/phase74_waitlite_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase75-syscallprobe-check",
            ["python", "scripts/phase75_syscallprobe_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase76-fcntl-setfd-check",
            ["python", "scripts/phase76_fcntl_setfd_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase77-ring3-lazy-plt-check",
            ["python", "scripts/phase77_ring3_lazy_plt_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase78-ipi-tlb-check",
            ["python", "scripts/phase78_ipi_tlb_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase79-ap-trampoline-check",
            ["python", "scripts/phase79_ap_trampoline_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase80-integration-check",
            ["python", "scripts/phase80_integration_check.py", "--timeout", str(max(args.smoke_timeout, 180))],
            None,
        ),
        (
            "phase81-hw-sysret-check",
            ["python", "scripts/phase81_hw_sysret_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase82-getcwd-check",
            ["python", "scripts/phase82_getcwd_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase83-chdirprobe-check",
            ["python", "scripts/phase83_chdirprobe_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase84-vma-split-check",
            ["python", "scripts/phase84_vma_split_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase85-fork-dup-check",
            ["python", "scripts/phase85_fork_dup_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase86-exec-lite-check",
            ["python", "scripts/phase86_exec_lite_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase87-pipe-lite-check",
            ["python", "scripts/phase87_pipe_lite_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase88-ring3-plt-fault-check",
            ["python", "scripts/phase88_ring3_plt_fault_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase89-ipi-send-check",
            ["python", "scripts/phase89_ipi_send_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase90-integration-check",
            ["python", "scripts/phase90_integration_check.py", "--timeout", str(max(args.smoke_timeout, 300))],
            None,
        ),
        (
            "phase91-fork-cow-check",
            ["python", "scripts/phase91_fork_cow_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase92-poll-lite-check",
            ["python", "scripts/phase92_poll_lite_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase93-mmap-gap-check",
            ["python", "scripts/phase93_mmap_gap_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase94-exec-argv-check",
            ["python", "scripts/phase94_exec_argv_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase95-pipe-probe-check",
            ["python", "scripts/phase95_pipe_probe_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase96-vma-coalesce-check",
            ["python", "scripts/phase96_vma_coalesce_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase97-work-steal-check",
            ["python", "scripts/phase97_work_steal_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase98-ap-runnable-check",
            ["python", "scripts/phase98_ap_runnable_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase99-lapic-icr-check",
            ["python", "scripts/phase99_lapic_icr_check.py", "--timeout", str(args.smoke_timeout)],
            None,
        ),
        (
            "phase100-integration-check",
            ["python", "scripts/phase100_integration_check.py", "--timeout", str(max(args.smoke_timeout, 300))],
            None,
        ),
        (
            "semantic-lint-check",
            ["python", "scripts/semantic_lint.py"],
            None,
        ),
        (
            "phase110-constitutional-check",
            [
                "python",
                "scripts/phase110_constitutional_check.py",
                "--timeout",
                str(max(args.smoke_timeout, 300)),
            ],
            None,
        ),
        (
            "phase5-soak-check",
            [
                "python",
                "scripts/phase5_soak_check.py",
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
            "phase5-latency-check",
            [
                "python",
                "scripts/phase5_latency_check.py",
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
        names = [name for name, _, _ in checks]
        if args.from_check not in names:
            print(f"Unknown --from-check name: {args.from_check}")
            print("Known checks:", ", ".join(names))
            return 2
        start_idx = names.index(args.from_check)
        checks = checks[start_idx:]
        print(f"Resuming from {args.from_check} ({len(checks)} checks)")

    phase5_timeout = args.boot_wait + max(args.soak_duration, args.latency_duration) + 180

    any_failed = False
    cleanup_qemu_processes()
    print("Validation matrix start")
    for name, cmd, timeout in checks:
        if name in ("phase5-soak-check", "phase5-latency-check"):
            timeout = phase5_timeout
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
        elif os.name == "nt":
            time.sleep(5)

        if name == "phase6-smoke-qemu":
            smoke_ok = check_phase6_smoke_output(output)
            if not smoke_ok:
                print("FAIL: Phase6-Smoke line missing or indicates false flags")
                any_failed = True
                continue
            print("PASS: Phase6-Smoke runtime path validated")

        if code != 0 and not (name == "phase6-smoke-qemu" and code == 124):
            print(f"FAIL: {name} exited with {code} in {elapsed:.1f}s")
            any_failed = True
        elif code == 124 and name == "phase6-smoke-qemu":
            # Expected timeout for non-terminating kernel run once smoke line is observed.
            print(f"PASS: {name} reached steady-state timeout in {elapsed:.1f}s")
        else:
            print(f"PASS: {name} in {elapsed:.1f}s")

    if any_failed:
        print("\nValidation matrix: FAIL")
        return 1
    print("\nValidation matrix: PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
