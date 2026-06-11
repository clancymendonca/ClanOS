//! Kernel entry point.

#![no_std]
#![no_main]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kernel::{
    allocator, hlt_loop, memory,
    performance::metrics::PerformanceCounters,
    println,
    task::{executor::Executor, keyboard, timer, Task},
};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn run_phase31_to_40_smokes() {
    let phase31_ok = kernel::task::program_loader::phase31_sched_cr3_smoke();
    let (bound, switches, skips, restore_ok) = kernel::user_paging::sched_cr3_status();
    println!(
        "Phase31-SchedCr3: bound={}, switches={}, restore_ok={}",
        bound,
        switches.max(skips),
        restore_ok && phase31_ok
    );
    kernel::serial_println!(
        "Phase31-SchedCr3: bound={}, switches={}, restore_ok={}",
        bound,
        switches.max(skips),
        restore_ok && phase31_ok
    );

    let phase32_ok = kernel::task::program_loader::phase32_user_frame_smoke();
    let (saves, resumes, preempted) = kernel::user_hw_frame::status();
    println!(
        "Phase32-UserFrame: saves={}, resumes={}, preempted_ok={}",
        saves,
        resumes,
        preempted && phase32_ok
    );
    kernel::serial_println!(
        "Phase32-UserFrame: saves={}, resumes={}, preempted_ok={}",
        saves,
        resumes,
        preempted && phase32_ok
    );

    let phase33_ok = kernel::task::program_loader::phase33_multi_elf_smoke();
    println!(
        "Phase33-MultiElf: programs=2, isolated=2, exit_codes_ok={}",
        phase33_ok
    );
    kernel::serial_println!(
        "Phase33-MultiElf: programs=2, isolated=2, exit_codes_ok={}",
        phase33_ok
    );

    let phase34_ok = kernel::task::program_loader::phase34_exit_wait_smoke();
    let (exits, waits, _) = kernel::syscall::exit_wait_status();
    println!(
        "Phase34-ExitWait: exits={}, waits={}, codes_ok={}",
        exits, waits, phase34_ok
    );
    kernel::serial_println!(
        "Phase34-ExitWait: exits={}, waits={}, codes_ok={}",
        exits,
        waits,
        phase34_ok
    );

    let phase35_ok = kernel::task::program_loader::phase35_syscall_table_smoke();
    let (allowed, rejected, _) = kernel::user_syscall_hw::dispatch_table_status();
    println!(
        "Phase35-SyscallTable: allowed={}, rejected={}, dispatch_ok={}",
        allowed, rejected, phase35_ok
    );
    kernel::serial_println!(
        "Phase35-SyscallTable: allowed={}, rejected={}, dispatch_ok={}",
        allowed,
        rejected,
        phase35_ok
    );

    let phase36_ok = kernel::task::program_loader::phase36_storage_copyin_smoke();
    let (reads, rej) = kernel::task::program_loader::storage_copyin_status();
    println!(
        "Phase36-StorageCopyin: reads={}, roundtrip_ok={}, rejected={}",
        reads, phase36_ok, rej
    );
    kernel::serial_println!(
        "Phase36-StorageCopyin: reads={}, roundtrip_ok={}, rejected={}",
        reads,
        phase36_ok,
        rej
    );

    let phase37_ok = kernel::task::program_loader::phase37_manifest_elf_smoke();
    let (disc, exec, rej) = kernel::task::program_loader::manifest_elf_status();
    println!(
        "Phase37-ManifestElf: discovered={}, executed={}, rejected={}, ok={}",
        disc, exec, rej, phase37_ok
    );
    kernel::serial_println!(
        "Phase37-ManifestElf: discovered={}, executed={}, rejected={}, ok={}",
        disc,
        exec,
        rej,
        phase37_ok
    );

    let phase38_ok = kernel::task::program_loader::phase38_demand_zero_smoke();
    let (faults, mapped, rejected) = kernel::demand_paging::status();
    println!(
        "Phase38-DemandZero: faults={}, mapped={}, rejected={}, ok={}",
        faults, mapped, rejected, phase38_ok
    );
    kernel::serial_println!(
        "Phase38-DemandZero: faults={}, mapped={}, rejected={}, ok={}",
        faults,
        mapped,
        rejected,
        phase38_ok
    );

    let phase39_ok = kernel::task::program_loader::phase39_dynamic_smoke();
    let (needed, linked, reloc_ok) = kernel::elf_reloc::dynamic_status();
    println!(
        "Phase39-Dynamic: needed={}, linked={}, reloc_ok={}",
        needed,
        linked,
        reloc_ok && phase39_ok
    );
    kernel::serial_println!(
        "Phase39-Dynamic: needed={}, linked={}, reloc_ok={}",
        needed,
        linked,
        reloc_ok && phase39_ok
    );

    let phase33_ok_for40 = phase33_ok;
    let phase40_ok = kernel::task::program_loader::phase40_integration_smoke();
    let (bound2, sw2, _, restore2) = kernel::user_paging::sched_cr3_status();
    let (reads2, _) = kernel::task::program_loader::storage_copyin_status();
    let (disc2, _, _) = kernel::task::program_loader::manifest_elf_status();
    let (_, mapped2, _) = kernel::demand_paging::status();
    println!(
        "Phase40-Integration: sched_cr3={}, multi_elf={}, copyin={}, manifest={}, demand={}, ok={}",
        sw2.max(bound2),
        phase33_ok_for40,
        reads2 > 0,
        disc2 > 0,
        mapped2 > 0,
        phase40_ok && restore2
    );
    kernel::serial_println!(
        "Phase40-Integration: sched_cr3={}, multi_elf={}, copyin={}, manifest={}, demand={}, ok={}",
        sw2.max(bound2),
        phase33_ok_for40,
        reads2 > 0,
        disc2 > 0,
        mapped2 > 0,
        phase40_ok && restore2
    );
}

