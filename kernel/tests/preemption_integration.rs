//! QEMU integration harness: preemption core + unified boot/system gates.

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::{panic::PanicInfo, sync::atomic::Ordering};
use kernel::{
    allocator, hlt_loop, memory,
    performance::{metrics::TICK_COUNTER, process_metrics},
    syscall,
    task::{keyboard, process, scheduler},
};
use x86_64::VirtAddr;

entry_point!(main);

fn init_test_kernel(boot_info: &'static BootInfo) {
    kernel::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    kernel::user_paging::init(phys_mem_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    let heap_frames = frame_allocator.allocated_frame_count();

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialisation failed");
    let _ = kernel::frame_ownership::init_from_memory_map(
        &boot_info.memory_map,
        frame_allocator.allocated_frame_count(),
    );
    unsafe {
        kernel::user_paging::set_boot_frame_allocator(
            &boot_info.memory_map,
            heap_frames + kernel::frame_ownership::MAX_TRACKED_FRAMES,
        );
    }

    keyboard::init_scancode_queue();
    kernel::storage::init();
    kernel::mouse::init();
    kernel::userland_install::install_native_packages();
    let boot_tick = TICK_COUNTER.load(Ordering::Relaxed);
    let _ = process::create_kernel_process("shell", boot_tick);
}

fn main(boot_info: &'static BootInfo) -> ! {
    init_test_kernel(boot_info);
    test_main();
    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

#[test_case]
fn preemption_tick_requests_accumulate() {
    let before = scheduler::stats();
    for _ in 0..(scheduler::SCHED_QUANTUM_TICKS * 2) {
        scheduler::on_timer_tick();
    }
    let after = scheduler::stats();

    assert!(after.timer_ticks >= before.timer_ticks + scheduler::SCHED_QUANTUM_TICKS * 2);
    assert!(after.reschedule_requests >= before.reschedule_requests + 2);
}

#[test_case]
fn fairness_metrics_detect_imbalance() {
    let balanced = [
        (1u64, "p1", 1000u64),
        (2u64, "p2", 1005u64),
        (3u64, "p3", 1002u64),
        (4u64, "p4", 1001u64),
    ];
    let balanced_metrics = process_metrics::compute_fairness_metrics(&balanced);
    assert!(balanced_metrics.is_fair());

    let imbalanced = [
        (1u64, "p1", 5000u64),
        (2u64, "p2", 1000u64),
        (3u64, "p3", 1000u64),
        (4u64, "p4", 1000u64),
    ];
    let imbalanced_metrics = process_metrics::compute_fairness_metrics(&imbalanced);
    assert!(!imbalanced_metrics.is_fair());
    assert!(imbalanced_metrics.has_severe_starvation());
}

#[test_case]
fn process_registry_lifecycle() {
    let created_tick = TICK_COUNTER.load(Ordering::Relaxed);
    let before_count = process::process_count();

    let pid = process::create_kernel_process("preemption-proc", created_tick)
        .expect("process should be created");

    assert!(process::process_count() >= before_count + 1);

    assert!(process::set_process_state(
        pid,
        process::ProcessState::Ready
    ));
    let ready = process::get_ready_processes();
    assert!(ready.iter().any(|p| *p == pid));

    assert!(process::add_process_cpu_ticks(pid, 42));
    assert!(process::record_context_switch(pid));

    assert!(process::terminate_process(pid, 0));
    let reaped = process::reap_terminated_processes();
    assert!(reaped >= 1);
}

#[test_case]
fn syscall_invalid_paths_are_rejected() {
    assert_eq!(
        syscall::invoke_raw(999, 0),
        Err(syscall::SyscallError::InvalidSyscall)
    );
    assert_eq!(
        syscall::invoke_raw(syscall::SyscallId::GetTickCount as u64, 123),
        Err(syscall::SyscallError::InvalidArgument)
    );
}

#[test_case]
fn boot_gate_unified() {
    assert!(kernel::boot_gate::boot_gate());
}
