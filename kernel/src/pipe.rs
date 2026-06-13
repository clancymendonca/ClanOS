//! Anonymous pipe ring buffer .

use alloc::format;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::task::process::{self, ProcessId};

const PIPE_CAPACITY: usize = 64;
const MAX_PIPES: usize = 4;

static PIPE_CREATED: AtomicU64 = AtomicU64::new(0);
static PIPE_BYTES: AtomicU64 = AtomicU64::new(0);
static POLL_CALLS: AtomicU64 = AtomicU64::new(0);
static POLL_READY: AtomicU64 = AtomicU64::new(0);
static HW_PIPE_PROBES: AtomicU64 = AtomicU64::new(0);

struct PipeBuffer {
    data: [u8; PIPE_CAPACITY],
    len: usize,
}

impl PipeBuffer {
    const fn empty() -> Self {
        Self {
            data: [0; PIPE_CAPACITY],
            len: 0,
        }
    }
}

static PIPES: spin::Mutex<[PipeBuffer; MAX_PIPES]> =
    spin::Mutex::new([const { PipeBuffer::empty() }; MAX_PIPES]);

pub fn status() -> (u64, u64) {
    (
        PIPE_CREATED.load(Ordering::Relaxed),
        PIPE_BYTES.load(Ordering::Relaxed),
    )
}

pub fn poll_status() -> (u64, u64) {
    (
        POLL_CALLS.load(Ordering::Relaxed),
        POLL_READY.load(Ordering::Relaxed),
    )
}

pub fn pipeprobe_status() -> (u64, u64) {
    (
        HW_PIPE_PROBES.load(Ordering::Relaxed),
        PIPE_BYTES.load(Ordering::Relaxed),
    )
}

pub fn pipe_readable(pipe_id: u32) -> bool {
    let pipes = PIPES.lock();
    pipes
        .get(pipe_id as usize)
        .map(|pipe| pipe.len > 0)
        .unwrap_or(false)
}

pub fn poll_syscall(fd: u64, events: u64) -> Result<u64, ()> {
    POLL_CALLS.fetch_add(1, Ordering::Relaxed);
    let pid = process::current_process_id()
        .or_else(|| process::smoke_process_id())
        .ok_or(())?;
    let path = process::with_process_mut(pid, |process| {
        process
            .fds_mut()
            .get(fd as usize)
            .and_then(|slot| slot.as_ref())
            .map(|slot| slot.path.clone())
    })
    .flatten()
    .ok_or(())?;
    let pipe_id = pipe_id_from_path(&path).ok_or(())?;
    if !is_pipe_read(&path) {
        return Err(());
    }
    let readable = (events & 1) != 0 && pipe_readable(pipe_id);
    if readable {
        POLL_READY.fetch_add(1, Ordering::Relaxed);
        Ok(1)
    } else {
        Ok(0)
    }
}

fn alloc_pipe_id() -> Option<u32> {
    let mut pipes = PIPES.lock();
    for (idx, pipe) in pipes.iter_mut().enumerate() {
        if pipe.len == 0 && pipe.data.iter().all(|b| *b == 0) {
            *pipe = PipeBuffer::empty();
            return Some(idx as u32);
        }
    }
    None
}

pub fn pipe_syscall(read_fd_out: u64, write_fd_out: u64) -> Result<u64, ()> {
    if read_fd_out == 0 || write_fd_out == 0 {
        return Err(());
    }
    let pid = process::current_process_id()
        .or_else(|| process::smoke_process_id())
        .ok_or(())?;
    let (read_fd, write_fd, _pipe_id) = pipe_create_for_process(pid)?;
    crate::user_copy::copy_to_user(&read_fd.to_le_bytes(), read_fd_out).map_err(|_| ())?;
    crate::user_copy::copy_to_user(&write_fd.to_le_bytes(), write_fd_out).map_err(|_| ())?;
    Ok(0)
}

pub fn pipe_create_for_process(pid: ProcessId) -> Result<(u32, u32, u32), ()> {
    let pipe_id = alloc_pipe_id().ok_or(())?;
    let read_path = format!("/@pipe/{pipe_id}/r");
    let write_path = format!("/@pipe/{pipe_id}/w");
    let read_fd = crate::fd_table::open_pipe_for_process(pid, &read_path)?;
    let write_fd = crate::fd_table::open_pipe_for_process(pid, &write_path)?;
    PIPE_CREATED.fetch_add(1, Ordering::Relaxed);
    Ok((read_fd, write_fd, pipe_id))
}

pub fn pipe_id_from_path(path: &str) -> Option<u32> {
    if !path.starts_with("/@pipe/") {
        return None;
    }
    let rest = path.strip_prefix("/@pipe/")?;
    let id = rest.split('/').next()?;
    id.parse().ok()
}

pub fn is_pipe_read(path: &str) -> bool {
    path.ends_with("/r")
}

pub fn is_pipe_write(path: &str) -> bool {
    path.ends_with("/w")
}