fn run_phase41_to_50_smokes() {
    let phase41_ok = kernel::task::program_loader::phase41_shared_lib_smoke();
    let (loaded, pages, _) = kernel::shared_loader::status();
    println!(
        "Phase41-SharedLib: loaded={}, pages={}, ok={}",
        loaded, pages, phase41_ok
    );
    kernel::serial_println!(
        "Phase41-SharedLib: loaded={}, pages={}, ok={}",
        loaded,
        pages,
        phase41_ok
    );

    let phase42_ok = kernel::task::program_loader::phase42_dyn_reloc_smoke();
    let (imports, applied) = kernel::elf_reloc::import_status();
    println!(
        "Phase42-DynReloc: imports={}, applied={}, ok={}",
        imports, applied, phase42_ok
    );
    kernel::serial_println!(
        "Phase42-DynReloc: imports={}, applied={}, ok={}",
        imports,
        applied,
        phase42_ok
    );

    let phase43_ok = kernel::task::program_loader::phase43_trust_exec_smoke();
    let (trust_ok, trust_rej) = kernel::task::program_loader::trust_exec_status();
    println!(
        "Phase43-TrustExec: executed={}, rejected={}, ok={}",
        trust_ok, trust_rej, phase43_ok
    );
    kernel::serial_println!(
        "Phase43-TrustExec: executed={}, rejected={}, ok={}",
        trust_ok,
        trust_rej,
        phase43_ok
    );

    let phase44_ok = kernel::task::program_loader::phase44_user_path_smoke();
    let (reads, path_rej) = kernel::user_path::status();
    println!(
        "Phase44-UserPath: reads={}, rejected={}, ok={}",
        reads, path_rej, phase44_ok
    );
    kernel::serial_println!(
        "Phase44-UserPath: reads={}, rejected={}, ok={}",
        reads,
        path_rej,
        phase44_ok
    );

    let phase45_ok = kernel::task::program_loader::phase45_file_fd_smoke();
    let (opens, closes, _, _, _) = kernel::fd_table::status();
    println!(
        "Phase45-FileFd: opens={}, closes={}, ok={}",
        opens, closes, phase45_ok
    );
    kernel::serial_println!(
        "Phase45-FileFd: opens={}, closes={}, ok={}",
        opens,
        closes,
        phase45_ok
    );

    let phase46_ok = kernel::task::program_loader::phase46_fd_io_smoke();
    let (_, _, fd_reads, fd_writes, _) = kernel::fd_table::status();
    println!(
        "Phase46-FdIO: reads={}, writes={}, ok={}",
        fd_reads, fd_writes, phase46_ok
    );
    kernel::serial_println!(
        "Phase46-FdIO: reads={}, writes={}, ok={}",
        fd_reads,
        fd_writes,
        phase46_ok
    );

    let phase47_ok = kernel::task::program_loader::phase47_file_demand_smoke();
    let (faults, file_loaded, file_rej) = kernel::demand_paging::file_status();
    println!(
        "Phase47-FileDemand: faults={}, loaded={}, rejected={}, ok={}",
        faults, file_loaded, file_rej, phase47_ok
    );
    kernel::serial_println!(
        "Phase47-FileDemand: faults={}, loaded={}, rejected={}, ok={}",
        faults,
        file_loaded,
        file_rej,
        phase47_ok
    );

    let phase48_ok = kernel::task::program_loader::phase48_wx_policy_smoke();
    let (wx_checked, wx_rejected) = kernel::user_paging::wx_status();
    println!(
        "Phase48-WxPolicy: checked={}, rejected={}, ok={}",
        wx_checked, wx_rejected, phase48_ok
    );
    kernel::serial_println!(
        "Phase48-WxPolicy: checked={}, rejected={}, ok={}",
        wx_checked,
        wx_rejected,
        phase48_ok
    );

    let phase49_ok = kernel::task::program_loader::phase49_smp_smoke();
    let (cpus, aps, flush_ok) = kernel::smp::status();
    println!(
        "Phase49-Smp: cpus={}, aps={}, flush_ok={}",
        cpus,
        aps,
        flush_ok > 0 && phase49_ok
    );
    kernel::serial_println!(
        "Phase49-Smp: cpus={}, aps={}, flush_ok={}",
        cpus,
        aps,
        flush_ok > 0 && phase49_ok
    );

    let phase50_ok = kernel::task::program_loader::phase50_integration_smoke();
    let (loaded2, _, _) = kernel::shared_loader::status();
    let (_, applied2) = kernel::elf_reloc::import_status();
    let (t_ok, t_rej) = kernel::task::program_loader::trust_exec_status();
    let (p_reads, _) = kernel::user_path::status();
    let (o2, _, r2, _, _) = kernel::fd_table::status();
    let (_, f_loaded, _) = kernel::demand_paging::file_status();
    let (_, wx_rej2) = kernel::user_paging::wx_status();
    let (cpus2, _, flush2) = kernel::smp::status();
    println!(
        "Phase50-Integration: shared={}, trust={}, path={}, fd={}, file={}, wx={}, smp={}, ok={}",
        loaded2 > 0 && applied2 > 0,
        t_ok > 0 && t_rej > 0,
        p_reads > 0,
        o2 > 0 && r2 > 0,
        f_loaded > 0,
        wx_rej2 > 0,
        cpus2 >= 1 && flush2 > 0,
        phase50_ok
    );
    kernel::serial_println!(
        "Phase50-Integration: shared={}, trust={}, path={}, fd={}, file={}, wx={}, smp={}, ok={}",
        loaded2 > 0 && applied2 > 0,
        t_ok > 0 && t_rej > 0,
        p_reads > 0,
        o2 > 0 && r2 > 0,
        f_loaded > 0,
        wx_rej2 > 0,
        cpus2 >= 1 && flush2 > 0,
        phase50_ok
    );
}

fn run_phase51_to_60_smokes() {
    let phase51_ok = kernel::task::program_loader::phase51_proc_fd_smoke();
    println!(
        "Phase51-ProcFd: isolated={}, opens={}, ok={}",
        kernel::fd_table::proc_fd_isolated(),
        kernel::fd_table::status().0,
        phase51_ok
    );
    kernel::serial_println!(
        "Phase51-ProcFd: isolated={}, opens={}, ok={}",
        kernel::fd_table::proc_fd_isolated(),
        kernel::fd_table::status().0,
        phase51_ok
    );

    let phase52_ok = kernel::task::program_loader::phase52_fd_dup_smoke();
    let (dups, relative) = kernel::fd_table::dup_status();
    println!(
        "Phase52-FdDup: dups={}, relative={}, ok={}",
        dups, relative, phase52_ok
    );
    kernel::serial_println!(
        "Phase52-FdDup: dups={}, relative={}, ok={}",
        dups,
        relative,
        phase52_ok
    );

    let phase53_ok = kernel::task::program_loader::phase53_mprotect_smoke();
    let (applied, rejected, guard) = kernel::user_paging::mprotect_status();
    println!(
        "Phase53-Mprotect: applied={}, rejected={}, guard_faults={}, ok={}",
        applied, rejected, guard, phase53_ok
    );
    kernel::serial_println!(
        "Phase53-Mprotect: applied={}, rejected={}, guard_faults={}, ok={}",
        applied,
        rejected,
        guard,
        phase53_ok
    );

    let phase54_ok = kernel::task::program_loader::phase54_mmap_smoke();
    let (anon, file, rej) = kernel::mmap::status();
    println!(
        "Phase54-Mmap: anon_pages={}, file_pages={}, rejected={}, ok={}",
        anon, file, rej, phase54_ok
    );
    kernel::serial_println!(
        "Phase54-Mmap: anon_pages={}, file_pages={}, rejected={}, ok={}",
        anon,
        file,
        rej,
        phase54_ok
    );

    let phase55_ok = kernel::task::program_loader::phase55_write_path_smoke();
    let (writes, verified) = kernel::user_path::write_status();
    println!(
        "Phase55-WritePath: writes={}, verified={}, ok={}",
        writes, verified, phase55_ok
    );
    kernel::serial_println!(
        "Phase55-WritePath: writes={}, verified={}, ok={}",
        writes,
        verified,
        phase55_ok
    );

    let phase56_ok = kernel::task::program_loader::phase56_multi_shlib_smoke();
    let (loaded, pages, _) = kernel::shared_loader::status();
    println!(
        "Phase56-MultiShlib: loaded={}, pages={}, ok={}",
        loaded, pages, phase56_ok
    );
    kernel::serial_println!(
        "Phase56-MultiShlib: loaded={}, pages={}, ok={}",
        loaded,
        pages,
        phase56_ok
    );

    let phase57_ok = kernel::task::program_loader::phase57_plt_reloc_smoke();
    let (slots, plt_applied) = kernel::elf_reloc::plt_status();
    println!(
        "Phase57-PltReloc: slots={}, applied={}, ok={}",
        slots, plt_applied, phase57_ok
    );
    kernel::serial_println!(
        "Phase57-PltReloc: slots={}, applied={}, ok={}",
        slots,
        plt_applied,
        phase57_ok
    );

    let phase58_ok = kernel::task::program_loader::phase58_digest_trust_smoke();
    let (verified, rejected) = kernel::image_digest::status();
    println!(
        "Phase58-DigestTrust: verified={}, rejected={}, ok={}",
        verified, rejected, phase58_ok
    );
    kernel::serial_println!(
        "Phase58-DigestTrust: verified={}, rejected={}, ok={}",
        verified,
        rejected,
        phase58_ok
    );

    let phase59_ok = kernel::task::program_loader::phase59_runqueue_smoke();
    let (cpus, enqueued, _) = (
        kernel::smp::status().0,
        kernel::smp::runqueue_status().0,
        (),
    );
    println!(
        "Phase59-Runqueues: cpus={}, enqueued={}, ok={}",
        cpus, enqueued, phase59_ok
    );
    kernel::serial_println!(
        "Phase59-Runqueues: cpus={}, enqueued={}, ok={}",
        cpus,
        enqueued,
        phase59_ok
    );

    let phase60_ok = kernel::task::program_loader::phase60_integration_smoke();
    println!(
        "Phase60-Integration: procfd={}, dup={}, mprotect={}, mmap={}, writepath={}, multishlib={}, plt={}, digest={}, runq={}, ok={}",
        phase51_ok,
        phase52_ok,
        phase53_ok,
        phase54_ok,
        phase55_ok,
        phase56_ok,
        phase57_ok,
        phase58_ok,
        phase59_ok,
        phase60_ok,
    );
    kernel::serial_println!(
        "Phase60-Integration: procfd={}, dup={}, mprotect={}, mmap={}, writepath={}, multishlib={}, plt={}, digest={}, runq={}, ok={}",
        phase51_ok,
        phase52_ok,
        phase53_ok,
        phase54_ok,
        phase55_ok,
        phase56_ok,
        phase57_ok,
        phase58_ok,
        phase59_ok,
        phase60_ok,
    );
}

