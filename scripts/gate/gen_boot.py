#!/usr/bin/env python3
"""Regenerate boot_gate.rs header from template; preserves exec_* bodies in place."""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "kernel" / "src" / "boot_gate.rs"
MARKER = "#[allow(unused_variables)]\nfn exec_phases_6_to_20()"

HEADER = '''//! Unified boot-time validation gate (phases 6–150 consolidated).
//!
//! Subsystem serial lines replace per-phase `PhaseN-*` boot markers.

pub const BOOT_GATE_VERSION: &str = "1.0.0";

fn ok_str(v: bool) -> &'static str {
    if v {
        "true"
    } else {
        "false"
    }
}

fn emit(name: &str, ok: bool) {
    crate::serial_println!("AresOS-BootGate: name={} ok={}", name, ok_str(ok));
}

fn eval_shell_storage() -> bool {
    let storage_smoke_ok = match crate::storage::list_files() {
        Ok(files) => !files.is_empty(),
        Err(_) => false,
    };
    let readme_smoke_ok = matches!(crate::storage::read_file("/README.txt"), Ok(Some(_)));
    let run_smoke_ok = crate::task::userspace::run_program("echo", &["phase6-smoke"]).is_ok();
    crate::storage::is_mounted()
        && storage_smoke_ok
        && readme_smoke_ok
        && run_smoke_ok
        && crate::storage::phase7_smoke_check()
        && crate::storage::phase8_smoke_check()
}

fn eval_loader_security() -> bool {
    crate::task::program_loader::phase9_smoke_check()
        && crate::security::phase10_smoke_check()
        && crate::storage::phase10_smoke_check()
        && crate::task::program_loader::phase11_smoke_check()
        && crate::task::program_loader::phase12_smoke_check()
        && crate::task::program_loader::phase13_smoke_check()
}

fn eval_memory_layout() -> bool {
    crate::frame_ownership::phase14_smoke_check()
        && crate::task::program_loader::phase15_smoke_check()
        && crate::task::program_loader::phase16_smoke_check()
}

fn eval_userspace_bootstrap() -> bool {
    crate::task::program_loader::phase17_smoke_check()
        && crate::task::program_loader::phase18_smoke_check()
        && crate::task::program_loader::phase19_smoke_check()
        && crate::task::program_loader::phase20_smoke_check()
}

/// Run phases 6–150 side effects and emit unified subsystem gate lines.
pub fn run_boot_gate() {
    let _ = exec_phases_6_to_20();
    let shell = eval_shell_storage();
    emit("shell_storage", shell);
    let loader = eval_loader_security();
    emit("loader_security", loader);
    let memory = eval_memory_layout();
    emit("memory_layout", memory);
    let userspace = eval_userspace_bootstrap();
    emit("userspace_bootstrap", userspace);

    crate::serial_println!("Boot: hw userspace gates start");
    let (hw_paging, sched, dynamic, fd_mmap, vm_fork) =
        x86_64::instructions::interrupts::without_interrupts(|| {
            let hw = exec_phase21_to_30_smokes();
            let s = exec_phase31_to_40_smokes();
            let d = exec_phase41_to_50_smokes();
            let f = exec_phase51_to_60_smokes();
            let v = exec_phase61_to_70_smokes();
            (hw, s, d, f, v)
        });
    emit("hw_paging", hw_paging);
    emit("sched_userspace", sched);
    emit("dynamic_runtime", dynamic);
    emit("fd_mmap", fd_mmap);
    emit("vm_fork", vm_fork);

    let syscall_ring3 = exec_phase71_to_80_smokes();
    emit("syscall_ring3", syscall_ring3);

    let path_exec = exec_phase81_to_90_smokes();
    emit("path_exec", path_exec);

    let smp_depth = exec_phase91_to_100_smokes();
    emit("smp_depth", smp_depth);

    let constitutional = exec_phase101_to_110_smokes();
    emit("constitutional", constitutional);

    let capabilities = exec_phase111_to_120_smokes();
    emit("capabilities", capabilities);

    let service_loader = exec_phase121_smoke();
    emit("service_loader", service_loader);

    let platform = exec_phase122_to_130_smokes();
    emit("platform_brokers", platform);

    let virtio = exec_phase201_virtio_smoke();
    emit("virtio_blk", virtio);

    let _ = crate::storage::ensure_filesystem_on_active();
    let build = exec_phase131_to_140_smokes();
    emit("build_endpoints", build);

    let network = exec_epoch4_network_smokes();
    emit("network_compat", network);

    let scheduler = exec_epoch5_scheduler_smokes();
    emit("scheduler_epoch", scheduler);

    let boundary = exec_milestone150();
    emit("boundary", boundary);

    let boot_ok = shell
        && loader
        && memory
        && userspace
        && hw_paging
        && sched
        && dynamic
        && fd_mmap
        && vm_fork
        && syscall_ring3
        && path_exec
        && smp_depth
        && constitutional
        && capabilities
        && service_loader
        && platform
        && virtio
        && build
        && network
        && scheduler
        && boundary;
    crate::serial_println!("AresOS-BootGate: ok={}", ok_str(boot_ok));
}

'''


def main() -> int:
    if not OUT.exists():
        raise SystemExit(f"missing {OUT}; bootstrap from git history")
    text = OUT.read_text(encoding="utf-8")
    idx = text.find(MARKER)
    if idx < 0:
        raise SystemExit(f"{OUT}: marker {MARKER!r} not found")
    body = text[idx:]
    OUT.write_text(HEADER + body, encoding="utf-8")
    print(f"gate/gen_boot: refreshed header in {OUT} ({len(body)} bytes body preserved)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
