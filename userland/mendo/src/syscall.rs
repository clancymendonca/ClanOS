//! Clan OS compat syscalls (`docs/ABI_SYSCALL.md`).

const SYS_EXIT_PROCESS: u64 = 61;
const SYS_WRITE_FD: u64 = 69;

/// Write up to `len` bytes from `buf` to `fd` (typically stdout = 1).
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

/// Terminate the process with `code`.
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