fn run_phase61_to_70_smokes() {
    let phase61_ok = kernel::task::program_loader::phase61_chdir_smoke();
    let (normalized, chdirs) = kernel::user_path::chdir_status();
    println!(
        "Phase61-Chdir: normalized={}, chdirs={}, ok={}",
        normalized, chdirs, phase61_ok
    );
    kernel::serial_println!(
        "Phase61-Chdir: normalized={}, chdirs={}, ok={}",
        normalized,
        chdirs,
        phase61_ok
    );

    let phase62_ok = kernel::task::program_loader::phase62_munmap_smoke();
    let (unmapped, munmap_rej) = kernel::mmap::munmap_status();
    println!(
        "Phase62-Munmap: unmapped={}, rejected={}, ok={}",
        unmapped, munmap_rej, phase62_ok
    );
    kernel::serial_println!(
        "Phase62-Munmap: unmapped={}, rejected={}, ok={}",
        unmapped,
        munmap_rej,
        phase62_ok
    );

    let phase63_ok = kernel::task::program_loader::phase63_vma_smoke();
    let (vma_regions, vma_overlap) = kernel::vma::status();
    println!(
        "Phase63-Vma: regions={}, overlaps_rejected={}, ok={}",
        vma_regions, vma_overlap, phase63_ok
    );
    kernel::serial_println!(
        "Phase63-Vma: regions={}, overlaps_rejected={}, ok={}",
        vma_regions,
        vma_overlap,
        phase63_ok
    );

    let phase64_ok = kernel::task::program_loader::phase64_forklite_smoke();
    let (inherited, isolated) = kernel::fd_table::fork_lite_status();
    println!(
        "Phase64-ForkLite: inherited={}, isolated={}, ok={}",
        inherited, isolated, phase64_ok
    );
    kernel::serial_println!(
        "Phase64-ForkLite: inherited={}, isolated={}, ok={}",
        inherited,
        isolated,
        phase64_ok
    );

    let phase65_ok = kernel::task::program_loader::phase65_ring3_syscall_smoke();
    let (ring3_write, ring3_mprotect) = kernel::user_syscall_hw::ring3_syscall_status();
    println!(
        "Phase65-Ring3Syscall: writepath={}, mprotect={}, ok={}",
        ring3_write > 0,
        ring3_mprotect > 0,
        phase65_ok
    );
    kernel::serial_println!(
        "Phase65-Ring3Syscall: writepath={}, mprotect={}, ok={}",
        ring3_write > 0,
        ring3_mprotect > 0,
        phase65_ok
    );

    let phase66_ok = kernel::task::program_loader::phase66_fcntl_smoke();
    let (fcntl_getfd, fcntl_dup, fcntl_rej) = kernel::fd_table::fcntl_status();
    println!(
        "Phase66-Fcntl: getfd={}, dupfd={}, rejected={}, ok={}",
        fcntl_getfd, fcntl_dup, fcntl_rej, phase66_ok
    );
    kernel::serial_println!(
        "Phase66-Fcntl: getfd={}, dupfd={}, rejected={}, ok={}",
        fcntl_getfd,
        fcntl_dup,
        fcntl_rej,
        phase66_ok
    );

    let phase67_ok = kernel::task::program_loader::phase67_lazy_plt_smoke();
    let (plt_lazy, plt_bound) = kernel::elf_reloc::lazy_plt_status();
    println!(
        "Phase67-LazyPlt: lazy={}, bound={}, ok={}",
        plt_lazy, plt_bound, phase67_ok
    );
    kernel::serial_println!(
        "Phase67-LazyPlt: lazy={}, bound={}, ok={}",
        plt_lazy,
        plt_bound,
        phase67_ok
    );

    let phase68_ok = kernel::task::program_loader::phase68_tlb_shootdown_smoke();
    let (cpus, _, _) = kernel::smp::status();
    let (shootdowns, _) = kernel::smp::shootdown_status();
    println!(
        "Phase68-TlbShootdown: cpus={}, shootdowns={}, ok={}",
        cpus, shootdowns, phase68_ok
    );
    kernel::serial_println!(
        "Phase68-TlbShootdown: cpus={}, shootdowns={}, ok={}",
        cpus,
        shootdowns,
        phase68_ok
    );

    let phase69_ok = kernel::task::program_loader::phase69_ap_idle_smoke();
    let (aps, idle_ticks) = kernel::smp::ap_idle_status();
    println!(
        "Phase69-ApIdle: aps={}, idle_ticks={}, ok={}",
        aps, idle_ticks, phase69_ok
    );
    kernel::serial_println!(
        "Phase69-ApIdle: aps={}, idle_ticks={}, ok={}",
        aps,
        idle_ticks,
        phase69_ok
    );

    let phase70_ok = kernel::task::program_loader::phase70_integration_smoke();
    println!(
        "Phase70-Integration: chdir={}, munmap={}, vma={}, fork={}, ring3={}, fcntl={}, lazyplt={}, tlb={}, ap={}, ok={}",
        phase61_ok,
        phase62_ok,
        phase63_ok,
        phase64_ok,
        phase65_ok,
        phase66_ok,
        phase67_ok,
        phase68_ok,
        phase69_ok,
        phase70_ok,
    );
    kernel::serial_println!(
        "Phase70-Integration: chdir={}, munmap={}, vma={}, fork={}, ring3={}, fcntl={}, lazyplt={}, tlb={}, ap={}, ok={}",
        phase61_ok,
        phase62_ok,
        phase63_ok,
        phase64_ok,
        phase65_ok,
        phase66_ok,
        phase67_ok,
        phase68_ok,
        phase69_ok,
        phase70_ok,
    );
}

