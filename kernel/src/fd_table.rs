//! Per-process file descriptor tables (Phases 45–46, 51–52).

use alloc::{format, string::String};
use core::sync::atomic::{AtomicU64, Ordering};

use crate::task::process::{self, ProcessId};

pub const MAX_FDS: usize = 8;

static FD_OPENS: AtomicU64 = AtomicU64::new(0);
static FD_CLOSES: AtomicU64 = AtomicU64::new(0);
static FD_READS: AtomicU64 = AtomicU64::new(0);
static FD_WRITES: AtomicU64 = AtomicU64::new(0);
static FD_REJECTED: AtomicU64 = AtomicU64::new(0);
static FD_DUPS: AtomicU64 = AtomicU64::new(0);
static FD_RELATIVE: AtomicU64 = AtomicU64::new(0);
static PROC_FD_ISOLATED: AtomicU64 = AtomicU64::new(0);
static FCNTL_GETFD: AtomicU64 = AtomicU64::new(0);
static FCNTL_SETFD: AtomicU64 = AtomicU64::new(0);
static FCNTL_DUPFD: AtomicU64 = AtomicU64::new(0);
static FCNTL_REJECTED: AtomicU64 = AtomicU64::new(0);
static FORK_INHERITED: AtomicU64 = AtomicU64::new(0);
static FORK_ISOLATED: AtomicU64 = AtomicU64::new(0);

pub const F_GETFD: u64 = 1;
pub const F_DUPFD: u64 = 2;
pub const F_SETFD: u64 = 3;
pub const FD_CLOEXEC: u64 = 1;

#[derive(Clone, Debug)]
pub struct FdSlotStorage {
    pub path: String,
    pub flags: u32,
}

pub fn status() -> (u64, u64, u64, u64, u64) {
    (
        FD_OPENS.load(Ordering::Relaxed),
        FD_CLOSES.load(Ordering::Relaxed),
        FD_READS.load(Ordering::Relaxed),
        FD_WRITES.load(Ordering::Relaxed),
        FD_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn dup_status() -> (u64, u64) {
    (
        FD_DUPS.load(Ordering::Relaxed),
        FD_RELATIVE.load(Ordering::Relaxed),
    )
}

pub fn proc_fd_isolated() -> bool {
    PROC_FD_ISOLATED.load(Ordering::Relaxed) != 0
}

fn resolve_path_for_process(pid: ProcessId, path: &str) -> Result<String, ()> {
    let absolute = if path.starts_with('/') {
        String::from(path)
    } else {
        let cwd = process::process_cwd(pid).ok_or(())?;
        FD_RELATIVE.fetch_add(1, Ordering::Relaxed);
        if cwd.ends_with('/') {
            format!("{cwd}{path}")
        } else {
            format!("{cwd}/{path}")
        }
    };
    let normalized = crate::user_path::normalize_absolute_path(&absolute)?;
    if !crate::user_path::validate_user_path(&normalized) {
        return Err(());
    }
    Ok(normalized)
}

pub fn open_pipe_for_process(pid: ProcessId, path: &str) -> Result<u32, ()> {
    if !path.starts_with("/@pipe/") {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    match process::with_process_mut(pid, |process| {
        for (idx, slot) in process.fds_mut().iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(FdSlotStorage {
                    path: String::from(path),
                    flags: 0,
                });
                FD_OPENS.fetch_add(1, Ordering::Relaxed);
                return Ok(idx as u32);
            }
        }
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        Err(())
    }) {
        Some(Ok(fd)) => Ok(fd),
        _ => Err(()),
    }
}

pub fn open_file_for_process(pid: ProcessId, path: &str) -> Result<u32, ()> {
    let path = resolve_path_for_process(pid, path).map_err(|_| {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    let creds = process::process_owner(pid).unwrap_or(crate::security::current_credentials());
    if crate::storage::read_file_checked(creds, &path).is_err() {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    match process::with_process_mut(pid, |process| {
        for (idx, slot) in process.fds_mut().iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(FdSlotStorage { path, flags: 0 });
                FD_OPENS.fetch_add(1, Ordering::Relaxed);
                return Ok(idx as u32);
            }
        }
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        Err(())
    }) {
        Some(Ok(fd)) => Ok(fd),
        _ => Err(()),
    }
}

pub fn dup_fd_for_process(pid: ProcessId, fd: u32) -> Result<u32, ()> {
    let idx = fd as usize;
    match process::with_process_mut(pid, |process| {
        let table = process.fds_mut();
        if idx >= MAX_FDS {
            FD_REJECTED.fetch_add(1, Ordering::Relaxed);
            return Err(());
        }
        let path = table[idx].as_ref().map(|slot| slot.path.clone()).ok_or(())?;
        for (new_idx, slot) in table.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(FdSlotStorage { path, flags: 0 });
                FD_DUPS.fetch_add(1, Ordering::Relaxed);
                return Ok(new_idx as u32);
            }
        }
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        Err(())
    }) {
        Some(Ok(fd)) => Ok(fd),
        _ => Err(()),
    }
}

