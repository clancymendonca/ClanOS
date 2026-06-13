//! Kernel entry point.

#![no_std]
#![deny(warnings)]
// Warning resolution (scope-freeze commit):
// W1: unused assignment in user_entry::write_user_stub_hw_syscall_rdi — initial len=0 never read; both branches assign before use, refactored to if/else initializer.
// W2: unnecessary unsafe in user_paging::unmap_user_page — mapper.unmap/flush safe inside existing unsafe mapper_for_phys scope.
// W3: unnecessary unsafe in user_syscall_hw::init_syscall_msrs — &raw const SYSCALL_STACK does not require unsafe in current edition.
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

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("AresOS v{} booting...", env!("CARGO_PKG_VERSION"));

    kernel::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    kernel::user_paging::init(phys_mem_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    let heap_frames = frame_allocator.allocated_frame_count();

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
    kernel::mouse::init();
    kernel::userland_install::install_native_packages();
    let boot_tick =
        kernel::performance::metrics::TICK_COUNTER.load(core::sync::atomic::Ordering::Relaxed);
    let _ = kernel::task::process::create_kernel_process("shell", boot_tick);

    println!("Memory subsystem initialised.");

    kernel::boot_gate::run_boot_gate();
    kernel::system_gate::run_boot_gate();
    kernel::serial_println!("Boot: validation gates complete");
    kernel::desktop_runtime::boot_desktop();

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
        println!("Preemption mode active. Spawning 4 kernel tasks for fairness testing.");
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

    let mut executor = Executor::new();
    executor.spawn(Task::named("keyboard", keyboard::print_keypresses()));
    executor.spawn(Task::named(
        "desktop-refresh",
        kernel::desktop_runtime::refresh_loop(),
    ));
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
    kernel::serial_println!("AresOS shell ready — type here: help | run demo-hello | ls | desktop");
    kernel::serial_println!("(Use this terminal for commands; QEMU window shows the desktop.)");
    executor.run();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::serial_println!("KERNEL PANIC: {}", info);
    println!("{}", info);
    hlt_loop();
}
