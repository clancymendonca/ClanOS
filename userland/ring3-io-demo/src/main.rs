//! Ring-3 demo: open `/README.txt`, read via VFS, write to serial stdout.

#![no_std]
#![no_main]

mod syscall;

const README_PATH: &str = "/README.txt";
const EXPECTED: &[u8] = b"Clan OS persistent storage";

const READ_LEN: usize = 64;

static mut READ_BUF: [u8; READ_LEN] = [0u8; READ_LEN];

fn bytes_match_prefix(buf: &[u8], prefix: &[u8]) -> bool {
    if buf.len() < prefix.len() {
        return false;
    }
    buf[..prefix.len()] == *prefix
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let fd = syscall::sys_open(README_PATH.as_ptr());
    if fd < 0 {
        syscall::sys_exit(1);
    }

    let n = syscall::sys_read(
        fd as u64,
        unsafe { READ_BUF.as_mut_ptr() },
        READ_LEN,
    );
    if n <= 0 {
        syscall::sys_exit(2);
    }

    let slice = unsafe { &READ_BUF[..n as usize] };
    if !bytes_match_prefix(slice, EXPECTED) {
        syscall::sys_exit(3);
    }

    if syscall::sys_write(1, slice.as_ptr(), slice.len()) <= 0 {
        syscall::sys_exit(4);
    }
    let _ = syscall::sys_write(1, b"\n".as_ptr(), 1);
    let _ = syscall::sys_close(fd as u64);
    syscall::sys_exit(0);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    syscall::sys_exit(99);
}
