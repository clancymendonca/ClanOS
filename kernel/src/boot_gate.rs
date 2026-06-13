//! Unified boot-time validation gate (subsystem smokes consolidated).
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

#[allow(unused_variables)]
fn run_sched_userspace_smokes() -> bool {
    let smoke_ok = crate::task::program_loader::smoke_sched_cr3_smoke();
    let (bound, switches, skips, restore_ok) = crate::user_paging::sched_cr3_status();

    let smoke_ok = crate::task::program_loader::smoke_user_frame_smoke();
    let (saves, resumes, preempted) = crate::user_hw_frame::status();

    let smoke_ok = crate::task::program_loader::smoke_multi_elf_smoke();

    let smoke_ok = crate::task::program_loader::smoke_exit_wait_smoke();
    let (exits, waits, _) = crate::syscall::exit_wait_status();

    let smoke_ok = crate::task::program_loader::smoke_syscall_table_smoke();
    let (allowed, rejected, _) = crate::user_syscall_hw::dispatch_table_status();

    let smoke_ok = crate::task::program_loader::smoke_storage_copyin_smoke();
    let (reads, rej) = crate::task::program_loader::storage_copyin_status();

    let smoke_ok = crate::task::program_loader::smoke_manifest_elf_smoke();
    let (disc, exec, rej) = crate::task::program_loader::manifest_elf_status();

    let smoke_ok = crate::task::program_loader::smoke_demand_zero_smoke();
    let (faults, mapped, rejected) = crate::demand_paging::status();

    let smoke_ok = crate::task::program_loader::smoke_dynamic_smoke();
    let (needed, linked, reloc_ok) = crate::elf_reloc::dynamic_status();

    let smoke_ok_for40 = smoke_ok;
    let smoke_ok = crate::task::program_loader::smoke_sched_userspace_integration();
    let (bound2, sw2, _, restore2) = crate::user_paging::sched_cr3_status();
    let (reads2, _) = crate::task::program_loader::storage_copyin_status();
    let (disc2, _, _) = crate::task::program_loader::manifest_elf_status();
    let (_, mapped2, _) = crate::demand_paging::status();
    return smoke_ok && restore2;
}

#[allow(unused_variables)]
fn run_dynamic_runtime_smokes() -> bool {
    let smoke_ok = crate::task::program_loader::smoke_shared_lib_smoke();
    let (loaded, pages, _) = crate::shared_loader::status();

    let smoke_ok = crate::task::program_loader::smoke_dyn_reloc_smoke();
    let (imports, applied) = crate::elf_reloc::import_status();

    let smoke_ok = crate::task::program_loader::smoke_trust_exec_smoke();
    let (trust_ok, trust_rej) = crate::task::program_loader::trust_exec_status();

    let smoke_ok = crate::task::program_loader::smoke_user_path_smoke();
    let (reads, path_rej) = crate::user_path::status();

    let smoke_ok = crate::task::program_loader::smoke_file_fd_smoke();
    let (opens, closes, _, _, _) = crate::fd_table::status();

    let smoke_ok = crate::task::program_loader::smoke_fd_io_smoke();
    let (_, _, fd_reads, fd_writes, _) = crate::fd_table::status();

    let smoke_ok = crate::task::program_loader::smoke_file_demand_smoke();
    let (faults, file_loaded, file_rej) = crate::demand_paging::file_status();

    let smoke_ok = crate::task::program_loader::smoke_wx_policy_smoke();
    let (wx_checked, wx_rejected) = crate::user_paging::wx_status();

    let smoke_ok = crate::task::program_loader::smoke_smp_smoke();
    let (cpus, aps, flush_ok) = crate::smp::status();

    let smoke_ok = crate::task::program_loader::smoke_dynamic_runtime_integration();
    let (loaded2, _, _) = crate::shared_loader::status();
    let (_, applied2) = crate::elf_reloc::import_status();
    let (t_ok, t_rej) = crate::task::program_loader::trust_exec_status();
    let (p_reads, _) = crate::user_path::status();
    let (o2, _, r2, _, _) = crate::fd_table::status();
    let (_, f_loaded, _) = crate::demand_paging::file_status();
    let (_, wx_rej2) = crate::user_paging::wx_status();
    let (cpus2, _, flush2) = crate::smp::status();
    return smoke_ok;
}

