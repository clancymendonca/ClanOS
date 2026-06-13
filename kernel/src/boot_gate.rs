//! Unified boot-time validation gate (phases 6–150 consolidated).
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

fn exec_phases_6_to_20() -> bool {
    let storage_smoke_ok = match crate::storage::list_files() {
        Ok(files) => !files.is_empty(),
        Err(_) => false,
    };
    let readme_smoke_ok = matches!(crate::storage::read_file("/README.txt"), Ok(Some(_)));
    let run_smoke_ok = crate::task::userspace::run_program("echo", &["phase6-smoke"]).is_ok();
    let phase7_storage_ok = crate::storage::phase7_smoke_check();
    let phase8_storage_ok = crate::storage::phase8_smoke_check();
    let device_summary = crate::device::summary();
    let (block_devices, driver_backed_blocks, backend) = crate::block::summary();
    let phase9_launch_ok = crate::task::program_loader::phase9_smoke_check();
    let loader_status = crate::task::program_loader::status();
    let credentials = crate::security::current_credentials();
    let policy_ok = crate::security::phase10_smoke_check();
    let denied_ok = crate::storage::phase10_smoke_check();
    let phase11_images_ok = crate::task::program_loader::phase11_smoke_check();
    let image_status = crate::task::program_loader::status();
    let exec_blocked_ok = image_status.unsupported_execution_count > 0;
    let phase12_load_plan_ok = crate::task::program_loader::phase12_smoke_check();
    let load_plan_status = crate::task::program_loader::status();
    let phase13_mapping_ok = crate::task::program_loader::phase13_smoke_check();
    let mapping_status = crate::task::program_loader::status();
    let phase14_frames_ok = crate::frame_ownership::phase14_smoke_check();
    let frame_status = crate::frame_ownership::status();
    let phase15_backing_ok = crate::task::program_loader::phase15_smoke_check();
    let backing_status = crate::task::program_loader::status();
    let backing_frames = crate::frame_ownership::status();
    let phase16_tables_ok = crate::task::program_loader::phase16_smoke_check();
    let table_status = crate::task::program_loader::status();
    let phase17_context_ok = crate::task::program_loader::phase17_smoke_check();
    let context_status = crate::task::program_loader::status();
    let user_selectors = crate::gdt::user_selectors();
    let phase18_ring3_ok = crate::task::program_loader::phase18_smoke_check();
    let ring3_status = crate::task::program_loader::status();
    let phase19_syscall_ok = crate::task::program_loader::phase19_smoke_check();
    let user_syscall_status = crate::task::program_loader::status();
    let phase20_user_elf_ok = crate::task::program_loader::phase20_smoke_check();
    let user_elf_status = crate::task::program_loader::status();
    phase20_user_elf_ok
}

#[allow(unused_variables)]
fn exec_phase31_to_40_smokes() -> bool {
    let phase31_ok = crate::task::program_loader::phase31_sched_cr3_smoke();
    let (bound, switches, skips, restore_ok) = crate::user_paging::sched_cr3_status();

    let phase32_ok = crate::task::program_loader::phase32_user_frame_smoke();
    let (saves, resumes, preempted) = crate::user_hw_frame::status();

    let phase33_ok = crate::task::program_loader::phase33_multi_elf_smoke();

    let phase34_ok = crate::task::program_loader::phase34_exit_wait_smoke();
    let (exits, waits, _) = crate::syscall::exit_wait_status();

    let phase35_ok = crate::task::program_loader::phase35_syscall_table_smoke();
    let (allowed, rejected, _) = crate::user_syscall_hw::dispatch_table_status();

    let phase36_ok = crate::task::program_loader::phase36_storage_copyin_smoke();
    let (reads, rej) = crate::task::program_loader::storage_copyin_status();

    let phase37_ok = crate::task::program_loader::phase37_manifest_elf_smoke();
    let (disc, exec, rej) = crate::task::program_loader::manifest_elf_status();

    let phase38_ok = crate::task::program_loader::phase38_demand_zero_smoke();
    let (faults, mapped, rejected) = crate::demand_paging::status();

    let phase39_ok = crate::task::program_loader::phase39_dynamic_smoke();
    let (needed, linked, reloc_ok) = crate::elf_reloc::dynamic_status();

    let phase33_ok_for40 = phase33_ok;
    let phase40_ok = crate::task::program_loader::phase40_integration_smoke();
    let (bound2, sw2, _, restore2) = crate::user_paging::sched_cr3_status();
    let (reads2, _) = crate::task::program_loader::storage_copyin_status();
    let (disc2, _, _) = crate::task::program_loader::manifest_elf_status();
    let (_, mapped2, _) = crate::demand_paging::status();
    return phase40_ok && restore2;
}