pub fn open_file(path: &str) -> Result<u32, ()> {
    let pid = process::current_process_id()
        .or_else(|| process::smoke_process_id())
        .ok_or(())?;
    open_file_for_process(pid, path)
}

pub fn close_file(fd: u32) -> Result<(), ()> {
    let pid = process::current_process_id()
        .or_else(|| process::smoke_process_id())
        .ok_or(())?;
    close_file_for_process(pid, fd)
}

pub fn close_file_for_process(pid: ProcessId, fd: u32) -> Result<(), ()> {
    let idx = fd as usize;
    match process::with_process_mut(pid, |process| {
        let table = process.fds_mut();
        if idx >= MAX_FDS || table[idx].is_none() {
            FD_REJECTED.fetch_add(1, Ordering::Relaxed);
            return Err(());
        }
        table[idx] = None;
        FD_CLOSES.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }) {
        Some(Ok(())) => Ok(()),
        _ => Err(()),
    }
}

pub fn read_fd(fd: u32, user_buf: u64, max_len: u64) -> Result<u64, ()> {
    let len = core::cmp::min(max_len, 64) as usize;
    if len == 0 || user_buf == 0 {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    let pid = process::current_process_id()
        .or_else(|| process::smoke_process_id())
        .ok_or(())?;
    let path = process::with_process_mut(pid, |process| {
        let idx = fd as usize;
        process
            .fds_mut()
            .get(idx)
            .and_then(|slot| slot.as_ref())
            .map(|slot| slot.path.clone())
    })
    .flatten()
    .ok_or(())?;
    if let Some(pipe_id) = crate::pipe::pipe_id_from_path(&path) {
        if !crate::pipe::is_pipe_read(&path) {
            FD_REJECTED.fetch_add(1, Ordering::Relaxed);
            return Err(());
        }
        return crate::pipe::read_pipe(pipe_id, user_buf, max_len);
    }
    let creds = process::process_owner(pid).unwrap_or(crate::security::current_credentials());
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
    let pid = process::current_process_id()
        .or_else(|| process::smoke_process_id())
        .ok_or(())?;
    let path = process::with_process_mut(pid, |process| {
        let idx = fd as usize;
        process
            .fds_mut()
            .get(idx)
            .and_then(|slot| slot.as_ref())
            .map(|slot| slot.path.clone())
    })
    .flatten()
    .ok_or(())?;
    if let Some(pipe_id) = crate::pipe::pipe_id_from_path(&path) {
        if !crate::pipe::is_pipe_write(&path) {
            FD_REJECTED.fetch_add(1, Ordering::Relaxed);
            return Err(());
        }
        return crate::pipe::write_pipe(pipe_id, user_buf, max_len);
    }
    if !path.starts_with("/tmp/") {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    let mut buf = [0u8; 32];
    crate::user_copy::copy_from_user(user_buf, &mut buf[..len]).map_err(|_| {
        FD_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    let payload = core::str::from_utf8(&buf[..len]).unwrap_or("smoke");
    let creds = process::process_owner(pid).unwrap_or(crate::security::current_credentials());
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

pub fn dup_fd(fd: u32) -> Result<u32, ()> {
    let pid = process::current_process_id()
        .or_else(|| process::smoke_process_id())
        .ok_or(())?;
    dup_fd_for_process(pid, fd)
}

pub fn read_fd_for_process(pid: ProcessId, fd: u32, user_buf: u64, max_len: u64) -> Result<u64, ()> {
    let len = core::cmp::min(max_len, 64) as usize;
    if len == 0 || user_buf == 0 {
        return Err(());
    }
    let path = process::with_process_mut(pid, |process| {
        let idx = fd as usize;
        process
            .fds_mut()
            .get(idx)
            .and_then(|slot| slot.as_ref())
            .map(|slot| slot.path.clone())
    })
    .flatten()
    .ok_or(())?;
    let creds = process::process_owner(pid).unwrap_or(crate::security::current_credentials());
    let contents = crate::storage::read_file_checked(creds, &path)
        .map_err(|_| ())?
        .ok_or(())?;
    let sample: alloc::vec::Vec<u8> = contents.as_bytes().iter().take(len).copied().collect();
    crate::user_copy::copy_to_user(&sample, user_buf).map_err(|_| ())?;
    FD_READS.fetch_add(1, Ordering::Relaxed);
    Ok(sample.len() as u64)
}

pub fn fcntl(fd: u32, cmd: u64, arg: u64) -> Result<u64, ()> {
    match cmd {
        F_GETFD => {
            FCNTL_GETFD.fetch_add(1, Ordering::Relaxed);
            let pid = process::current_process_id()
                .or_else(|| process::smoke_process_id())
                .ok_or(())?;
            let flags = process::with_process_mut(pid, |process| {
                process.fds_mut().get(fd as usize).and_then(|slot| slot.as_ref()).map(|slot| slot.flags)
            })
            .flatten()
            .ok_or(())?;
            Ok(flags as u64)
        }
        F_SETFD => {
            let pid = process::current_process_id()
                .or_else(|| process::smoke_process_id())
                .ok_or(())?;
            let updated = process::with_process_mut(pid, |process| {
                if let Some(slot) = process.fds_mut().get_mut(fd as usize) {
                    if let Some(entry) = slot.as_mut() {
                        entry.flags = (arg & 0xffff_ffff) as u32;
                        return true;
                    }
                }
                false
            })
            .unwrap_or(false);
            if !updated {
                FCNTL_REJECTED.fetch_add(1, Ordering::Relaxed);
                return Err(());
            }
            FCNTL_SETFD.fetch_add(1, Ordering::Relaxed);
            Ok(0)
        }
        F_DUPFD => {
            let out = dup_fd(fd).map(u64::from).map_err(|_| {
                FCNTL_REJECTED.fetch_add(1, Ordering::Relaxed);
            })?;
            FCNTL_DUPFD.fetch_add(1, Ordering::Relaxed);
            Ok(out)
        }
        _ => {
            FCNTL_REJECTED.fetch_add(1, Ordering::Relaxed);
            Err(())
        }
    }
}

pub fn fcntl_status() -> (u64, u64, u64) {
    (
        FCNTL_GETFD.load(Ordering::Relaxed),
        FCNTL_DUPFD.load(Ordering::Relaxed),
        FCNTL_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn fcntl_setfd_status() -> (u64, u64, u64) {
    (
        FCNTL_SETFD.load(Ordering::Relaxed),
        FCNTL_GETFD.load(Ordering::Relaxed),
        FCNTL_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn fork_lite_status() -> (u64, u64) {
    (
        FORK_INHERITED.load(Ordering::Relaxed),
        FORK_ISOLATED.load(Ordering::Relaxed),
    )
}

pub fn phase61_smoke() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = process::create_kernel_process_as("chdir-smoke", tick, creds) else {
        return false;
    };
    let chdir_ok = crate::user_path::chdir_for_process(pid, "/tmp").is_ok();
    let open_ok = open_file_for_process(pid, "../bin/hello").is_ok();
    let bad = crate::user_path::chdir_for_process(pid, "/tmp/../etc/passwd").is_err();
    let (normalized, chdirs) = crate::user_path::chdir_status();
    chdir_ok && open_ok && bad && normalized > 0 && chdirs > 0
}

pub fn phase64_smoke() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(parent) = process::create_kernel_process_as("fork-parent", tick, creds) else {
        return false;
    };
    let Some(fd) = open_file_for_process(parent, "/bin/hello").ok() else {
        return false;
    };
    let Some(child) = process::fork_lite(parent, tick.saturating_add(1)) else {
        return false;
    };
    let _ = close_file_for_process(parent, fd);
    let child_open = process::with_process_mut(child, |p| p.fds_mut()[0].is_some()).unwrap_or(false);
    if child_open {
        FORK_INHERITED.fetch_add(1, Ordering::Relaxed);
        FORK_ISOLATED.fetch_add(1, Ordering::Relaxed);
    }
    child_open
}

pub fn phase66_smoke() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = process::create_kernel_process_as("fcntl-smoke", tick, creds) else {
        return false;
    };
    process::set_smoke_process_id(Some(pid));
    let Some(fd) = open_file_for_process(pid, "/bin/hello").ok() else {
        return false;
    };
    let getfd = fcntl(fd, F_GETFD, 0).is_ok();
    let dup = fcntl(fd, F_DUPFD, 0).is_ok();
    let reject = fcntl(fd, 99, 0).is_err();
    process::set_smoke_process_id(None);
    let (getfd_n, dup_n, rejected) = fcntl_status();
    getfd && dup && reject && getfd_n > 0 && dup_n > 0 && rejected > 0
}

pub fn phase76_smoke() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = process::create_kernel_process_as("fcntl-setfd-smoke", tick, creds) else {
        return false;
    };
    process::set_smoke_process_id(Some(pid));
    let Some(fd) = open_file_for_process(pid, "/bin/hello").ok() else {
        return false;
    };
    let setfd = fcntl(fd, F_SETFD, FD_CLOEXEC).is_ok();
    let getfd = fcntl(fd, F_GETFD, 0).ok() == Some(FD_CLOEXEC);
    let reject = fcntl(fd, 99, 0).is_err();
    process::set_smoke_process_id(None);
    let (setfd_n, getfd_n, rejected) = fcntl_setfd_status();
    setfd && getfd && reject && setfd_n > 0 && getfd_n > 0 && rejected > 0
}

pub fn phase45_smoke() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let Some(pid) = process::create_kernel_process_as("fd-smoke-a", tick, crate::security::Credentials::shell_user())
    else {
        return false;
    };
    process::set_smoke_process_id(Some(pid));
    let Some(fd) = open_file_for_process(pid, "/bin/hello").ok() else {
        return false;
    };
    let closed = close_file_for_process(pid, fd).is_ok();
    process::set_smoke_process_id(None);
    let (opens, closes, _, _, _) = status();
    closed && opens > 0 && closes > 0
}

pub fn phase46_smoke() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let Some(pid) = process::create_kernel_process_as("fd-io-smoke", tick, crate::security::Credentials::shell_user())
    else {
        return false;
    };
    process::set_smoke_process_id(Some(pid));
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(
        crate::security::Credentials::shell_user(),
        "hello",
    )
    .ok() else {
        return false;
    };
    let _ = process::set_process_cr3(pid, built.hw.cr3_phys);
    let Some(fd) = open_file_for_process(pid, "/bin/hello").ok() else {
        return false;
    };
    let user_buf = crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(128);
    let before = FD_READS.load(Ordering::Relaxed);
    process::set_current_process_id(Some(pid));
    let ok = crate::user_paging::with_user_page_table(&built.hw, || read_fd(fd, user_buf, 16).ok())
        .ok()
        .flatten()
        .is_some();
    process::set_current_process_id(None);
    let _ = close_file_for_process(pid, fd);
    process::set_smoke_process_id(None);
    ok && FD_READS.load(Ordering::Relaxed) > before
}

pub fn phase51_smoke() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid_a) = process::create_kernel_process_as("proc-fd-a", tick, creds) else {
        return false;
    };
    let Some(pid_b) = process::create_kernel_process_as("proc-fd-b", tick.saturating_add(1), creds) else {
        return false;
    };
    let fd_a = open_file_for_process(pid_a, "/bin/hello").ok();
    let fd_b = open_file_for_process(pid_b, "/bin/hello").ok();
    if fd_a.is_none() || fd_b.is_none() {
        return false;
    }
    let close_a = close_file_for_process(pid_a, fd_a.unwrap()).is_ok();
    let still_open = process::with_process_mut(pid_b, |p| p.fds_mut()[0].is_some())
        .unwrap_or(false);
    if close_a && still_open {
        PROC_FD_ISOLATED.store(1, Ordering::Relaxed);
    }
    close_a && still_open
}

pub fn phase52_smoke() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = process::create_kernel_process_as("fd-dup-smoke", tick, creds) else {
        return false;
    };
    let _ = process::set_process_cwd(pid, "/tmp");
    let fd0 = open_file_for_process(pid, "/bin/hello").ok();
    let dup = fd0.and_then(|fd| dup_fd_for_process(pid, fd).ok());
    let relative = open_file_for_process(pid, "phase52-smoke.txt").ok();
    if fd0.is_none() || dup.is_none() || relative.is_none() {
        return false;
    }
    let dup_reads = process::with_process_mut(pid, |p| {
        p.fds_mut()[fd0.unwrap() as usize].is_some() && p.fds_mut()[dup.unwrap() as usize].is_some()
    })
    .unwrap_or(false);
    dup_reads && FD_DUPS.load(Ordering::Relaxed) > 0
}