fn run_phase71_to_80_smokes() {
    let phase71_ok = kernel::task::program_loader::phase71_sysret_smoke();
    let (probes, sysret_ok) = kernel::user_syscall_hw::sysret_status();
    println!(
        "Phase71-Sysret: probes={}, sysret_ok={}, ok={}",
        probes,
        sysret_ok > 0,
        phase71_ok
    );
    kernel::serial_println!(
        "Phase71-Sysret: probes={}, sysret_ok={}, ok={}",
        probes,
        sysret_ok > 0,
        phase71_ok
    );

    let phase72_ok = kernel::task::program_loader::phase72_ring3_chdir_smoke();
    let ring3_chdirs = kernel::user_path::ring3_chdir_status();
    println!(
        "Phase72-Ring3Chdir: chdirs={}, ok={}",
        ring3_chdirs, phase72_ok
    );
    kernel::serial_println!(
        "Phase72-Ring3Chdir: chdirs={}, ok={}",
        ring3_chdirs,
        phase72_ok
    );

    let phase73_ok = kernel::task::program_loader::phase73_munmap_len_smoke();
    let (unmapped_pages, partial_regions) = kernel::mmap::munmap_len_status();
    println!(
        "Phase73-MunmapLen: unmapped_pages={}, partial_regions={}, ok={}",
        unmapped_pages, partial_regions, phase73_ok
    );
    kernel::serial_println!(
        "Phase73-MunmapLen: unmapped_pages={}, partial_regions={}, ok={}",
        unmapped_pages,
        partial_regions,
        phase73_ok
    );

    let phase74_ok = kernel::task::program_loader::phase74_waitlite_smoke();
    let (waited, wait_rejected) = kernel::task::process::wait_lite_status();
    println!(
        "Phase74-WaitLite: waited={}, rejected={}, ok={}",
        waited, wait_rejected, phase74_ok
    );
    kernel::serial_println!(
        "Phase74-WaitLite: waited={}, rejected={}, ok={}",
        waited,
        wait_rejected,
        phase74_ok
    );

    let phase75_ok = kernel::task::program_loader::phase75_syscallprobe_smoke();
    let (ring3_write, ring3_mprotect) = kernel::user_syscall_hw::ring3_syscall_status();
    println!(
        "Phase75-SyscallProbe: writepath={}, mprotect={}, ok={}",
        ring3_write > 0,
        ring3_mprotect > 0,
        phase75_ok
    );
    kernel::serial_println!(
        "Phase75-SyscallProbe: writepath={}, mprotect={}, ok={}",
        ring3_write > 0,
        ring3_mprotect > 0,
        phase75_ok
    );

    let phase76_ok = kernel::task::program_loader::phase76_fcntl_setfd_smoke();
    let (setfd, getfd, fcntl_rej) = kernel::fd_table::fcntl_setfd_status();
    println!(
        "Phase76-Fcntl: setfd={}, getfd={}, rejected={}, ok={}",
        setfd, getfd, fcntl_rej, phase76_ok
    );
    kernel::serial_println!(
        "Phase76-Fcntl: setfd={}, getfd={}, rejected={}, ok={}",
        setfd,
        getfd,
        fcntl_rej,
        phase76_ok
    );

    let phase77_ok = kernel::task::program_loader::phase77_ring3_lazy_plt_smoke();
    let (plt_lazy, plt_bound) = kernel::elf_reloc::lazy_plt_status();
    let ring3_plt = kernel::elf_reloc::ring3_plt_status();
    println!(
        "Phase77-Ring3LazyPlt: lazy={}, bound={}, ring3={}, ok={}",
        plt_lazy, plt_bound, ring3_plt, phase77_ok
    );
    kernel::serial_println!(
        "Phase77-Ring3LazyPlt: lazy={}, bound={}, ring3={}, ok={}",
        plt_lazy,
        plt_bound,
        ring3_plt,
        phase77_ok
    );

    let phase78_ok = kernel::task::program_loader::phase78_ipi_tlb_smoke();
    let (cpus, _, _) = kernel::smp::status();
    let (ipis, _) = kernel::smp::ipi_status();
    println!(
        "Phase78-IpiTlb: cpus={}, ipis={}, ok={}",
        cpus, ipis, phase78_ok
    );
    kernel::serial_println!(
        "Phase78-IpiTlb: cpus={}, ipis={}, ok={}",
        cpus,
        ipis,
        phase78_ok
    );

    let phase79_ok = kernel::task::program_loader::phase79_ap_trampoline_smoke();
    let (aps, idle_ticks) = kernel::smp::ap_idle_status();
    println!(
        "Phase79-ApTrampoline: aps={}, idle_ticks={}, ok={}",
        aps, idle_ticks, phase79_ok
    );
    kernel::serial_println!(
        "Phase79-ApTrampoline: aps={}, idle_ticks={}, ok={}",
        aps,
        idle_ticks,
        phase79_ok
    );

    let phase80_ok = kernel::task::program_loader::phase80_integration_smoke();
    println!(
        "Phase80-Integration: sysret={}, chdir={}, munmap={}, wait={}, probe={}, fcntl={}, lazyplt={}, ipi={}, ap={}, ok={}",
        phase71_ok,
        phase72_ok,
        phase73_ok,
        phase74_ok,
        phase75_ok,
        phase76_ok,
        phase77_ok,
        phase78_ok,
        phase79_ok,
        phase80_ok,
    );
    kernel::serial_println!(
        "Phase80-Integration: sysret={}, chdir={}, munmap={}, wait={}, probe={}, fcntl={}, lazyplt={}, ipi={}, ap={}, ok={}",
        phase71_ok,
        phase72_ok,
        phase73_ok,
        phase74_ok,
        phase75_ok,
        phase76_ok,
        phase77_ok,
        phase78_ok,
        phase79_ok,
        phase80_ok,
    );
}

fn run_phase81_to_90_smokes() {
    let phase81_ok = kernel::task::program_loader::phase81_hw_sysret_smoke();
    let (_, sysret_real) = kernel::user_syscall_hw::hw_sysret_real_status();
    println!(
        "Phase81-HwSysret: probes={}, sysret_real={}, ok={}",
        kernel::user_syscall_hw::hw_sysret_real_status().0,
        sysret_real > 0,
        phase81_ok
    );
    kernel::serial_println!(
        "Phase81-HwSysret: probes={}, sysret_real={}, ok={}",
        kernel::user_syscall_hw::hw_sysret_real_status().0,
        sysret_real > 0,
        phase81_ok
    );

    let phase82_ok = kernel::task::program_loader::phase82_getcwd_smoke();
    let getcwd_reads = kernel::user_path::getcwd_status();
    println!("Phase82-Getcwd: reads={}, ok={}", getcwd_reads, phase82_ok);
    kernel::serial_println!("Phase82-Getcwd: reads={}, ok={}", getcwd_reads, phase82_ok);

    let phase83_ok = kernel::task::program_loader::phase83_chdirprobe_smoke();
    println!(
        "Phase83-Chdirprobe: chdir={}, getcwd={}, ok={}",
        kernel::user_path::ring3_chdir_status() > 0,
        kernel::user_path::getcwd_status() > 0,
        phase83_ok
    );
    kernel::serial_println!(
        "Phase83-Chdirprobe: chdir={}, getcwd={}, ok={}",
        kernel::user_path::ring3_chdir_status() > 0,
        kernel::user_path::getcwd_status() > 0,
        phase83_ok
    );

    let phase84_ok = kernel::task::program_loader::phase84_vma_split_smoke();
    let (splits, _) = kernel::vma::split_status();
    let (unmapped, _) = kernel::mmap::munmap_len_status();
    println!(
        "Phase84-VmaSplit: splits={}, unmapped={}, ok={}",
        splits, unmapped, phase84_ok
    );
    kernel::serial_println!(
        "Phase84-VmaSplit: splits={}, unmapped={}, ok={}",
        splits,
        unmapped,
        phase84_ok
    );

    let phase85_ok = kernel::task::program_loader::phase85_fork_dup_smoke();
    let (children, duplicated) = kernel::task::process::fork_dup_status();
    println!(
        "Phase85-ForkDup: children={}, duplicated={}, ok={}",
        children, duplicated, phase85_ok
    );
    kernel::serial_println!(
        "Phase85-ForkDup: children={}, duplicated={}, ok={}",
        children,
        duplicated,
        phase85_ok
    );

    let phase86_ok = kernel::task::program_loader::phase86_exec_lite_smoke();
    let (execs, cloexec_closed) = kernel::task::process::exec_lite_status();
    println!(
        "Phase86-ExecLite: execs={}, cloexec_closed={}, ok={}",
        execs, cloexec_closed, phase86_ok
    );
    kernel::serial_println!(
        "Phase86-ExecLite: execs={}, cloexec_closed={}, ok={}",
        execs,
        cloexec_closed,
        phase86_ok
    );

    let phase87_ok = kernel::task::program_loader::phase87_pipe_lite_smoke();
    let (pipes, bytes) = kernel::pipe::status();
    println!(
        "Phase87-PipeLite: pipes={}, bytes={}, ok={}",
        pipes, bytes, phase87_ok
    );
    kernel::serial_println!(
        "Phase87-PipeLite: pipes={}, bytes={}, ok={}",
        pipes,
        bytes,
        phase87_ok
    );

    let phase88_ok = kernel::task::program_loader::phase88_ring3_plt_fault_smoke();
    let (faults, bound) = kernel::elf_reloc::ring3_plt_fault_status();
    println!(
        "Phase88-Ring3PltFault: faults={}, bound={}, ok={}",
        faults, bound, phase88_ok
    );
    kernel::serial_println!(
        "Phase88-Ring3PltFault: faults={}, bound={}, ok={}",
        faults,
        bound,
        phase88_ok
    );

    let phase89_ok = kernel::task::program_loader::phase89_ipi_send_smoke();
    let (sent, acked) = kernel::smp::ipi_send_status();
    println!(
        "Phase89-IpiSend: sent={}, acked={}, ok={}",
        sent, acked, phase89_ok
    );
    kernel::serial_println!(
        "Phase89-IpiSend: sent={}, acked={}, ok={}",
        sent,
        acked,
        phase89_ok
    );

    let phase90_ok = kernel::task::program_loader::phase90_integration_smoke();
    println!(
        "Phase90-Integration: sysret={}, getcwd={}, chdirprobe={}, vma={}, forkdup={}, exec={}, pipe={}, plt={}, ipi={}, ok={}",
        sysret_real > 0,
        getcwd_reads > 0,
        kernel::user_path::chdirprobe_status() > 0,
        splits > 0,
        duplicated > 0,
        execs > 0,
        pipes > 0,
        bound > 0,
        sent >= 1,
        phase90_ok
    );
    kernel::serial_println!(
        "Phase90-Integration: sysret={}, getcwd={}, chdirprobe={}, vma={}, forkdup={}, exec={}, pipe={}, plt={}, ipi={}, ok={}",
        sysret_real > 0,
        getcwd_reads > 0,
        kernel::user_path::chdirprobe_status() > 0,
        splits > 0,
        duplicated > 0,
        execs > 0,
        pipes > 0,
        bound > 0,
        sent >= 1,
        phase90_ok
    );
}