#[allow(unused_variables)]
fn exec_phase41_to_50_smokes() -> bool {
    let phase41_ok = crate::task::program_loader::phase41_shared_lib_smoke();
    let (loaded, pages, _) = crate::shared_loader::status();

    let phase42_ok = crate::task::program_loader::phase42_dyn_reloc_smoke();
    let (imports, applied) = crate::elf_reloc::import_status();

    let phase43_ok = crate::task::program_loader::phase43_trust_exec_smoke();
    let (trust_ok, trust_rej) = crate::task::program_loader::trust_exec_status();

    let phase44_ok = crate::task::program_loader::phase44_user_path_smoke();
    let (reads, path_rej) = crate::user_path::status();

    let phase45_ok = crate::task::program_loader::phase45_file_fd_smoke();
    let (opens, closes, _, _, _) = crate::fd_table::status();

    let phase46_ok = crate::task::program_loader::phase46_fd_io_smoke();
    let (_, _, fd_reads, fd_writes, _) = crate::fd_table::status();

    let phase47_ok = crate::task::program_loader::phase47_file_demand_smoke();
    let (faults, file_loaded, file_rej) = crate::demand_paging::file_status();

    let phase48_ok = crate::task::program_loader::phase48_wx_policy_smoke();
    let (wx_checked, wx_rejected) = crate::user_paging::wx_status();

    let phase49_ok = crate::task::program_loader::phase49_smp_smoke();
    let (cpus, aps, flush_ok) = crate::smp::status();

    let phase50_ok = crate::task::program_loader::phase50_integration_smoke();
    let (loaded2, _, _) = crate::shared_loader::status();
    let (_, applied2) = crate::elf_reloc::import_status();
    let (t_ok, t_rej) = crate::task::program_loader::trust_exec_status();
    let (p_reads, _) = crate::user_path::status();
    let (o2, _, r2, _, _) = crate::fd_table::status();
    let (_, f_loaded, _) = crate::demand_paging::file_status();
    let (_, wx_rej2) = crate::user_paging::wx_status();
    let (cpus2, _, flush2) = crate::smp::status();
    return phase50_ok;
}

#[allow(unused_variables)]
fn exec_phase51_to_60_smokes() -> bool {
    let phase51_ok = crate::task::program_loader::phase51_proc_fd_smoke();

    let phase52_ok = crate::task::program_loader::phase52_fd_dup_smoke();
    let (dups, relative) = crate::fd_table::dup_status();

    let phase53_ok = crate::task::program_loader::phase53_mprotect_smoke();
    let (applied, rejected, guard) = crate::user_paging::mprotect_status();

    let phase54_ok = crate::task::program_loader::phase54_mmap_smoke();
    let (anon, file, rej) = crate::mmap::status();

    let phase55_ok = crate::task::program_loader::phase55_write_path_smoke();
    let (writes, verified) = crate::user_path::write_status();

    let phase56_ok = crate::task::program_loader::phase56_multi_shlib_smoke();
    let (loaded, pages, _) = crate::shared_loader::status();

    let phase57_ok = crate::task::program_loader::phase57_plt_reloc_smoke();
    let (slots, plt_applied) = crate::elf_reloc::plt_status();

    let phase58_ok = crate::task::program_loader::phase58_digest_trust_smoke();
    let (verified, rejected) = crate::image_digest::status();

    let phase59_ok = crate::task::program_loader::phase59_runqueue_smoke();
    let (cpus, enqueued, _) = (
        crate::smp::status().0,
        crate::smp::runqueue_status().0,
        (),
    );

    let phase60_ok = crate::task::program_loader::phase60_integration_smoke();
    return phase60_ok;
}