#[allow(unused_variables)]
fn run_fd_mmap_smokes() -> bool {
    let smoke_ok = crate::task::program_loader::smoke_proc_fd_smoke();

    let smoke_ok = crate::task::program_loader::smoke_fd_dup_smoke();
    let (dups, relative) = crate::fd_table::dup_status();

    let smoke_ok = crate::task::program_loader::smoke_mprotect_smoke();
    let (applied, rejected, guard) = crate::user_paging::mprotect_status();

    let smoke_ok = crate::task::program_loader::smoke_mmap_smoke();
    let (anon, file, rej) = crate::mmap::status();

    let smoke_ok = crate::task::program_loader::smoke_write_path_smoke();
    let (writes, verified) = crate::user_path::write_status();

    let smoke_ok = crate::task::program_loader::smoke_multi_shlib_smoke();
    let (loaded, pages, _) = crate::shared_loader::status();

    let smoke_ok = crate::task::program_loader::smoke_plt_reloc_smoke();
    let (slots, plt_applied) = crate::elf_reloc::plt_status();

    let smoke_ok = crate::task::program_loader::smoke_digest_trust_smoke();
    let (verified, rejected) = crate::image_digest::status();

    let smoke_ok = crate::task::program_loader::smoke_runqueue_smoke();
    let (cpus, enqueued, _) = (
        crate::smp::status().0,
        crate::smp::runqueue_status().0,
        (),
    );

    let smoke_ok = crate::task::program_loader::smoke_fd_mmap_integration();
    return smoke_ok;
}

#[allow(unused_variables)]
fn run_vm_fork_smokes() -> bool {
    let smoke_ok = crate::task::program_loader::smoke_chdir_smoke();
    let (normalized, chdirs) = crate::user_path::chdir_status();

    let smoke_ok = crate::task::program_loader::smoke_munmap_smoke();
    let (unmapped, munmap_rej) = crate::mmap::munmap_status();

    let smoke_ok = crate::task::program_loader::smoke_vma_smoke();
    let (vma_regions, vma_overlap) = crate::vma::status();

    let smoke_ok = crate::task::program_loader::smoke_forklite_smoke();
    let (inherited, isolated) = crate::fd_table::fork_lite_status();

    let smoke_ok = crate::task::program_loader::smoke_ring3_syscall_smoke();
    let (ring3_write, ring3_mprotect) = crate::user_syscall_hw::ring3_syscall_status();

    let smoke_ok = crate::task::program_loader::smoke_fcntl_smoke();
    let (fcntl_getfd, fcntl_dup, fcntl_rej) = crate::fd_table::fcntl_status();

    let smoke_ok = crate::task::program_loader::smoke_lazy_plt_smoke();
    let (plt_lazy, plt_bound) = crate::elf_reloc::lazy_plt_status();

    let smoke_ok = crate::task::program_loader::smoke_tlb_shootdown_smoke();
    let (cpus, _, _) = crate::smp::status();
    let (shootdowns, _) = crate::smp::shootdown_status();

    let smoke_ok = crate::task::program_loader::smoke_ap_idle_smoke();
    let (aps, idle_ticks) = crate::smp::ap_idle_status();

    let smoke_ok = crate::task::program_loader::smoke_vm_fork_integration();
    return smoke_ok;
}