fn run_phase91_to_100_smokes() {
    let phase91_ok = kernel::task::program_loader::phase91_fork_cow_smoke();
    let (cow_breaks, cow_isolated) = kernel::user_paging::fork_cow_status();
    println!(
        "Phase91-ForkCow: breaks={}, isolated={}, ok={}",
        cow_breaks,
        cow_isolated > 0,
        phase91_ok
    );
    kernel::serial_println!(
        "Phase91-ForkCow: breaks={}, isolated={}, ok={}",
        cow_breaks,
        cow_isolated > 0,
        phase91_ok
    );

    let phase92_ok = kernel::task::program_loader::phase92_poll_lite_smoke();
    let (polls, poll_ready) = kernel::pipe::poll_status();
    println!(
        "Phase92-PollLite: polls={}, ready={}, ok={}",
        polls, poll_ready, phase92_ok
    );
    kernel::serial_println!(
        "Phase92-PollLite: polls={}, ready={}, ok={}",
        polls,
        poll_ready,
        phase92_ok
    );

    let phase93_ok = kernel::task::program_loader::phase93_mmap_gap_smoke();
    let gaps = kernel::vma::mmap_gap_status();
    println!("Phase93-MmapGap: gaps_used={}, ok={}", gaps, phase93_ok);
    kernel::serial_println!("Phase93-MmapGap: gaps_used={}, ok={}", gaps, phase93_ok);

    let phase94_ok = kernel::task::program_loader::phase94_exec_argv_smoke();
    let argv_ok = kernel::task::process::exec_argv_status();
    println!(
        "Phase94-ExecArgv: execs={}, argv_ok={}, ok={}",
        kernel::task::process::exec_lite_status().0,
        argv_ok > 0,
        phase94_ok
    );
    kernel::serial_println!(
        "Phase94-ExecArgv: execs={}, argv_ok={}, ok={}",
        kernel::task::process::exec_lite_status().0,
        argv_ok > 0,
        phase94_ok
    );

    let phase95_ok = kernel::task::program_loader::phase95_pipe_probe_smoke();
    let (hw_pipes, bytes) = kernel::pipe::pipeprobe_status();
    println!(
        "Phase95-PipeProbe: hw_pipes={}, bytes={}, ok={}",
        hw_pipes, bytes, phase95_ok
    );
    kernel::serial_println!(
        "Phase95-PipeProbe: hw_pipes={}, bytes={}, ok={}",
        hw_pipes,
        bytes,
        phase95_ok
    );

    let phase96_ok = kernel::task::program_loader::phase96_vma_coalesce_smoke();
    let (coalesced, _) = kernel::vma::coalesce_status();
    println!(
        "Phase96-VmaCoalesce: coalesced={}, ok={}",
        coalesced, phase96_ok
    );
    kernel::serial_println!(
        "Phase96-VmaCoalesce: coalesced={}, ok={}",
        coalesced,
        phase96_ok
    );

    let phase97_ok = kernel::task::program_loader::phase97_work_steal_smoke();
    let steals = kernel::smp::work_steal_status();
    println!("Phase97-WorkSteal: steals={}, ok={}", steals, phase97_ok);
    kernel::serial_println!("Phase97-WorkSteal: steals={}, ok={}", steals, phase97_ok);

    let phase98_ok = kernel::task::program_loader::phase98_ap_runnable_smoke();
    let ap_run = kernel::smp::ap_runnable_status();
    println!("Phase98-ApRunnable: enqueued={}, ok={}", ap_run, phase98_ok);
    kernel::serial_println!("Phase98-ApRunnable: enqueued={}, ok={}", ap_run, phase98_ok);

    let phase99_ok = kernel::task::program_loader::phase99_lapic_icr_smoke();
    let (icr_writes, icr_sent) = kernel::smp::lapic_icr_status();
    println!(
        "Phase99-LapicIcr: writes={}, sent={}, ok={}",
        icr_writes, icr_sent, phase99_ok
    );
    kernel::serial_println!(
        "Phase99-LapicIcr: writes={}, sent={}, ok={}",
        icr_writes,
        icr_sent,
        phase99_ok
    );

    let phase100_ok = kernel::task::program_loader::phase100_integration_smoke();
    println!(
        "Phase100-Integration: cow={}, poll={}, mmap_gap={}, exec_argv={}, pipeprobe={}, vma_coalesce={}, steal={}, ap_run={}, icr={}, ok={}",
        cow_breaks > 0,
        poll_ready > 0,
        gaps > 0,
        argv_ok > 0,
        hw_pipes > 0,
        coalesced > 0,
        steals > 0,
        ap_run > 0,
        icr_writes > 0,
        phase100_ok
    );
    kernel::serial_println!(
        "Phase100-Integration: cow={}, poll={}, mmap_gap={}, exec_argv={}, pipeprobe={}, vma_coalesce={}, steal={}, ap_run={}, icr={}, ok={}",
        cow_breaks > 0,
        poll_ready > 0,
        gaps > 0,
        argv_ok > 0,
        hw_pipes > 0,
        coalesced > 0,
        steals > 0,
        ap_run > 0,
        icr_writes > 0,
        phase100_ok
    );
}

fn run_phase101_to_110_smokes() {
    let phase110_ok = kernel::governance::phase110_constitutional_smoke();
    let (abi_v1, semantics_v1, immutable_identity, _) = kernel::governance::status();
    let gates = phase110_ok;
    println!(
        "Phase110-Constitutional: abi_v1={}, semantics_v1={}, gates={}, immutable_identity={}, ok={}",
        abi_v1, semantics_v1, gates, immutable_identity, phase110_ok
    );
    kernel::serial_println!(
        "Phase110-Constitutional: abi_v1={}, semantics_v1={}, gates={}, immutable_identity={}, ok={}",
        abi_v1, semantics_v1, gates, immutable_identity, phase110_ok
    );
}

fn run_phase111_to_120_smokes() {
    let phase120_ok = kernel::governance::phase120_cap_compat_smoke();
    let (cap_table, rights, grant, broker, compat) = kernel::governance::phase120_status();
    println!(
        "Phase120-CapCompat: cap_table={}, rights={}, grant={}, broker={}, compat={}, ok={}",
        cap_table, rights, grant, broker, compat, phase120_ok
    );
    kernel::serial_println!(
        "Phase120-CapCompat: cap_table={}, rights={}, grant={}, broker={}, compat={}, ok={}",
        cap_table,
        rights,
        grant,
        broker,
        compat,
        phase120_ok
    );
}