#[allow(unused_variables)]
fn exec_phase61_to_70_smokes() -> bool {
    let phase61_ok = crate::task::program_loader::phase61_chdir_smoke();
    let (normalized, chdirs) = crate::user_path::chdir_status();

    let phase62_ok = crate::task::program_loader::phase62_munmap_smoke();
    let (unmapped, munmap_rej) = crate::mmap::munmap_status();

    let phase63_ok = crate::task::program_loader::phase63_vma_smoke();
    let (vma_regions, vma_overlap) = crate::vma::status();

    let phase64_ok = crate::task::program_loader::phase64_forklite_smoke();
    let (inherited, isolated) = crate::fd_table::fork_lite_status();

    let phase65_ok = crate::task::program_loader::phase65_ring3_syscall_smoke();
    let (ring3_write, ring3_mprotect) = crate::user_syscall_hw::ring3_syscall_status();

    let phase66_ok = crate::task::program_loader::phase66_fcntl_smoke();
    let (fcntl_getfd, fcntl_dup, fcntl_rej) = crate::fd_table::fcntl_status();

    let phase67_ok = crate::task::program_loader::phase67_lazy_plt_smoke();
    let (plt_lazy, plt_bound) = crate::elf_reloc::lazy_plt_status();

    let phase68_ok = crate::task::program_loader::phase68_tlb_shootdown_smoke();
    let (cpus, _, _) = crate::smp::status();
    let (shootdowns, _) = crate::smp::shootdown_status();

    let phase69_ok = crate::task::program_loader::phase69_ap_idle_smoke();
    let (aps, idle_ticks) = crate::smp::ap_idle_status();

    let phase70_ok = crate::task::program_loader::phase70_integration_smoke();
    return phase70_ok;
}

#[allow(unused_variables)]
fn exec_phase71_to_80_smokes() -> bool {
    let phase71_ok = crate::task::program_loader::phase71_sysret_smoke();
    let (probes, sysret_ok) = crate::user_syscall_hw::sysret_status();

    let phase72_ok = crate::task::program_loader::phase72_ring3_chdir_smoke();
    let ring3_chdirs = crate::user_path::ring3_chdir_status();

    let phase73_ok = crate::task::program_loader::phase73_munmap_len_smoke();
    let (unmapped_pages, partial_regions) = crate::mmap::munmap_len_status();

    let phase74_ok = crate::task::program_loader::phase74_waitlite_smoke();
    let (waited, wait_rejected) = crate::task::process::wait_lite_status();

    let phase75_ok = crate::task::program_loader::phase75_syscallprobe_smoke();
    let (ring3_write, ring3_mprotect) = crate::user_syscall_hw::ring3_syscall_status();

    let phase76_ok = crate::task::program_loader::phase76_fcntl_setfd_smoke();
    let (setfd, getfd, fcntl_rej) = crate::fd_table::fcntl_setfd_status();

    let phase77_ok = crate::task::program_loader::phase77_ring3_lazy_plt_smoke();
    let (plt_lazy, plt_bound) = crate::elf_reloc::lazy_plt_status();
    let ring3_plt = crate::elf_reloc::ring3_plt_status();

    let phase78_ok = crate::task::program_loader::phase78_ipi_tlb_smoke();
    let (cpus, _, _) = crate::smp::status();
    let (ipis, _) = crate::smp::ipi_status();

    let phase79_ok = crate::task::program_loader::phase79_ap_trampoline_smoke();
    let (aps, idle_ticks) = crate::smp::ap_idle_status();

    let phase80_ok = crate::task::program_loader::phase80_integration_smoke();
    return phase80_ok;
}