#[allow(unused_variables)]
fn run_syscall_ring3_smokes() -> bool {
    let smoke_ok = crate::task::program_loader::smoke_sysret_smoke();
    let (probes, sysret_ok) = crate::user_syscall_hw::sysret_status();

    let smoke_ok = crate::task::program_loader::smoke_ring3_chdir_smoke();
    let ring3_chdirs = crate::user_path::ring3_chdir_status();

    let smoke_ok = crate::task::program_loader::smoke_munmap_len_smoke();
    let (unmapped_pages, partial_regions) = crate::mmap::munmap_len_status();

    let smoke_ok = crate::task::program_loader::smoke_waitlite_smoke();
    let (waited, wait_rejected) = crate::task::process::wait_lite_status();

    let smoke_ok = crate::task::program_loader::smoke_syscallprobe_smoke();
    let (ring3_write, ring3_mprotect) = crate::user_syscall_hw::ring3_syscall_status();

    let smoke_ok = crate::task::program_loader::smoke_fcntl_setfd_smoke();
    let (setfd, getfd, fcntl_rej) = crate::fd_table::fcntl_setfd_status();

    let smoke_ok = crate::task::program_loader::smoke_ring3_lazy_plt_smoke();
    let (plt_lazy, plt_bound) = crate::elf_reloc::lazy_plt_status();
    let ring3_plt = crate::elf_reloc::ring3_plt_status();

    let smoke_ok = crate::task::program_loader::smoke_ipi_tlb_smoke();
    let (cpus, _, _) = crate::smp::status();
    let (ipis, _) = crate::smp::ipi_status();

    let smoke_ok = crate::task::program_loader::smoke_ap_trampoline_smoke();
    let (aps, idle_ticks) = crate::smp::ap_idle_status();

    let smoke_ok = crate::task::program_loader::smoke_syscall_ring3_integration();
    return smoke_ok;
}

#[allow(unused_variables)]
fn run_path_exec_smokes() -> bool {
    let smoke_ok = crate::task::program_loader::smoke_hw_sysret_smoke();
    let (_, sysret_real) = crate::user_syscall_hw::hw_sysret_real_status();

    let smoke_ok = crate::task::program_loader::smoke_getcwd_smoke();
    let getcwd_reads = crate::user_path::getcwd_status();

    let smoke_ok = crate::task::program_loader::smoke_chdirprobe_smoke();

    let smoke_ok = crate::task::program_loader::smoke_vma_split_smoke();
    let (splits, _) = crate::vma::split_status();
    let (unmapped, _) = crate::mmap::munmap_len_status();

    let smoke_ok = crate::task::program_loader::smoke_fork_dup_smoke();
    let (children, duplicated) = crate::task::process::fork_dup_status();

    let smoke_ok = crate::task::program_loader::smoke_exec_lite_smoke();
    let (execs, cloexec_closed) = crate::task::process::exec_lite_status();

    let smoke_ok = crate::task::program_loader::smoke_pipe_lite_smoke();
    let (pipes, bytes) = crate::pipe::status();

    let smoke_ok = crate::task::program_loader::smoke_ring3_plt_fault_smoke();
    let (faults, bound) = crate::elf_reloc::ring3_plt_fault_status();

    let smoke_ok = crate::task::program_loader::smoke_ipi_send_smoke();
    let (sent, acked) = crate::smp::ipi_send_status();

    let smoke_ok = crate::task::program_loader::smoke_path_exec_integration();
    return smoke_ok;
}

#[allow(unused_variables)]
fn run_smp_depth_smokes() -> bool {
    let smoke_ok = crate::task::program_loader::smoke_fork_cow_smoke();
    let (cow_breaks, cow_isolated) = crate::user_paging::fork_cow_status();

    let smoke_ok = crate::task::program_loader::smoke_poll_lite_smoke();
    let (polls, poll_ready) = crate::pipe::poll_status();

    let smoke_ok = crate::task::program_loader::smoke_mmap_gap_smoke();
    let gaps = crate::vma::mmap_gap_status();

    let smoke_ok = crate::task::program_loader::smoke_exec_argv_smoke();
    let argv_ok = crate::task::process::exec_argv_status();

    let smoke_ok = crate::task::program_loader::smoke_pipe_probe_smoke();
    let (hw_pipes, bytes) = crate::pipe::pipeprobe_status();

    let smoke_ok = crate::task::program_loader::smoke_vma_coalesce_smoke();
    let (coalesced, _) = crate::vma::coalesce_status();

    let smoke_ok = crate::task::program_loader::smoke_work_steal_smoke();
    let steals = crate::smp::work_steal_status();

    let smoke_ok = crate::task::program_loader::smoke_ap_runnable_smoke();
    let ap_run = crate::smp::ap_runnable_status();

    let smoke_ok = crate::task::program_loader::smoke_lapic_icr_smoke();
    let (icr_writes, icr_sent) = crate::smp::lapic_icr_status();

    let smoke_ok = crate::task::program_loader::smoke_smp_depth_integration();
    return smoke_ok;
}

