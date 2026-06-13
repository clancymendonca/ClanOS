//! Ring-3 smoke: register SIGUSR1 handler, self-kill, delivery on syscall return, SigReturn.

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use clan_rt::ring3_syscall::{
    sys_exit, sys_gettick, sys_kill, sys_sigaction, sys_sigreturn, sys_write,
};

const SIGUSR1: u64 = 10;
const SIG_DFL: u64 = 0;

static HANDLED: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
#[inline(never)]
extern "C" fn sigusr1_handler() -> ! {
    HANDLED.store(1, Ordering::Relaxed);
    sys_sigreturn();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let handler = sigusr1_handler as u64;
    if sys_sigaction(SIGUSR1, handler) != SIG_DFL as i64 {
        sys_exit(1);
    }
    if sys_kill(0, SIGUSR1) != 0 {
        sys_exit(2);
    }
    let _tick = sys_gettick();
    if HANDLED.load(Ordering::Relaxed) != 1 {
        sys_exit(3);
    }
    let msg = b"sig ok\n";
    if sys_write(1, msg.as_ptr(), msg.len()) <= 0 {
        sys_exit(4);
    }
    sys_exit(0);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    sys_exit(99);
}