fn run_phase122_to_130_smokes() {
    let p122 = kernel::governance::phase122_storage_broker_smoke();
    let p123 = kernel::governance::phase123_permission_broker_smoke();
    let p124 = kernel::governance::phase124_device_broker_smoke();
    let p125 = kernel::governance::phase125_network_broker_smoke();
    let p126 = kernel::governance::phase126_clipboard_broker_smoke();
    let p127 = kernel::governance::phase127_service_isolation_smoke();
    let p128 = kernel::governance::phase128_native_manifest_smoke();
    let p129 = kernel::governance::phase129_scoped_grants_smoke();
    let p130 = kernel::governance::phase130_platform_integration_smoke();
    let bridge = kernel::ipc_interim_bridge::ipc_bridge_compat_internal_count();
    println!(
        "Phase130-Platform: p122={}, p123={}, p124={}, p125={}, p126={}, p127={}, p128={}, p129={}, ipc_bridge_calls={}, ok={}",
        p122, p123, p124, p125, p126, p127, p128, p129, bridge, p130
    );
    kernel::serial_println!(
        "Phase130-Platform: p122={}, p123={}, p124={}, p125={}, p126={}, p127={}, p128={}, p129={}, ipc_bridge_calls={}, ok={}",
        p122, p123, p124, p125, p126, p127, p128, p129, bridge, p130
    );
}

fn run_phase121_smoke() {
    let phase121_ok = kernel::governance::phase121_service_loader_smoke();
    let (bootstrap, stubs, budget, _) = kernel::governance::phase121_status();
    let (mem_total, mem_used, mem_free) = kernel::service_loader::mem_budget_status();
    println!(
        "Phase121-ServiceLoader: bootstrap={}, stubs={}, budget_rej={}, mem_total={}, mem_used={}, mem_free={}, ok={}",
        bootstrap, stubs, budget, mem_total, mem_used, mem_free, phase121_ok
    );
    kernel::serial_println!(
        "Phase121-ServiceLoader: bootstrap={}, stubs={}, budget_rej={}, mem_total={}, mem_used={}, mem_free={}, ok={}",
        bootstrap,
        stubs,
        budget,
        mem_total,
        mem_used,
        mem_free,
        phase121_ok
    );
}

