//! User-supplied path validation and copyin .

use core::sync::atomic::{AtomicU64, Ordering};

static PATH_READS: AtomicU64 = AtomicU64::new(0);
static PATH_WRITES: AtomicU64 = AtomicU64::new(0);
static PATH_REJECTED: AtomicU64 = AtomicU64::new(0);
static PATH_NORMALIZED: AtomicU64 = AtomicU64::new(0);
static CHDIR_COUNT: AtomicU64 = AtomicU64::new(0);
static RING3_CHDIRS: AtomicU64 = AtomicU64::new(0);
static GETCWD_READS: AtomicU64 = AtomicU64::new(0);
static CHDIRPROBE_OK: AtomicU64 = AtomicU64::new(0);

pub const MAX_USER_PATH_LEN: usize = 96;

pub fn status() -> (u64, u64) {
    (
        PATH_READS.load(Ordering::Relaxed),
        PATH_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn write_status() -> (u64, u64) {
    (
        PATH_WRITES.load(Ordering::Relaxed),
        PATH_READS.load(Ordering::Relaxed),
    )
}

pub fn chdir_status() -> (u64, u64) {
    (
        PATH_NORMALIZED.load(Ordering::Relaxed),
        CHDIR_COUNT.load(Ordering::Relaxed),
    )
}

pub fn getcwd_status() -> u64 {
    GETCWD_READS.load(Ordering::Relaxed)
}

pub fn chdirprobe_status() -> u64 {
    CHDIRPROBE_OK.load(Ordering::Relaxed)
}

pub fn getcwd_to_user(user_buf: u64) -> Result<usize, ()> {
    if user_buf == 0 {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    let pid = crate::task::process::current_process_id()
        .or_else(|| crate::task::process::smoke_process_id())
        .ok_or(())?;
    let cwd = crate::task::process::process_cwd(pid).ok_or(())?;
    let bytes = cwd.as_bytes();
    let len = core::cmp::min(bytes.len(), MAX_USER_PATH_LEN);
    crate::user_copy::copy_to_user(&bytes[..len], user_buf).map_err(|_| {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    let _ = crate::user_copy::copy_to_user(&[0u8], user_buf + len as u64);
    GETCWD_READS.fetch_add(1, Ordering::Relaxed);
    Ok(len)
}

pub fn smoke_getcwd() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = crate::task::process::create_kernel_process_as("getcwd-smoke", tick, creds)
    else {
        return false;
    };
    crate::task::process::set_smoke_process_id(Some(pid));
    let chdir_ok = crate::task::process::set_process_cwd(pid, "/tmp");
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(creds, "hello").ok()
    else {
        return false;
    };
    let user_buf = crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(128);
    let copy_ok = crate::user_paging::with_user_page_table(&built.hw, || {
        getcwd_to_user(user_buf).ok()?;
        Some(true)
    })
    .ok()
    .flatten()
    .is_some();
    crate::task::process::set_smoke_process_id(None);
    chdir_ok
        && copy_ok
        && crate::task::process::process_cwd(pid).as_deref() == Some("/tmp")
        && getcwd_status() > 0
}

pub fn smoke_chdir_probe() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = crate::task::process::create_kernel_process_as("chdirprobe", tick, creds)
    else {
        return false;
    };
    let Some(built) =
        crate::task::program_loader::build_hw_page_table_program(creds, "chdirprobe").ok()
    else {
        return false;
    };
    crate::task::process::set_smoke_process_id(Some(pid));
    crate::task::process::set_current_process_id(Some(pid));
    let user_path = crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(128);
    let user_buf = user_path.saturating_sub(64);
    let ok = crate::user_paging::with_user_page_table(&built.hw, || {
        for (i, byte) in b"/tmp".iter().enumerate() {
            crate::user_copy::copy_to_user(core::slice::from_ref(byte), user_path + i as u64)
                .ok()?;
        }
        let _ = crate::user_copy::copy_to_user(&[0u8], user_path + b"/tmp".len() as u64);
        chdir_from_user(user_path).ok()?;
        getcwd_to_user(user_buf).ok()?;
        Some(crate::task::process::process_cwd(pid).as_deref() == Some("/tmp"))
    })
    .ok()
    .flatten()
    .unwrap_or(false);
    crate::task::process::set_smoke_process_id(None);
    crate::task::process::set_current_process_id(None);
    if ok {
        CHDIRPROBE_OK.fetch_add(1, Ordering::Relaxed);
        RING3_CHDIRS.fetch_add(1, Ordering::Relaxed);
    }
    ok && chdirprobe_status() > 0 && getcwd_status() > 0
}

pub fn ring3_chdir_status() -> u64 {
    RING3_CHDIRS.load(Ordering::Relaxed)
}

pub fn normalize_absolute_path(path: &str) -> Result<alloc::string::String, ()> {
    if !path.starts_with('/') {
        return Err(());
    }
    let mut parts = alloc::vec::Vec::new();
    for segment in path.split('/') {
        if segment.is_empty() || segment == "." {
            continue;
        }
        if segment == ".." {
            parts.pop();
            continue;
        }
        parts.push(segment);
    }
    let mut out = alloc::string::String::from("/");
    for (idx, part) in parts.iter().enumerate() {
        if idx > 0 {
            out.push('/');
        }
        out.push_str(part);
    }
    if out.len() > MAX_USER_PATH_LEN {
        return Err(());
    }
    PATH_NORMALIZED.fetch_add(1, Ordering::Relaxed);
    Ok(out)
}

pub fn validate_user_path(path: &str) -> bool {
    if !path.starts_with('/') {
        return false;
    }
    if path.contains("..") {
        return false;
    }
    if path.len() > MAX_USER_PATH_LEN {
        return false;
    }
    path == "/tmp"
        || path.starts_with("/tmp/")
        || path == "/bin"
        || path.starts_with("/bin/")
        || path == "/lib"
        || path.starts_with("/lib/")
}

pub fn copy_raw_path_from_user(user_ptr: u64) -> Result<alloc::string::String, ()> {
    let mut buf = [0u8; MAX_USER_PATH_LEN];
    let mut len = 0usize;
    for i in 0..MAX_USER_PATH_LEN {
        let mut byte = [0u8; 1];
        crate::user_copy::copy_from_user(user_ptr.saturating_add(i as u64), &mut byte).map_err(
            |_| {
                PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
            },
        )?;
        if byte[0] == 0 {
            break;
        }
        buf[len] = byte[0];
        len += 1;
    }
    if len == 0 || buf[0] != b'/' {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    core::str::from_utf8(&buf[..len])
        .map_err(|_| {
            PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
        })
        .map(alloc::string::String::from)
}

pub fn chdir_for_process(pid: crate::task::process::ProcessId, path: &str) -> Result<(), ()> {
    let normalized = normalize_absolute_path(path).map_err(|_| {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    if !validate_user_path(&normalized) {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    if !crate::task::process::set_process_cwd(pid, &normalized) {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    CHDIR_COUNT.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

pub fn chdir_from_user(user_ptr: u64) -> Result<(), ()> {
    let raw = copy_raw_path_from_user(user_ptr)?;
    let pid = crate::task::process::current_process_id()
        .or_else(|| crate::task::process::smoke_process_id())
        .ok_or(())?;
    chdir_for_process(pid, &raw)
}

pub fn copy_path_from_user(user_ptr: u64) -> Result<alloc::string::String, ()> {
    let mut buf = [0u8; MAX_USER_PATH_LEN];
    let mut len = 0usize;
    for i in 0..MAX_USER_PATH_LEN {
        let mut byte = [0u8; 1];
        crate::user_copy::copy_from_user(user_ptr.saturating_add(i as u64), &mut byte).map_err(
            |_| {
                PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
            },
        )?;
        if byte[0] == 0 {
            break;
        }
        buf[len] = byte[0];
        len += 1;
    }
    if len == 0 {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    let path = core::str::from_utf8(&buf[..len]).map_err(|_| {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    if !validate_user_path(path) {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    Ok(alloc::string::String::from(path))
}

pub fn read_path_probe(user_path_ptr: u64, user_buf: u64) -> Result<u64, ()> {
    let path = copy_path_from_user(user_path_ptr)?;
    let creds = crate::security::current_credentials();
    let contents = crate::storage::read_file_checked(creds, &path)
        .map_err(|_| {
            PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
        })?
        .ok_or_else(|| {
            PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
        })?;
    let sample: alloc::vec::Vec<u8> = contents.as_bytes().iter().take(32).copied().collect();
    crate::user_copy::copy_to_user(&sample, user_buf).map_err(|_| {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    PATH_READS.fetch_add(1, Ordering::Relaxed);
    Ok(sample.len() as u64)
}

pub fn write_path_probe(user_path_ptr: u64, user_data_ptr: u64) -> Result<u64, ()> {
    let path = copy_path_from_user(user_path_ptr)?;
    if !path.starts_with("/tmp/") {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    let mut buf = [0u8; 32];
    crate::user_copy::copy_from_user(user_data_ptr, &mut buf).map_err(|_| {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    let payload = core::str::from_utf8(&buf[..len]).map_err(|_| {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    let creds = crate::security::current_credentials();
    crate::storage::write_file_checked(creds, &path, payload).map_err(|_| {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    PATH_WRITES.fetch_add(1, Ordering::Relaxed);
    Ok(len as u64)
}

pub fn smoke_user_path_write() -> bool {
    let path = "/tmp/user-path-smoke";
    let creds = crate::security::Credentials::shell_user();
    if crate::storage::write_file_checked(creds, path, "user-path-ok").is_err()
        && crate::storage::write_file_checked(
            crate::security::Credentials::admin(),
            path,
            "user-path-ok",
        )
        .is_err()
    {
        return false;
    }
    let verified = matches!(
        crate::storage::read_file_checked(creds, path),
        Ok(Some(contents)) if contents == "user-path-ok"
    );
    if verified {
        PATH_WRITES.fetch_add(1, Ordering::Relaxed);
        PATH_READS.fetch_add(1, Ordering::Relaxed);
    }
    verified
}

pub fn smoke_ring3_chdir() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = crate::task::process::create_kernel_process_as("ring3-chdir", tick, creds)
    else {
        return false;
    };
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(creds, "hello").ok()
    else {
        return false;
    };
    let _ = crate::task::process::set_process_cr3(pid, built.hw.cr3_phys);
    crate::task::process::set_smoke_process_id(Some(pid));
    crate::task::process::set_current_process_id(Some(pid));
    let user_path = crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(128);
    let bad_path = crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(192);
    let chdir_ok = crate::user_paging::with_user_page_table(&built.hw, || {
        for (i, byte) in b"/tmp".iter().enumerate() {
            crate::user_copy::copy_to_user(core::slice::from_ref(byte), user_path + i as u64)
                .ok()?;
        }
        let _ = crate::user_copy::copy_to_user(&[0u8], user_path + b"/tmp".len() as u64);
        chdir_from_user(user_path).ok()
    })
    .ok()
    .flatten()
    .is_some();
    let cwd_ok = crate::task::process::process_cwd(pid).as_deref() == Some("/tmp");
    let bad = crate::user_paging::with_user_page_table(&built.hw, || {
        for (i, byte) in b"/tmp/../etc/passwd".iter().enumerate() {
            crate::user_copy::copy_to_user(core::slice::from_ref(byte), bad_path + i as u64)
                .ok()?;
        }
        let _ =
            crate::user_copy::copy_to_user(&[0u8], bad_path + b"/tmp/../etc/passwd".len() as u64);
        Some(chdir_from_user(bad_path).is_err())
    })
    .ok()
    .flatten()
    .unwrap_or(false);
    crate::task::process::set_smoke_process_id(None);
    crate::task::process::set_current_process_id(None);
    if chdir_ok && cwd_ok {
        RING3_CHDIRS.fetch_add(1, Ordering::Relaxed);
    }
    chdir_ok && cwd_ok && bad && RING3_CHDIRS.load(Ordering::Relaxed) > 0
}

pub fn smoke_user_path_read() -> bool {
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(
        crate::security::Credentials::shell_user(),
        "hello",
    )
    .ok() else {
        return false;
    };
    let user_path = crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(256);
    let user_buf = user_path.saturating_sub(64);
    let before = PATH_READS.load(Ordering::Relaxed);
    let ok = crate::user_paging::with_user_page_table(&built.hw, || {
        crate::user_paging::map_demand_zero_page(built.hw.cr3_phys, user_path & !0xfff).ok();
        for (i, byte) in b"/bin/hello".iter().enumerate() {
            crate::user_copy::copy_to_user(core::slice::from_ref(byte), user_path + i as u64)
                .ok()?;
        }
        let _ = crate::user_copy::copy_to_user(&[0u8], user_path + b"/bin/hello".len() as u64);
        read_path_probe(user_path, user_buf).ok()?;
        Some(())
    })
    .ok()
    .flatten()
    .is_some();
    ok && PATH_READS.load(Ordering::Relaxed) > before
}