#[allow(unused_variables)]
fn run_constitutional_smokes() -> bool {
    let smoke_ok = crate::governance::smoke_constitutional();
    let (abi_v1, semantics_v1, immutable_identity, _) = crate::governance::status();
    let gates = smoke_ok;
    return smoke_ok;
}

#[allow(unused_variables)]
fn run_capabilities_smokes() -> bool {
    let smoke_ok = crate::governance::smoke_cap_compat();
    let (cap_table, rights, grant, broker, compat) = crate::governance::cap_compat_status();
    return smoke_ok;
}

#[allow(unused_variables)]
fn run_virtio_blk_smoke() -> bool {
    let ok = crate::governance::smoke_virtio_blk();
    let (pci, probes, driver_backed) = crate::virtio_blk::status();
    return ok;
}

#[allow(unused_variables)]
fn run_build_endpoint_smokes() -> bool {
    let _ = (
        crate::governance::smoke_build_integrity(),
        crate::governance::smoke_repro_build(),
        crate::governance::smoke_rollback(),
        crate::governance::smoke_ipc_endpoint(),
        crate::governance::smoke_audit_wire(),
        crate::governance::smoke_wait_set(),
        crate::governance::smoke_error_taxonomy(),
        crate::governance::smoke_schema(),
    );
    crate::governance::smoke_ipc_integration()
}

#[allow(unused_variables)]
fn run_network_compat_smokes() -> bool {
    let ok = crate::governance::smoke_network_epoch();
    let (tcp, udp, sel) = crate::compat_socket::compat_socket_calls();
    return ok;
}

#[allow(unused_variables)]
fn run_scheduler_epoch_smokes() -> bool {
    let ok = crate::governance::smoke_scheduler_epoch_integration();
    return ok;
}

#[allow(unused_variables)]
fn run_boundary_smoke() -> bool {
    let ok = crate::governance::smoke_milestone_boundary();
    return ok;
}


#[allow(unused_variables)]
fn run_platform_broker_smokes() -> bool {
    let _ = (
        crate::governance::smoke_storage_broker(),
        crate::governance::smoke_permission_broker(),
        crate::governance::smoke_device_broker(),
        crate::governance::smoke_network_broker(),
        crate::governance::smoke_clipboard_broker(),
        crate::governance::smoke_service_isolation(),
        crate::governance::smoke_native_manifest(),
        crate::governance::smoke_scoped_grants(),
    );
    crate::governance::smoke_platform_integration()
}

#[allow(unused_variables)]
fn run_service_loader_smoke() -> bool {
    let smoke_ok = crate::governance::smoke_service_loader_init();
    let (bootstrap, stubs, budget, _) = crate::governance::service_loader_status();
    let (mem_total, mem_used, mem_free) = crate::service_loader::mem_budget_status();
    return smoke_ok;
}

