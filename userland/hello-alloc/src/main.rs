//! Ring-3 smoke: `alloc::Vec` + `String` on clan-rt bump heap.

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use clan_rt::ring3_syscall::{sys_exit, sys_write};

const EXPECTED_SUM: u32 = (64 * 63) / 2;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    clan_rt::heap::heap_reset();

    let mut values: Vec<u32> = Vec::new();
    for i in 0..64 {
        values.push(i);
    }
    let sum: u32 = values.iter().copied().sum();
    if sum != EXPECTED_SUM {
        sys_exit(1);
    }

    let mut msg = String::from("clan-rt heap ok\n");
    if clan_rt::heap::heap_used() == 0 {
        sys_exit(2);
    }
    if sys_write(1, msg.as_ptr(), msg.len()) <= 0 {
        sys_exit(3);
    }
    msg.clear();
    if !msg.is_empty() {
        sys_exit(4);
    }

    sys_exit(0);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    sys_exit(99);
}

#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    sys_exit(88);
}
