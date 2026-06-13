//! Ring-3 `syscall`/`sysret` stubs (`docs/ABI_SYSCALL.md`).

#![allow(unsafe_code)]

use super::syscalls::{SYS_CLOSE, SYS_EXIT, SYS_OPEN, SYS_READ, SYS_WRITE};

#[inline(always)]
pub fn sys_open(path: *const u8) -> i64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_OPEN,
            in("rdi") path as u64,
            lateout("rax") ret,
            options(nostack)
        );
    }
    ret as i64
}

#[inline(always)]
pub fn sys_close(fd: u64) -> i64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_CLOSE,
            in("rdi") fd,
            lateout("rax") ret,
            options(nostack)
        );
    }
    ret as i64
}

#[inline(always)]
pub fn sys_read(fd: u64, buf: *mut u8, len: usize) -> i64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_READ,
            in("rdi") fd,
            in("rsi") buf as u64,
            in("rdx") len as u64,
            lateout("rax") ret,
            options(nostack)
        );
    }
    ret as i64
}

#[inline(always)]
pub fn sys_write(fd: u64, buf: *const u8, len: usize) -> i64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_WRITE,
            in("rdi") fd,
            in("rsi") buf as u64,
            in("rdx") len as u64,
            lateout("rax") ret,
            options(nostack)
        );
    }
    ret as i64
}

#[inline(always)]
pub fn sys_exit(code: u64) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_EXIT,
            in("rdi") code,
            options(nostack, nomem)
        );
    }
    loop {
        core::hint::spin_loop();
    }
}

#[inline(always)]
pub fn sys_gettick() -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") super::syscalls::SYS_GETTICK,
            in("rdi") 0u64,
            lateout("rax") ret,
            options(nostack)
        );
    }
    ret
}

#[inline(always)]
pub fn sys_kill(pid: u64, signo: u64) -> i64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") super::syscalls::SYS_KILL,
            in("rdi") pid,
            in("rsi") signo,
            lateout("rax") ret,
            options(nostack)
        );
    }
    ret as i64
}

#[inline(always)]
pub fn sys_sigaction(signo: u64, handler: u64) -> i64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") super::syscalls::SYS_SIGACTION,
            in("rdi") signo,
            in("rsi") handler,
            lateout("rax") ret,
            options(nostack)
        );
    }
    ret as i64
}

#[inline(never)]
pub fn sys_sigreturn() -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") super::syscalls::SYS_SIGRETURN,
            in("rdi") 0u64,
            options(nostack, nomem)
        );
    }
    loop {
        core::hint::spin_loop();
    }
}