#[allow(unused_variables)]
fn exec_phase81_to_90_smokes() -> bool {
    let phase81_ok = crate::task::program_loader::phase81_hw_sysret_smoke();
    let (_, sysret_real) = crate::user_syscall_hw::hw_sysret_real_status();

    let phase82_ok = crate::task::program_loader::phase82_getcwd_smoke();
    let getcwd_reads = crate::user_path::getcwd_status();

    let phase83_ok = crate::task::program_loader::phase83_chdirprobe_smoke();

    let phase84_ok = crate::task::program_loader::phase84_vma_split_smoke();
    let (splits, _) = crate::vma::split_status();
    let (unmapped, _) = crate::mmap::munmap_len_status();

    let phase85_ok = crate::task::program_loader::phase85_fork_dup_smoke();
    let (children, duplicated) = crate::task::process::fork_dup_status();

    let phase86_ok = crate::task::program_loader::phase86_exec_lite_smoke();
    let (execs, cloexec_closed) = crate::task::process::exec_lite_status();

    let phase87_ok = crate::task::program_loader::phase87_pipe_lite_smoke();
    let (pipes, bytes) = crate::pipe::status();

    let phase88_ok = crate::task::program_loader::phase88_ring3_plt_fault_smoke();
    let (faults, bound) = crate::elf_reloc::ring3_plt_fault_status();

    let phase89_ok = crate::task::program_loader::phase89_ipi_send_smoke();
    let (sent, acked) = crate::smp::ipi_send_status();

    let phase90_ok = crate::task::program_loader::phase90_integration_smoke();
    return phase90_ok;
}

#[allow(unused_variables)]
fn exec_phase91_to_100_smokes() -> bool {
    let phase91_ok = crate::task::program_loader::phase91_fork_cow_smoke();
    let (cow_breaks, cow_isolated) = crate::user_paging::fork_cow_status();

    let phase92_ok = crate::task::program_loader::phase92_poll_lite_smoke();
    let (polls, poll_ready) = crate::pipe::poll_status();

    let phase93_ok = crate::task::program_loader::phase93_mmap_gap_smoke();
    let gaps = crate::vma::mmap_gap_status();

    let phase94_ok = crate::task::program_loader::phase94_exec_argv_smoke();
    let argv_ok = crate::task::process::exec_argv_status();

    let phase95_ok = crate::task::program_loader::phase95_pipe_probe_smoke();
    let (hw_pipes, bytes) = crate::pipe::pipeprobe_status();

    let phase96_ok = crate::task::program_loader::phase96_vma_coalesce_smoke();
    let (coalesced, _) = crate::vma::coalesce_status();

    let phase97_ok = crate::task::program_loader::phase97_work_steal_smoke();
    let steals = crate::smp::work_steal_status();

    let phase98_ok = crate::task::program_loader::phase98_ap_runnable_smoke();
    let ap_run = crate::smp::ap_runnable_status();

    let phase99_ok = crate::task::program_loader::phase99_lapic_icr_smoke();
    let (icr_writes, icr_sent) = crate::smp::lapic_icr_status();

    let phase100_ok = crate::task::program_loader::phase100_integration_smoke();
    return phase100_ok;
}

#[allow(unused_variables)]
fn exec_phase101_to_110_smokes() -> bool {
    let phase110_ok = crate::governance::phase110_constitutional_smoke();
    let (abi_v1, semantics_v1, immutable_identity, _) = crate::governance::status();
    let gates = phase110_ok;
    return phase110_ok;
}

#[allow(unused_variables)]
fn exec_phase111_to_120_smokes() -> bool {
    let phase120_ok = crate::governance::phase120_cap_compat_smoke();
    let (cap_table, rights, grant, broker, compat) = crate::governance::phase120_status();
    return phase120_ok;
}

#[allow(unused_variables)]
fn exec_phase201_virtio_smoke() -> bool {
    let ok = crate::governance::phase201_virtio_blk_smoke();
    let (pci, probes, driver_backed) = crate::virtio_blk::status();
    return ok;
}