pub fn write_pipe_kernel(pipe_id: u32, bytes: &[u8]) -> Result<usize, ()> {
    let mut pipes = PIPES.lock();
    let pipe = pipes.get_mut(pipe_id as usize).ok_or(())?;
    if pipe.len.saturating_add(bytes.len()) > PIPE_CAPACITY {
        return Err(());
    }
    pipe.data[pipe.len..pipe.len + bytes.len()].copy_from_slice(bytes);
    pipe.len += bytes.len();
    PIPE_BYTES.fetch_add(bytes.len() as u64, Ordering::Relaxed);
    Ok(bytes.len())
}

pub fn read_pipe(pipe_id: u32, user_buf: u64, max_len: u64) -> Result<u64, ()> {
    let len = core::cmp::min(max_len as usize, PIPE_CAPACITY);
    if len == 0 || user_buf == 0 {
        return Err(());
    }
    let mut pipes = PIPES.lock();
    let pipe = pipes.get_mut(pipe_id as usize).ok_or(())?;
    let take = core::cmp::min(len, pipe.len);
    let out = &pipe.data[..take];
    crate::user_copy::copy_to_user(out, user_buf).map_err(|_| ())?;
    if take > 0 {
        pipe.data.copy_within(take..pipe.len, 0);
        pipe.len -= take;
    }
    PIPE_BYTES.fetch_add(take as u64, Ordering::Relaxed);
    Ok(take as u64)
}

pub fn write_pipe(pipe_id: u32, user_buf: u64, max_len: u64) -> Result<u64, ()> {
    let len = core::cmp::min(max_len as usize, PIPE_CAPACITY);
    if len == 0 || user_buf == 0 {
        return Err(());
    }
    let mut buf = [0u8; PIPE_CAPACITY];
    crate::user_copy::copy_from_user(user_buf, &mut buf[..len]).map_err(|_| ())?;
    write_pipe_kernel(pipe_id, &buf[..len]).map(|n| n as u64)
}

pub fn smoke_pipe_lite() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = process::create_kernel_process_as("pipe-smoke", tick, creds) else {
        return false;
    };
    process::set_smoke_process_id(Some(pid));
    let Ok((read_fd, write_fd, pipe_id)) = pipe_create_for_process(pid) else {
        return false;
    };
    let wrote = write_pipe_kernel(pipe_id, b"pipe87").is_ok();
    let user_buf = crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(128);
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(creds, "hello").ok()
    else {
        return false;
    };
    let _ = process::set_process_cr3(pid, built.hw.cr3_phys);
    process::set_current_process_id(Some(pid));
    let read_ok = crate::user_paging::with_user_page_table(&built.hw, || {
        crate::fd_table::read_fd(read_fd, user_buf, 8).ok()
    })
    .ok()
    .flatten()
    .is_some();
    let bad_write = crate::fd_table::write_fd(read_fd, user_buf, 4).is_err();
    let _ = (write_fd, bad_write);
    process::set_smoke_process_id(None);
    process::set_current_process_id(None);
    let (pipes, bytes) = status();
    wrote && read_ok && pipes > 0 && bytes > 0
}

pub fn smoke_poll_lite() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = process::create_kernel_process_as("poll-smoke", tick, creds) else {
        return false;
    };
    process::set_smoke_process_id(Some(pid));
    let Ok((read_fd, _write_fd, pipe_id)) = pipe_create_for_process(pid) else {
        return false;
    };
    let empty = poll_syscall(read_fd as u64, 1).ok() == Some(0);
    let _ = write_pipe_kernel(pipe_id, b"x");
    let ready = poll_syscall(read_fd as u64, 1).ok() == Some(1);
    process::set_smoke_process_id(None);
    let (polls, ready_n) = poll_status();
    empty && ready && polls > 0 && ready_n > 0
}

pub fn smoke_pipe_probe() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = process::create_kernel_process_as("pipeprobe", tick, creds) else {
        return false;
    };
    process::set_smoke_process_id(Some(pid));
    process::set_current_process_id(Some(pid));
    let Ok((read_fd, write_fd, pipe_id)) = pipe_create_for_process(pid) else {
        return false;
    };
    let _ = write_pipe_kernel(pipe_id, b"pipe95");
    let Some(built) =
        crate::task::program_loader::build_hw_page_table_program(creds, "pipeprobe").ok()
    else {
        return false;
    };
    let _ = process::set_process_cr3(pid, built.hw.cr3_phys);
    let user_buf = crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(128);
    let read_ok = crate::user_paging::with_user_page_table(&built.hw, || {
        crate::fd_table::read_fd(read_fd, user_buf, 8).ok()
    })
    .ok()
    .flatten()
    .is_some();
    HW_PIPE_PROBES.fetch_add(1, Ordering::Relaxed);
    let _ = (write_fd,);
    process::set_smoke_process_id(None);
    process::set_current_process_id(None);
    let (probes, bytes) = pipeprobe_status();
    read_ok && probes > 0 && bytes > 0
}
