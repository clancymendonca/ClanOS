//! User-supplied path validation and copyin (Phase 44).

use core::sync::atomic::{AtomicU64, Ordering};

static PATH_READS: AtomicU64 = AtomicU64::new(0);
static PATH_REJECTED: AtomicU64 = AtomicU64::new(0);

pub const MAX_USER_PATH_LEN: usize = 96;

pub fn status() -> (u64, u64) {
    (
        PATH_READS.load(Ordering::Relaxed),
        PATH_REJECTED.load(Ordering::Relaxed),
    )
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
    path.starts_with("/bin/") || path.starts_with("/tmp/")
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
    let sample: alloc::vec::Vec<u8> = contents
        .as_bytes()
        .iter()
        .take(32)
        .copied()
        .collect();
    crate::user_copy::copy_to_user(&sample, user_buf).map_err(|_| {
        PATH_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;
    PATH_READS.fetch_add(1, Ordering::Relaxed);
    Ok(sample.len() as u64)
}

pub fn phase44_smoke() -> bool {
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
            crate::user_copy::copy_to_user(core::slice::from_ref(byte), user_path + i as u64).ok()?;
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
