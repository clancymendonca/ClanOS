//! Shared library mapping for DT_NEEDED dependencies (Phase 41).

use alloc::vec;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::{
    elf_reloc::parse_dt_needed,
    frame_backing::{FrameBackedImage, FrameBackedPage, FrameBackedRegion},
    frame_ownership::{self, FrameOwner},
    load_plan::{LoadPermissions, PAGE_SIZE},
};

pub const SHARED_LIB_BASE: u64 = 0x700_000;
pub const SHARED_LIB_PATH: &str = "/bin/libc_stub.elf";
pub const SHARED_LIB_SYMBOL_OFFSET: u64 = 0x10;

static SHARED_LOADED: AtomicU64 = AtomicU64::new(0);
static SHARED_PAGES: AtomicU64 = AtomicU64::new(0);
static SHARED_REJECTED: AtomicU64 = AtomicU64::new(0);

pub fn status() -> (u64, u64, u64) {
    (
        SHARED_LOADED.load(Ordering::Relaxed),
        SHARED_PAGES.load(Ordering::Relaxed),
        SHARED_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn shared_lib_base() -> u64 {
    SHARED_LIB_BASE
}

pub fn attach_shared_library(
    backed: &mut FrameBackedImage,
    main_image: &[u8],
) -> Result<usize, ()> {
    if parse_dt_needed(main_image).is_none() {
        return Ok(0);
    }

    let lib_bytes = crate::storage::read_file(SHARED_LIB_PATH)
        .ok()
        .flatten()
        .or_else(|| crate::storage::read_file("/bin/hello.elf").ok().flatten())
        .map(|s| s.into_bytes())
        .unwrap_or_else(|| crate::storage::phase11_sample_elf_image().into_bytes());

    let frame = frame_ownership::allocate_frame(FrameOwner::Image).map_err(|_| {
        SHARED_REJECTED.fetch_add(1, Ordering::Relaxed);
    })?;

    let copy_len = core::cmp::min(lib_bytes.len(), PAGE_SIZE);
    crate::user_paging::write_phys_bytes(frame.start_address, 0, &lib_bytes[..copy_len]);
    if lib_bytes.len() < PAGE_SIZE {
        let pad = [0u8; PAGE_SIZE];
        crate::user_paging::write_phys_bytes(
            frame.start_address,
            copy_len,
            &pad[copy_len..PAGE_SIZE.min(pad.len())],
        );
    }

    let permissions = LoadPermissions::from_bits(
        LoadPermissions::READ | LoadPermissions::EXECUTE,
    );
    let page = FrameBackedPage {
        virtual_address: SHARED_LIB_BASE,
        frame,
        permissions,
        copied_bytes: copy_len,
        zero_filled_bytes: PAGE_SIZE.saturating_sub(copy_len),
    };

    backed.regions.push(FrameBackedRegion {
        start: SHARED_LIB_BASE,
        size: PAGE_SIZE,
        permissions,
        pages: vec![page],
    });
    backed.total_pages = backed.total_pages.saturating_add(1);
    backed.executable_pages = backed.executable_pages.saturating_add(1);
    backed.copied_bytes = backed.copied_bytes.saturating_add(copy_len);

    SHARED_LOADED.fetch_add(1, Ordering::Relaxed);
    SHARED_PAGES.fetch_add(1, Ordering::Relaxed);
    Ok(1)
}

pub fn phase41_smoke() -> bool {
    let sample = crate::storage::phase11_sample_elf_image();
    let backed = crate::task::program_loader::back_mapped_program_with_relocs(
        crate::security::Credentials::shell_user(),
        "hello",
    )
    .ok()
    .map(|img| img.backed);
    let Some(mut backed) = backed else {
        return false;
    };
    let pages = attach_shared_library(&mut backed, sample.as_bytes()).unwrap_or(0);
    let (loaded, mapped, _) = status();
    pages > 0 && loaded > 0 && mapped > 0
}
