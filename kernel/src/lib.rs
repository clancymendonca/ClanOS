//! Clan OS kernel library.
//!
//! A hobby operating system written in Rust, exploring modern systems programming
//! and kernel development. Focuses on performance, modular design, and system
//! transparency while leveraging Rust's safety guarantees.

#![no_std]
#![no_main]
#![deny(warnings)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub mod address_space;
pub mod allocator;
pub mod audit_wire;
pub mod block;
pub mod boundary_gate;
pub mod build_integrity;
pub mod checkpoint;
pub mod clipboard_broker;
pub mod compat_socket;
pub mod compositor;
pub mod corpus_runner;
pub mod demand_paging;
pub mod desktop_runtime;
pub mod desktop_shell;
pub mod device;
pub mod device_broker;
pub mod driver_host;
pub mod elf_reloc;
pub mod embedded_hello_alloc;
pub mod embedded_mendo;
pub mod embedded_programs;
pub mod embedded_ring3_io_demo;
pub mod embedded_sig_demo;
pub mod exec_image;
pub mod ext2;
pub mod fd_table;
pub mod federation;
pub mod frame_backing;
pub mod frame_ownership;
pub mod framebuffer;
pub mod gdt;
pub mod governance;
pub mod image_digest;
pub mod interrupts;
pub mod ipc_endpoints;
pub mod ipc_interim_bridge;
pub mod kernel_object;
pub mod load_plan;
pub mod mapping_stub;
pub mod memory;
pub mod mmap;
pub mod mouse;
pub mod native_manifest;
pub mod native_syscall;
pub mod network_broker;
pub mod network_stack;
pub mod oom_policy;
pub mod path_broker;
pub mod performance;
pub mod permission_broker;
pub mod pipe;
pub mod posix_server;
pub mod validation_gate;
pub mod ring3_trampoline;
pub mod security;
pub mod semantic_observability;
pub mod serial;
pub mod service_isolation;
pub mod service_loader;
pub mod service_scheduler;
pub mod shared_loader;
pub mod signal;
pub mod smp;
pub mod storage;
pub mod storage_broker;
pub mod syscall;
pub mod task;
pub mod user_context;
pub mod user_copy;
pub mod user_entry;
pub mod user_hw_frame;
pub mod user_memory;
pub mod user_paging;
pub mod user_path;
pub mod user_syscall;
pub mod user_syscall_hw;
pub mod userland_install;
pub mod vfs;
pub mod vga_buffer;
pub mod virtio;
pub mod virtio_blk;
pub mod virtio_net;
pub mod vma;
pub mod window_manager;

use core::panic::PanicInfo;

/// Initialises all kernel subsystems.
///
/// Must be called exactly once, as early as possible in kernel startup.
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    smp::init();
}

// ────────────────────────────────── test harness ──────────────────────────────

/// Exit codes that are recognised by the QEMU `isa-debug-exit` device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self);
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

/// Halt the CPU until the next interrupt arrives (power-efficient idle loop).
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
use bootloader::{entry_point, BootInfo};
#[cfg(test)]
use x86_64::VirtAddr;

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    user_paging::init(phys_mem_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    let heap_frames = frame_allocator.allocated_frame_count();
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialisation failed");
    let _ = frame_ownership::init_from_memory_map(
        &boot_info.memory_map,
        frame_allocator.allocated_frame_count(),
    );
    unsafe {
        user_paging::set_boot_frame_allocator(
            &boot_info.memory_map,
            heap_frames + frame_ownership::MAX_TRACKED_FRAMES,
        );
    }
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
