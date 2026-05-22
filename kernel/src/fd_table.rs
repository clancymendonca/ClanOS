//! Per-bring-up file descriptor table (Phases 45–46).

use alloc::string::String;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

pub const MAX_FDS: usize = 8;

static FD_OPENS: AtomicU64 = AtomicU64::new(0);
static FD_CLOSES: AtomicU64 = AtomicU64::new(0);
static FD_READS: AtomicU64 = AtomicU64::new(0);
static FD_WRITES: AtomicU64 = AtomicU64::new(0);
static FD_REJECTED: AtomicU64 = AtomicU64::new(0);

struct FdSlot {
    path: String,
}

static FD_TABLE: Mutex<[Option<FdSlot>; MAX_FDS]> = Mutex::new([const { None }; MAX_FDS]);

pub fn status() -> (u64, u64, u64, u64, u64) {
    (
        FD_OPENS.load(Ordering::Relaxed),
        FD_CLOSES.load(Ordering::Relaxed),
        FD_READS.load(Ordering::Relaxed),
        FD_WRITES.load(Ordering::Relaxed),
        FD_REJECTED.load(Ordering::Relaxed),
    )
}

fn with_table<F, R>(f: F) -> R
where
    F: FnOnce(&mut [Option<FdSlot>; MAX_FDS]) -> R,
{
    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut table = FD_TABLE.lock();
        f(&mut *table)
    })
}

pub fn open_file(path: &str) -> Result<u32, ()> {
    if !crate::user_path::validate_user_path(path) {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    let creds = crate::security::current_credentials();
    if crate::storage::read_file_checked(creds, path).is_err() {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    with_table(|table| {
        for (idx, slot) in table.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(FdSlot {
                    path: String::from(path),
                });
                FD_OPENS.fetch_add(1, Ordering::Relaxed);
                return Ok(idx as u32);
            }
        }
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        Err(())
    })
}

pub fn close_file(fd: u32) -> Result<(), ()> {
    let idx = fd as usize;
    with_table(|table| {
        if idx >= MAX_FDS || table[idx].is_none() {
            FD_REJECTED.fetch_add(1, Ordering::Relaxed);
            return Err(());
        }
        table[idx] = None;
        FD_CLOSES.fetch_add(1, Ordering::Relaxed);
        Ok(())
    })
}

pub fn read_fd(fd: u32, user_buf: u64, max_len: u64) -> Result<u64, ()> {
    let len = core::cmp::min(max_len, 64) as usize;
    if len == 0 || user_buf == 0 {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    let path = with_table(|table| {
        let idx = fd as usize;
        table
            .get(idx)
            .and_then(|slot| slot.as_ref())
            .map(|slot| slot.path.clone())
    })
    .ok_or(())?;
    let creds = crate::security::current_credentials();
    let contents = crate::storage::read_file_checked(creds, &path)
        .map_err(|_| {
            FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        })?
        .ok_or_else(|| {
            FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        })?;
    let sample: alloc::vec::Vec<u8> = contents
        .as_bytes()
        .iter()
        .take(len)
        .copied()
        .collect();
    crate::user_copy::copy_to_user(&sample, user_buf).map_err(|_| {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    FD_READS.fetch_add(1, Ordering::Relaxed);
    Ok(sample.len() as u64)
}

pub fn write_fd(fd: u32, user_buf: u64, max_len: u64) -> Result<u64, ()> {
    let len = core::cmp::min(max_len, 32) as usize;
    if len == 0 || user_buf == 0 {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    let path = with_table(|table| {
        let idx = fd as usize;
        table
            .get(idx)
            .and_then(|slot| slot.as_ref())
            .map(|slot| slot.path.clone())
    })
    .ok_or(())?;
    if !path.starts_with("/tmp/") {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    let mut buf = [0u8; 32];
    crate::user_copy::copy_from_user(user_buf, &mut buf[..len]).map_err(|_| {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    let payload = core::str::from_utf8(&buf[..len]).unwrap_or("smoke");
    let creds = crate::security::current_credentials();
    crate::storage::write_file_checked(creds, &path, payload).map_err(|_| {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    FD_WRITES.fetch_add(1, Ordering::Relaxed);
    Ok(len as u64)
}

pub fn open_file_user_path(user_path_ptr: u64) -> Result<u32, ()> {
    let path = crate::user_path::copy_path_from_user(user_path_ptr)?;
    open_file(&path)
}

pub fn phase45_smoke() -> bool {
    let Some(fd) = open_file("/bin/hello").ok() else {
        return false;
    };
    let closed = close_file(fd).is_ok();
    let (opens, closes, _, _, _) = status();
    closed && opens > 0 && closes > 0
}

pub fn phase46_smoke() -> bool {
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(
        crate::security::Credentials::shell_user(),
        "hello",
    )
    .ok() else {
        return false;
    };
    let Some(fd) = open_file("/bin/hello").ok() else {
        return false;
    };
    let user_buf = crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(128);
    let before = FD_READS.load(Ordering::Relaxed);
    let ok = crate::user_paging::with_user_page_table(&built.hw, || read_fd(fd, user_buf, 16).ok())
        .ok()
        .flatten()
        .is_some();
    let _ = close_file(fd);
    ok && FD_READS.load(Ordering::Relaxed) > before
}