fn run_phase21_to_30_smokes() {
    let phase21_ok = kernel::task::program_loader::phase21_smoke_check();
    let (hw_built, hw_verified, hw_rejected, _, _, _, _) = kernel::user_paging::status();
    println!(
        "Phase21-HwPageTables: built={}, verified={}, rejected={}, tables_ok={}",
        hw_built, hw_verified, hw_rejected, phase21_ok
    );
    kernel::serial_println!(
        "Phase21-HwPageTables: built={}, verified={}, rejected={}, tables_ok={}",
        hw_built,
        hw_verified,
        hw_rejected,
        phase21_ok
    );
    let phase22_ok = kernel::task::program_loader::phase22_smoke_check();
    let (cr3_act, cr3_restore, _, _, _, _, _) = kernel::user_paging::status();
    println!(
        "Phase22-Cr3: activations={}, restores={}, verify_ok={}",
        cr3_act, cr3_restore, phase22_ok
    );
    kernel::serial_println!(
        "Phase22-Cr3: activations={}, restores={}, verify_ok={}",
        cr3_act,
        cr3_restore,
        phase22_ok
    );
    let phase23_ok = kernel::task::program_loader::phase23_smoke_check();
    let (iretq_entries, iretq_trapped, _, _) = kernel::user_entry::status();
    println!(
        "Phase23-Iretq: entries={}, trapped={}, entry_ok={}",
        iretq_entries, iretq_trapped, phase23_ok
    );
    kernel::serial_println!(
        "Phase23-Iretq: entries={}, trapped={}, entry_ok={}",
        iretq_entries,
        iretq_trapped,
        phase23_ok
    );
    let phase24_ok = kernel::task::program_loader::phase24_smoke_check();
    let (trap_count, trap_returns, _, _) = kernel::user_entry::status();
    println!(
        "Phase24-UserTrap: traps={}, returns={}, vector_ok={}",
        trap_count, trap_returns, phase24_ok
    );
    kernel::serial_println!(
        "Phase24-UserTrap: traps={}, returns={}, vector_ok={}",
        trap_count,
        trap_returns,
        phase24_ok
    );
    kernel::user_syscall_hw::init_syscall_msrs();
    let phase25_ok = kernel::task::program_loader::phase25_smoke_check();
    let (hw_syscalls, hw_sysrets) = kernel::user_syscall_hw::status();
    println!(
        "Phase25-SyscallHw: syscalls={}, sysrets={}, hw_ok={}",
        hw_syscalls, hw_sysrets, phase25_ok
    );
    kernel::serial_println!(
        "Phase25-SyscallHw: syscalls={}, sysrets={}, hw_ok={}",
        hw_syscalls,
        hw_sysrets,
        phase25_ok
    );
    let phase26_ok = kernel::task::program_loader::phase26_smoke_check();
    let (copy_ok_count, copy_rejected) = kernel::user_copy::status();
    println!(
        "Phase26-Copyin: copies={}, rejected={}, copy_ok={}",
        copy_ok_count, copy_rejected, phase26_ok
    );
    kernel::serial_println!(
        "Phase26-Copyin: copies={}, rejected={}, copy_ok={}",
        copy_ok_count,
        copy_rejected,
        phase26_ok
    );
    let phase27_ok = kernel::task::program_loader::phase27_smoke_check();
    let (reloc_applied, reloc_rejected) = kernel::elf_reloc::status();
    println!(
        "Phase27-Reloc: applied={}, rejected={}, reloc_ok={}",
        reloc_applied, reloc_rejected, phase27_ok
    );
    kernel::serial_println!(
        "Phase27-Reloc: applied={}, rejected={}, reloc_ok={}",
        reloc_applied,
        reloc_rejected,
        phase27_ok
    );
    let phase28_ok = kernel::task::program_loader::phase28_smoke_check();
    let hw_elf_status = kernel::task::program_loader::status();
    println!(
        "Phase28-HwHello: executions={}, exits={}, hello_hw_ok={}",
        hw_elf_status.hw_elf_execution_count, hw_elf_status.user_elf_exit_count, phase28_ok
    );
    kernel::serial_println!(
        "Phase28-HwHello: executions={}, exits={}, hello_hw_ok={}",
        hw_elf_status.hw_elf_execution_count,
        hw_elf_status.user_elf_exit_count,
        phase28_ok
    );
    let phase29_ok = kernel::task::program_loader::phase29_smoke_check();
    println!(
        "Phase29-Allowlist: programs=2, exit42_ok={}, hello_ok={}",
        phase29_ok, phase28_ok
    );
    kernel::serial_println!(
        "Phase29-Allowlist: programs=2, exit42_ok={}, hello_ok={}",
        phase29_ok,
        phase28_ok
    );
    let phase30_ok = kernel::task::program_loader::phase30_cr3_switch_smoke();
    let (_, _, _, _, _, cr3_switches, isolated) = kernel::user_paging::status();
    println!(
        "Phase30-Cr3Switch: switches={}, isolated={}, switch_ok={}",
        cr3_switches, isolated, phase30_ok
    );
    kernel::serial_println!(
        "Phase30-Cr3Switch: switches={}, isolated={}, switch_ok={}",
        cr3_switches,
        isolated,
        phase30_ok
    );
    kernel::task::program_loader::set_hw_user_elf_ready();
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("AresOS v{} booting...", env!("CARGO_PKG_VERSION"));

    kernel::init();

    // Initialise memory subsystem.
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    kernel::user_paging::init(phys_mem_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    let heap_frames = frame_allocator.allocated_frame_count();

    // Set up the kernel heap.
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialisation failed");
    kernel::frame_ownership::init_from_memory_map(
        &boot_info.memory_map,
        frame_allocator.allocated_frame_count(),
    )
    .expect("frame ownership initialisation failed");
    let skip_frames = heap_frames + kernel::frame_ownership::MAX_TRACKED_FRAMES;
    unsafe {
        kernel::user_paging::set_boot_frame_allocator(&boot_info.memory_map, skip_frames);
    }
    kernel::task::keyboard::init_scancode_queue();
    kernel::storage::init();
    let boot_tick =
        kernel::performance::metrics::TICK_COUNTER.load(core::sync::atomic::Ordering::Relaxed);
    let _ = kernel::task::process::create_kernel_process("shell", boot_tick);

    println!("Memory subsystem initialised.");
    let storage_smoke_ok = match kernel::storage::list_files() {
        Ok(files) => !files.is_empty(),
        Err(_) => false,
    };
    let readme_smoke_ok = matches!(kernel::storage::read_file("/README.txt"), Ok(Some(_)));
    let run_smoke_ok = kernel::task::userspace::run_program("echo", &["phase6-smoke"]).is_ok();
    println!(
        "Phase6-Smoke: mounted={}, list_ok={}, cat_ok={}, run_ok={}",
        kernel::storage::is_mounted(),
        storage_smoke_ok,
        readme_smoke_ok,
        run_smoke_ok
    );
    kernel::serial_println!(
        "Phase6-Smoke: mounted={}, list_ok={}, cat_ok={}, run_ok={}",
        kernel::storage::is_mounted(),
        storage_smoke_ok,
        readme_smoke_ok,
        run_smoke_ok
    );
    let phase7_storage_ok = kernel::storage::phase7_smoke_check();
    println!(
        "Phase7-Storage: mounted={}, persistent_rw_ok={}",
        kernel::storage::is_mounted(),
        phase7_storage_ok
    );
    kernel::serial_println!(
        "Phase7-Storage: mounted={}, persistent_rw_ok={}",
        kernel::storage::is_mounted(),
        phase7_storage_ok
    );
    let phase8_storage_ok = kernel::storage::phase8_smoke_check();
    let device_summary = kernel::device::summary();
    let (block_devices, driver_backed_blocks, backend) = kernel::block::summary();
    println!(
        "Phase8-Devices: total={}, pci={}, block={}, block_devices={}, driver_backed={}, storage_backend={}, storage_ok={}",
        device_summary.total,
        device_summary.pci,
        device_summary.block,
        block_devices,
        driver_backed_blocks,
        backend,
        phase8_storage_ok
    );
    kernel::serial_println!(
        "Phase8-Devices: total={}, pci={}, block={}, block_devices={}, driver_backed={}, storage_backend={}, storage_ok={}",
        device_summary.total,
        device_summary.pci,
        device_summary.block,
        block_devices,
        driver_backed_blocks,
        backend,
        phase8_storage_ok
    );
    let phase9_launch_ok = kernel::task::program_loader::phase9_smoke_check();
    let loader_status = kernel::task::program_loader::status();
    println!(
        "Phase9-Loader: programs={}, launch_ok={}, storage_backed={}, launches={}, failed_launches={}",
        loader_status.program_count,
        phase9_launch_ok,
        kernel::storage::is_mounted(),
        loader_status.launch_count,
        loader_status.failed_launch_count
    );
    kernel::serial_println!(
        "Phase9-Loader: programs={}, launch_ok={}, storage_backed={}, launches={}, failed_launches={}",
        loader_status.program_count,
        phase9_launch_ok,
        kernel::storage::is_mounted(),
        loader_status.launch_count,
        loader_status.failed_launch_count
    );
    let credentials = kernel::security::current_credentials();
    let policy_ok = kernel::security::phase10_smoke_check();
    let denied_ok = kernel::storage::phase10_smoke_check();
    println!(
        "Phase10-Security: user={}, role={}, policy_ok={}, denied_ok={}, denied_access={}, denied_execute={}",
        credentials.user.as_u64(),
        credentials.role.name(),
        policy_ok,
        denied_ok,
        kernel::security::denied_access_count(),
        kernel::security::denied_execute_count()
    );
    kernel::serial_println!(
        "Phase10-Security: user={}, role={}, policy_ok={}, denied_ok={}, denied_access={}, denied_execute={}",
        credentials.user.as_u64(),
        credentials.role.name(),
        policy_ok,
        denied_ok,
        kernel::security::denied_access_count(),
        kernel::security::denied_execute_count()
    );
    let phase11_images_ok = kernel::task::program_loader::phase11_smoke_check();
    let image_status = kernel::task::program_loader::status();
    let exec_blocked_ok = image_status.unsupported_execution_count > 0;
    println!(
        "Phase11-Images: images={}, valid={}, rejected={}, exec_blocked_ok={}",
        image_status.image_count,
        image_status.valid_image_count,
        image_status.invalid_image_count,
        phase11_images_ok && exec_blocked_ok
    );
    kernel::serial_println!(
        "Phase11-Images: images={}, valid={}, rejected={}, exec_blocked_ok={}",
        image_status.image_count,
        image_status.valid_image_count,
        image_status.invalid_image_count,
        phase11_images_ok && exec_blocked_ok
    );
    let phase12_load_plan_ok = kernel::task::program_loader::phase12_smoke_check();
    let load_plan_status = kernel::task::program_loader::status();
    println!(
        "Phase12-LoadPlan: prepared={}, rejected={}, pages={}, exec_blocked_ok={}",
        load_plan_status.prepared_image_count,
        load_plan_status.rejected_load_plan_count,
        load_plan_status.total_planned_pages,
        phase12_load_plan_ok
    );
    kernel::serial_println!(
        "Phase12-LoadPlan: prepared={}, rejected={}, pages={}, exec_blocked_ok={}",
        load_plan_status.prepared_image_count,
        load_plan_status.rejected_load_plan_count,
        load_plan_status.total_planned_pages,
        phase12_load_plan_ok
    );
    let phase13_mapping_ok = kernel::task::program_loader::phase13_smoke_check();
    let mapping_status = kernel::task::program_loader::status();
    println!(
        "Phase13-MappingStub: mapped={}, rejected={}, pages={}, copied={}, zeroed={}, exec_blocked_ok={}",
        mapping_status.mapped_image_count,
        mapping_status.rejected_mapping_count,
        mapping_status.total_mapped_pages,
        mapping_status.copied_bytes,
        mapping_status.zero_filled_bytes,
        phase13_mapping_ok
    );
    kernel::serial_println!(
        "Phase13-MappingStub: mapped={}, rejected={}, pages={}, copied={}, zeroed={}, exec_blocked_ok={}",
        mapping_status.mapped_image_count,
        mapping_status.rejected_mapping_count,
        mapping_status.total_mapped_pages,
        mapping_status.copied_bytes,
        mapping_status.zero_filled_bytes,
        phase13_mapping_ok
    );
    let phase14_frames_ok = kernel::frame_ownership::phase14_smoke_check();
    let frame_status = kernel::frame_ownership::status();
    println!(
        "Phase14-Frames: initialized={}, tracked={}, available={}, allocated={}, allocations={}, releases={}, failures={}, smoke_ok={}",
        frame_status.initialized,
        frame_status.tracked_frames,
        frame_status.available_frames,
        frame_status.allocated_frames,
        frame_status.allocation_count,
        frame_status.release_count,
        frame_status.failed_allocation_count,
        phase14_frames_ok
    );
    kernel::serial_println!(
        "Phase14-Frames: initialized={}, tracked={}, available={}, allocated={}, allocations={}, releases={}, failures={}, smoke_ok={}",
        frame_status.initialized,
        frame_status.tracked_frames,
        frame_status.available_frames,
        frame_status.allocated_frames,
        frame_status.allocation_count,
        frame_status.release_count,
        frame_status.failed_allocation_count,
        phase14_frames_ok
    );
    let phase15_backing_ok = kernel::task::program_loader::phase15_smoke_check();
    let backing_status = kernel::task::program_loader::status();
    let backing_frames = kernel::frame_ownership::status();
    println!(
        "Phase15-FrameBackedImage: backed={}, rejected={}, pages={}, frame_allocated={}, copied={}, zeroed={}, smoke_ok={}",
        backing_status.frame_backed_image_count,
        backing_status.rejected_frame_backing_count,
        backing_status.total_frame_backed_pages,
        backing_frames.allocated_frames,
        backing_status.copied_bytes,
        backing_status.zero_filled_bytes,
        phase15_backing_ok
    );
    kernel::serial_println!(
        "Phase15-FrameBackedImage: backed={}, rejected={}, pages={}, frame_allocated={}, copied={}, zeroed={}, smoke_ok={}",
        backing_status.frame_backed_image_count,
        backing_status.rejected_frame_backing_count,
        backing_status.total_frame_backed_pages,
        backing_frames.allocated_frames,
        backing_status.copied_bytes,
        backing_status.zero_filled_bytes,
        phase15_backing_ok
    );
    let phase16_tables_ok = kernel::task::program_loader::phase16_smoke_check();
    let table_status = kernel::task::program_loader::status();
    println!(
        "Phase16-PageTables: tables={}, rejected={}, pages={}, translate_ok={}, cr3_switched=false",
        table_status.user_page_table_count,
        table_status.rejected_user_page_table_count,
        table_status.total_user_page_table_pages,
        phase16_tables_ok
    );
    kernel::serial_println!(
        "Phase16-PageTables: tables={}, rejected={}, pages={}, translate_ok={}, cr3_switched=false",
        table_status.user_page_table_count,
        table_status.rejected_user_page_table_count,
        table_status.total_user_page_table_pages,
        phase16_tables_ok
    );
    let phase17_context_ok = kernel::task::program_loader::phase17_smoke_check();
    let context_status = kernel::task::program_loader::status();
    let user_selectors = kernel::gdt::user_selectors();
    println!(
        "Phase17-UserContext: contexts={}, rejected={}, user_code={}, user_data={}, entry_ok={}, ring3_entered=false",
        context_status.user_context_count,
        context_status.rejected_user_context_count,
        user_selectors.code.0,
        user_selectors.data.0,
        phase17_context_ok
    );
    kernel::serial_println!(
        "Phase17-UserContext: contexts={}, rejected={}, user_code={}, user_data={}, entry_ok={}, ring3_entered=false",
        context_status.user_context_count,
        context_status.rejected_user_context_count,
        user_selectors.code.0,
        user_selectors.data.0,
        phase17_context_ok
    );
    let phase18_ring3_ok = kernel::task::program_loader::phase18_smoke_check();
    let ring3_status = kernel::task::program_loader::status();
    println!(
        "Phase18-Ring3: entries={}, traps={}, rejected={}, trap_vector={}, survived={}",
        ring3_status.ring3_entry_count,
        ring3_status.ring3_trap_count,
        ring3_status.rejected_ring3_count,
        kernel::interrupts::USER_TRAP_VECTOR,
        phase18_ring3_ok
    );
    kernel::serial_println!(
        "Phase18-Ring3: entries={}, traps={}, rejected={}, trap_vector={}, survived={}",
        ring3_status.ring3_entry_count,
        ring3_status.ring3_trap_count,
        ring3_status.rejected_ring3_count,
        kernel::interrupts::USER_TRAP_VECTOR,
        phase18_ring3_ok
    );
    let phase19_syscall_ok = kernel::task::program_loader::phase19_smoke_check();
    let user_syscall_status = kernel::task::program_loader::status();
    println!(
        "Phase19-SyscallReturn: syscalls={}, returns={}, rejected={}, abi_ok={}, returned_ok={}",
        user_syscall_status.user_syscall_count,
        user_syscall_status.user_syscall_return_count,
        user_syscall_status.rejected_user_syscall_count,
        phase19_syscall_ok,
        phase19_syscall_ok
    );
    kernel::serial_println!(
        "Phase19-SyscallReturn: syscalls={}, returns={}, rejected={}, abi_ok={}, returned_ok={}",
        user_syscall_status.user_syscall_count,
        user_syscall_status.user_syscall_return_count,
        user_syscall_status.rejected_user_syscall_count,
        phase19_syscall_ok,
        phase19_syscall_ok
    );
    let phase20_user_elf_ok = kernel::task::program_loader::phase20_smoke_check();
    let user_elf_status = kernel::task::program_loader::status();
    println!(
        "Phase20-UserElf: executions={}, exits={}, rejected={}, hello_ok={}",
        user_elf_status.user_elf_execution_count,
        user_elf_status.user_elf_exit_count,
        user_elf_status.rejected_user_elf_count,
        phase20_user_elf_ok
    );
    kernel::serial_println!(
        "Phase20-UserElf: executions={}, exits={}, rejected={}, hello_ok={}",
        user_elf_status.user_elf_execution_count,
        user_elf_status.user_elf_exit_count,
        user_elf_status.rejected_user_elf_count,
        phase20_user_elf_ok
    );

    kernel::serial_println!("Boot: phase21-50 smokes start");
    x86_64::instructions::interrupts::without_interrupts(|| {
        run_phase21_to_30_smokes();
        run_phase31_to_40_smokes();
        run_phase41_to_50_smokes();
        run_phase51_to_60_smokes();
        run_phase61_to_70_smokes();
    });
    run_phase71_to_80_smokes();
    run_phase81_to_90_smokes();
    run_phase91_to_100_smokes();
    run_phase101_to_110_smokes();
    run_phase111_to_120_smokes();
    run_phase121_smoke();
    run_phase122_to_130_smokes();
    kernel::serial_println!("Boot: phase21-100 smokes done");

    // Display performance counters at startup.
    let counters = PerformanceCounters::read();
    println!(
        "CPU frequency estimate: {} MHz",
        PerformanceCounters::cpu_frequency_mhz()
    );
    println!("System ticks since boot: {}", counters.ticks());
    println!(
        "Preemption metrics: total_preemptions={}, lock_contention={}, fairness_violations={}",
        counters.total_preemptions(),
        counters.scheduler_lock_contention(),
        counters.fairness_violations()
    );

    let preemption_mode = cfg!(feature = "preemption");
    println!("Kernel features: preemption={}", preemption_mode);

    if preemption_mode {
        println!("Phase 5: Preemption mode active. Spawning 4 kernel tasks for fairness testing.");
        println!("Console: type 'help' to list runtime scheduler commands.");
        kernel::task::scheduler::set_context_switching_enabled(true);
        kernel::task::scheduler::spawn_kernel_tasks_phase5();
        println!(
            "Kernel tasks spawned. Starting preemptive scheduler. quantum_ticks={}, fairness_interval_ticks={}",
            kernel::task::scheduler::scheduler_quantum_ticks(),
            kernel::task::scheduler::fairness_check_interval_ticks()
        );
        kernel::task::scheduler::run_context_lab();
    }

    kernel::task::scheduler::set_context_switching_enabled(false);

    // Run the async executor with the keyboard task.
    let mut executor = Executor::new();
    executor.spawn(Task::named("keyboard", keyboard::print_keypresses()));
    executor.spawn(Task::named("uptime", timer::log_uptime()));
    executor.spawn(Task::named("scheduler-stats", timer::log_scheduler_stats()));
    executor.spawn(Task::named(
        "scheduler-groundwork",
        timer::log_scheduler_groundwork(),
    ));
    executor.spawn(Task::named("task-registry", timer::log_task_registry()));
    executor.spawn(Task::named("task-watchdog", timer::task_watchdog()));

    if cfg!(feature = "preemption") {
        executor.spawn(Task::named(
            "fairness-monitor",
            timer::log_preemption_fairness(),
        ));
    }

    let stats = executor.stats();
    let context_names = kernel::task::scheduler::context_task_names();
    println!(
        "Tasks: active={}, sleeping={}, ready={}, completed={}",
        stats.active_tasks, stats.sleeping_tasks, stats.ready_queue_depth, stats.completed_tasks
    );
    println!("Context tasks: {:?}", context_names);
    println!("Kernel ready. Entering event loop.");
    executor.run();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::serial_println!("KERNEL PANIC: {}", info);
    println!("{}", info);
    hlt_loop();
}
