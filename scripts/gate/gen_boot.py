#!/usr/bin/env python3
"""Regenerate validation_gate.rs early smokes from template (manual merge for run_* bodies)."""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "kernel" / "src" / "validation_gate.rs"
MARKER = "#[allow(unused_variables)]\nfn run_sched_userspace_smokes()"

HEADER = '''//! Unified boot-time validation gate (subsystem smokes consolidated).
//!
//! Subsystem serial lines replace legacy numbered boot markers.

pub const BOOT_GATE_VERSION: &str = "1.0.0";

fn ok_str(v: bool) -> &'static str {
    if v {
        "true"
    } else {
        "false"
    }
}

fn emit(name: &str, ok: bool) {
    crate::serial_println!("ClanOS-BootGate: name={} ok={}", name, ok_str(ok));
}

pub fn smoke_shell_storage() -> bool {
    let storage_smoke_ok = match crate::storage::list_files() {
        Ok(files) => !files.is_empty(),
        Err(_) => false,
    };
    let readme_smoke_ok = matches!(crate::storage::read_file("/README.txt"), Ok(Some(_)));
    let run_smoke_ok = crate::task::userspace::run_program("echo", &["shell-storage-smoke"]).is_ok();
    crate::storage::is_mounted()
        && storage_smoke_ok
        && readme_smoke_ok
        && run_smoke_ok
        && crate::storage::smoke_persistence()
        && crate::storage::smoke_driver_backend()
}

pub fn smoke_loader_security() -> bool {
    crate::task::program_loader::smoke_program_discovery()
        && crate::security::smoke_access_policy()
        && crate::storage::smoke_cred_enforcement()
        && crate::task::program_loader::smoke_elf_inventory()
        && crate::task::program_loader::smoke_load_plan()
        && crate::task::program_loader::smoke_mapping_stub()
}

pub fn smoke_memory_layout() -> bool {
    crate::frame_ownership::smoke_frame_registry()
        && crate::task::program_loader::smoke_frame_backing()
        && crate::task::program_loader::smoke_hw_page_tables()
}

pub fn smoke_userspace_bootstrap() -> bool {
    crate::task::program_loader::smoke_user_context()
        && crate::task::program_loader::smoke_ring3_trampoline()
        && crate::task::program_loader::smoke_user_syscall_probe()
        && crate::task::program_loader::smoke_minimal_user_elf()
}

/// Run all boot subsystems and emit unified serial gate lines.
pub fn run_boot_gate() {
    let shell = smoke_shell_storage();
    emit("shell_storage", shell);
    let loader = smoke_loader_security();
    emit("loader_security", loader);
    let memory = smoke_memory_layout();
    emit("memory_layout", memory);
    let userspace = smoke_userspace_bootstrap();
    emit("userspace_bootstrap", userspace);

    crate::serial_println!("Boot: hw userspace gates start");
    let (hw_paging, sched, dynamic, fd_mmap, vm_fork) =
        x86_64::instructions::interrupts::without_interrupts(|| {
            let hw = run_hw_paging_smokes();
            let s = run_sched_userspace_smokes();
            let d = run_dynamic_runtime_smokes();
            let f = run_fd_mmap_smokes();
            let v = run_vm_fork_smokes();
            (hw, s, d, f, v)
        });
    emit("hw_paging", hw_paging);
    emit("sched_userspace", sched);
    emit("dynamic_runtime", dynamic);
    emit("fd_mmap", fd_mmap);
    emit("vm_fork", vm_fork);

    let syscall_ring3 = run_syscall_ring3_smokes();
    emit("syscall_ring3", syscall_ring3);

    let path_exec = run_path_exec_smokes();
    emit("path_exec", path_exec);

    let smp_depth = run_smp_depth_smokes();
    emit("smp_depth", smp_depth);

    let constitutional = run_constitutional_smokes();
    emit("constitutional", constitutional);

    let capabilities = run_capabilities_smokes();
    emit("capabilities", capabilities);

    let service_loader = run_service_loader_smoke();
    emit("service_loader", service_loader);

    let platform = run_platform_broker_smokes();
    emit("platform_brokers", platform);

    let virtio = run_virtio_blk_smoke();
    emit("virtio_blk", virtio);

    let _ = crate::storage::ensure_filesystem_on_active();
    let build = run_build_endpoint_smokes();
    emit("build_endpoints", build);

    let network = run_network_compat_smokes();
    emit("network_compat", network);

    let scheduler = run_scheduler_epoch_smokes();
    emit("scheduler_epoch", scheduler);

    let boundary = run_boundary_smoke();
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
    crate::serial_println!("ClanOS-BootGate: ok={}", ok_str(boot_ok));
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
