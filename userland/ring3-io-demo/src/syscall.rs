//! Clan OS compat syscalls for ring-3 I/O demo (`docs/ABI_SYSCALL.md`).

const SYS_EXIT_PROCESS: u64 = 61;
const SYS_OPEN_FILE: u64 = 66;
const SYS_CLOSE_FILE: u64 = 67;
const SYS_READ_FD: u64 = 68;
const SYS_WRITE_FD: u64 = 69;

#[inline(always)]
pub fn sys_open(path: *const u8) -> i64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_OPEN_FILE,
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
            in("rax") SYS_CLOSE_FILE,
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
            in("rax") SYS_READ_FD,
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
            in("rax") SYS_WRITE_FD,
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
            in("rax") SYS_EXIT_PROCESS,
            in("rdi") code,
            options(nostack, nomem)
        );
    }
    loop {
        core::hint::spin_loop();
    }
}