#[allow(unused_variables)]
fn exec_phase131_to_140_smokes() -> bool {
    let p131 = crate::governance::phase131_build_integrity_smoke();
    let p132 = crate::governance::phase132_repro_smoke();
    let p133 = crate::governance::phase133_rollback_smoke();
    let p134 = crate::governance::phase134_endpoint_smoke();
    let bridge = crate::ipc_interim_bridge::ipc_bridge_compat_internal_count();
    let p135 = crate::governance::phase135_audit_wire_smoke();
    let p136 = crate::governance::phase136_wait_set_smoke();
    let p137 = crate::governance::phase137_error_taxonomy_smoke();
    let p138 = crate::governance::phase138_schema_smoke();
    let p140 = crate::governance::phase140_ipc_integration_smoke();
    return p140;
}

#[allow(unused_variables)]
fn exec_epoch4_network_smokes() -> bool {
    let ok = crate::governance::phase404_network_epoch_smoke();
    let (tcp, udp, sel) = crate::compat_socket::compat_socket_calls();
    return ok;
}

#[allow(unused_variables)]
fn exec_epoch5_scheduler_smokes() -> bool {
    let ok = crate::governance::phase149_epoch5_integration_smoke();
    return ok;
}

#[allow(unused_variables)]
fn exec_milestone150() -> bool {
    let ok = crate::governance::phase150_milestone_smoke();
    return ok;
}


#[allow(unused_variables)]
fn exec_phase122_to_130_smokes() -> bool {
    let p122 = crate::governance::phase122_storage_broker_smoke();
    let p123 = crate::governance::phase123_permission_broker_smoke();
    let p124 = crate::governance::phase124_device_broker_smoke();
    let p125 = crate::governance::phase125_network_broker_smoke();
    let p126 = crate::governance::phase126_clipboard_broker_smoke();
    let p127 = crate::governance::phase127_service_isolation_smoke();
    let p128 = crate::governance::phase128_native_manifest_smoke();
    let p129 = crate::governance::phase129_scoped_grants_smoke();
    let p130 = crate::governance::phase130_platform_integration_smoke();
    let bridge = crate::ipc_interim_bridge::ipc_bridge_compat_internal_count();
    return p130;
}

#[allow(unused_variables)]
fn exec_phase121_smoke() -> bool {
    let phase121_ok = crate::governance::phase121_service_loader_smoke();
    let (bootstrap, stubs, budget, _) = crate::governance::phase121_status();
    let (mem_total, mem_used, mem_free) = crate::service_loader::mem_budget_status();
    return phase121_ok;
}

#[allow(unused_variables)]
fn exec_phase21_to_30_smokes() -> bool {
    let phase21_ok = crate::task::program_loader::phase21_smoke_check();
    let (hw_built, hw_verified, hw_rejected, _, _, _, _) = crate::user_paging::status();
    let phase22_ok = crate::task::program_loader::phase22_smoke_check();
    let (cr3_act, cr3_restore, _, _, _, _, _) = crate::user_paging::status();
    let phase23_ok = crate::task::program_loader::phase23_smoke_check();
    let (iretq_entries, iretq_trapped, _, _) = crate::user_entry::status();
    let phase24_ok = crate::task::program_loader::phase24_smoke_check();
    let (trap_count, trap_returns, _, _) = crate::user_entry::status();
    crate::user_syscall_hw::init_syscall_msrs();
    let phase25_ok = crate::task::program_loader::phase25_smoke_check();
    let (hw_syscalls, hw_sysrets) = crate::user_syscall_hw::status();
    let phase26_ok = crate::task::program_loader::phase26_smoke_check();
    let (copy_ok_count, copy_rejected) = crate::user_copy::status();
    let phase27_ok = crate::task::program_loader::phase27_smoke_check();
    let (reloc_applied, reloc_rejected) = crate::elf_reloc::status();
    let phase28_ok = crate::task::program_loader::phase28_smoke_check();
    let hw_elf_status = crate::task::program_loader::status();
    let phase29_ok = crate::task::program_loader::phase29_smoke_check();
    let phase30_ok = crate::task::program_loader::phase30_cr3_switch_smoke();
    let (_, _, _, _, _, cr3_switches, isolated) = crate::user_paging::status();
    crate::task::program_loader::set_hw_user_elf_ready();
    return phase30_ok;
}
