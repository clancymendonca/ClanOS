#!/usr/bin/env python3
"""One-shot helper: merge boot_gate.rs + system_gate.rs → validation_gate.rs.

Migration completed 2026-06-20 (ADR-0001). Retained for archaeology only.
"""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
boot_path = ROOT / "kernel/src/_boot_gate_orig.rs"
system_path = ROOT / "kernel/src/_system_gate_orig.rs"
if not boot_path.exists():
    boot_path = ROOT / "kernel/src/boot_gate.rs"
if not system_path.exists():
    system_path = ROOT / "kernel/src/system_gate.rs"

boot = boot_path.read_text(encoding="utf-8-sig")
system = system_path.read_text(encoding="utf-8-sig")
boot_lines = boot.splitlines()

run_start = next(i for i, line in enumerate(boot_lines) if line.startswith("pub fn run_boot_gate"))
boot_gate_fn = next(i for i, line in enumerate(boot_lines) if line.startswith("pub fn boot_gate"))
smoke_shell = next(i for i, line in enumerate(boot_lines) if line.startswith("pub fn smoke_shell_storage"))

depth = 0
run_body_end = run_start
for i in range(run_start, len(boot_lines)):
    depth += boot_lines[i].count("{") - boot_lines[i].count("}")
    if i > run_start and depth == 0:
        run_body_end = i + 1
        break

early_smokes = "\n".join(boot_lines[smoke_shell:run_start])
boot_body = "\n".join(boot_lines[run_body_end:boot_gate_fn])
boot_body = boot_body.replace(
    "/// Composite boot gate (no serial emission).",
    "/// Composite boot subsystem smokes (no serial emission).",
)

sys_lines = system.splitlines()
sys_start = next(i for i, line in enumerate(sys_lines) if line.startswith("use core::sync"))
sys_end = next(i for i, line in enumerate(sys_lines) if line.startswith("fn ok_str"))
system_body = "\n".join(sys_lines[sys_start:sys_end])
system_body = system_body.replace(
    "crate::milestone150::smoke_milestone_boundary()",
    "crate::boundary_gate::smoke_boundary()",
)

header = """//! Unified validation gate — all subsystem smokes at boot.
//!
//! Serial: `ClanOS-Gate: name=<subsystem> ok=<bool>` and summary `ClanOS-Gate: ok=<bool>`.
//! Legacy aliases optional via `VALIDATION_GATE_EMIT_LEGACY_ALIASES`.

pub const VALIDATION_GATE_VERSION: &str = "2.0.0";
const VALIDATION_GATE_EMIT_LEGACY_ALIASES: bool = true;

"""

ok_str_emit = """fn ok_str(v: bool) -> &'static str {
    if v {
        "true"
    } else {
        "false"
    }
}

fn emit(name: &str, ok: bool, legacy_boot: bool) {
    crate::serial_println!("ClanOS-Gate: name={} ok={}", name, ok_str(ok));
    if legacy_boot && VALIDATION_GATE_EMIT_LEGACY_ALIASES {
        crate::serial_println!("ClanOS-BootGate: name={} ok={}", name, ok_str(ok));
    }
}

"""

run_fn_text = """/// Evaluate all subsystems and emit unified serial gate lines.
pub fn run_validation_gate() {
    let shell = smoke_shell_storage();
    emit("shell_storage", shell, true);
    let loader = smoke_loader_security();
    emit("loader_security", loader, true);
    let memory = smoke_memory_layout();
    emit("memory_layout", memory, true);
    let userspace = smoke_userspace_bootstrap();
    emit("userspace_bootstrap", userspace, true);

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
    emit("hw_paging", hw_paging, true);
    emit("sched_userspace", sched, true);
    emit("dynamic_runtime", dynamic, true);
    emit("fd_mmap", fd_mmap, true);
    emit("vm_fork", vm_fork, true);

    let syscall_ring3 = run_syscall_ring3_smokes();
    emit("syscall_ring3", syscall_ring3, true);
    let path_exec = run_path_exec_smokes();
    emit("path_exec", path_exec, true);
    let smp_depth = run_smp_depth_smokes();
    emit("smp_depth", smp_depth, true);
    let constitutional = run_constitutional_smokes();
    emit("constitutional", constitutional, true);
    let capabilities = run_capabilities_smokes();
    emit("capabilities", capabilities, true);
    let service_loader = run_service_loader_smoke();
    emit("service_loader", service_loader, true);
    let platform = run_platform_broker_smokes();
    emit("platform_brokers", platform, true);
    let virtio = run_virtio_blk_smoke();
    emit("virtio_blk", virtio, true);

    let _ = crate::storage::ensure_filesystem_on_active();
    let build = run_build_endpoint_smokes();
    emit("build_endpoints", build, true);
    let network_compat = run_network_compat_smokes();
    emit("network_compat", network_compat, true);
    let scheduler = run_scheduler_epoch_smokes();
    emit("scheduler_epoch", scheduler, true);
    let boundary = run_boundary_smoke();
    emit("boundary", boundary, true);

    let boot_ok = shell && loader && memory && userspace && hw_paging && sched && dynamic
        && fd_mmap && vm_fork && syscall_ring3 && path_exec && smp_depth && constitutional
        && capabilities && service_loader && platform && virtio && build && network_compat
        && scheduler && boundary;

    let integrity = integrity_gate();
    emit("integrity", integrity, false);
    let scheduling = scheduling_gate();
    emit("scheduling", scheduling, false);
    let hardware = hardware_gate();
    emit("hardware", hardware, false);
    let federation = federation_gate();
    emit("federation", federation, false);
    let release = release_gate();
    emit("release", release, false);
    let desktop_preview = desktop_preview_gate();
    emit("desktop_preview", desktop_preview, false);
    let desktop = desktop_gate();
    emit("desktop", desktop, false);
    let compat_runtime = smoke_compat_runtime();
    emit("compat_runtime", compat_runtime, false);
    let compat_fd_vm = smoke_compat_fd_vm();
    emit("compat_fd_vm", compat_fd_vm, false);
    let compat_signal = smoke_compat_signal();
    emit("compat_signal", compat_signal, false);
    let storage_depth = smoke_storage_depth();
    emit("storage_depth", storage_depth, false);
    let posix_compat = smoke_posix_compat();
    emit("posix_compat", posix_compat, false);

    if compat_runtime && compat_fd_vm && compat_signal && storage_depth && posix_compat {
        COMPAT_SUBSYSTEMS_OK.store(true, Ordering::Release);
    }

    let functional = functional_gate();
    emit("functional", functional, false);
    let ci = ci_gate();
    emit("ci", ci, false);
    let production = production_gate();
    emit("production", production, false);
    let network = network_gate();
    emit("network", network, false);
    let all_ok = boot_ok && system_gate();

    if VALIDATION_GATE_EMIT_LEGACY_ALIASES {
        crate::serial_println!("ClanOS-BootGate: ok={}", ok_str(boot_ok));
        crate::serial_println!("ClanOS-SystemGate: ok={}", ok_str(all_ok));
    }
    crate::serial_println!("ClanOS-Gate: ok={}", ok_str(all_ok));
}

/// Composite validation gate (no serial emission).
pub fn validation_gate_composite() -> bool {
    boot_gate() && system_gate()
}
"""

out = header + ok_str_emit + early_smokes + "\n" + boot_body + "\n" + system_body + "\n" + run_fn_text
(ROOT / "kernel/src/validation_gate.rs").write_text(out, encoding="utf-8")
print(f"wrote validation_gate.rs ({len(out.splitlines())} lines)")