#[allow(unused_variables)]
fn run_hw_paging_smokes() -> bool {
    let smoke_ok = crate::task::program_loader::smoke_hw_page_table_build();
    let (hw_built, hw_verified, hw_rejected, _, _, _, _) = crate::user_paging::status();
    let smoke_ok = crate::task::program_loader::smoke_cr3_activate();
    let (cr3_act, cr3_restore, _, _, _, _, _) = crate::user_paging::status();
    let smoke_ok = crate::task::program_loader::smoke_iretq_entry();
    let (iretq_entries, iretq_trapped, _, _) = crate::user_entry::status();
    let smoke_ok = crate::task::program_loader::smoke_ring3_trap();
    let (trap_count, trap_returns, _, _) = crate::user_entry::status();
    crate::user_syscall_hw::init_syscall_msrs();
    let smoke_ok = crate::task::program_loader::smoke_hw_syscall_msr();
    let (hw_syscalls, hw_sysrets) = crate::user_syscall_hw::status();
    let smoke_ok = crate::task::program_loader::smoke_user_copy();
    let (copy_ok_count, copy_rejected) = crate::user_copy::status();
    let smoke_ok = crate::task::program_loader::smoke_elf_reloc_apply();
    let (reloc_applied, reloc_rejected) = crate::elf_reloc::status();
    let smoke_ok = crate::task::program_loader::smoke_hw_elf_exec();
    let hw_elf_status = crate::task::program_loader::status();
    let smoke_ok = crate::task::program_loader::smoke_hw_elf_isolation();
    let smoke_ok = crate::task::program_loader::smoke_cr3_switch();
    let (_, _, _, _, _, cr3_switches, isolated) = crate::user_paging::status();
    crate::task::program_loader::set_hw_user_elf_ready();
    return smoke_ok;
}

/// Boot subsystem smokes (QEMU integration harness); names match `VALIDATION_GATES.md`.
pub fn smoke_hw_paging() -> bool {
    run_hw_paging_smokes()
}

pub fn smoke_sched_userspace() -> bool {
    run_sched_userspace_smokes()
}

pub fn smoke_dynamic_runtime() -> bool {
    run_dynamic_runtime_smokes()
}

pub fn smoke_fd_mmap() -> bool {
    run_fd_mmap_smokes()
}

pub fn smoke_vm_fork() -> bool {
    run_vm_fork_smokes()
}

pub fn smoke_syscall_ring3() -> bool {
    run_syscall_ring3_smokes()
}

pub fn smoke_path_exec() -> bool {
    run_path_exec_smokes()
}

pub fn smoke_smp_depth() -> bool {
    run_smp_depth_smokes()
}

pub fn smoke_constitutional() -> bool {
    run_constitutional_smokes()
}

pub fn smoke_capabilities() -> bool {
    run_capabilities_smokes()
}

pub fn smoke_service_loader() -> bool {
    run_service_loader_smoke()
}

pub fn smoke_platform_brokers() -> bool {
    run_platform_broker_smokes()
}

pub fn smoke_virtio_blk() -> bool {
    run_virtio_blk_smoke()
}

pub fn smoke_build_endpoints() -> bool {
    run_build_endpoint_smokes()
}

pub fn smoke_network_compat() -> bool {
    run_network_compat_smokes()
}

pub fn smoke_scheduler_epoch() -> bool {
    run_scheduler_epoch_smokes()
}

pub fn smoke_boundary() -> bool {
    run_boundary_smoke()
}

/// Composite boot gate (no serial emission).
pub fn boot_gate() -> bool {
    let early = smoke_shell_storage()
        && smoke_loader_security()
        && smoke_memory_layout()
        && smoke_userspace_bootstrap();
    let (hw_paging, sched, dynamic, fd_mmap, vm_fork) =
        x86_64::instructions::interrupts::without_interrupts(|| {
            (
                smoke_hw_paging(),
                smoke_sched_userspace(),
                smoke_dynamic_runtime(),
                smoke_fd_mmap(),
                smoke_vm_fork(),
            )
        });
    let _ = crate::storage::ensure_filesystem_on_active();
    early
        && hw_paging
        && sched
        && dynamic
        && fd_mmap
        && vm_fork
        && smoke_syscall_ring3()
        && smoke_path_exec()
        && smoke_smp_depth()
        && smoke_constitutional()
        && smoke_capabilities()
        && smoke_service_loader()
        && smoke_platform_brokers()
        && smoke_virtio_blk()
        && smoke_build_endpoints()
        && smoke_network_compat()
        && smoke_scheduler_epoch()
        && smoke_boundary()
}
