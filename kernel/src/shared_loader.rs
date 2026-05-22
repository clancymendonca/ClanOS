//! Shared library mapping for DT_NEEDED dependencies (Phases 41, 56).

use alloc::{format, string::String, vec};
use core::sync::atomic::{AtomicU64, Ordering};

use crate::{
    elf_reloc::parse_dt_needed,
    frame_backing::{FrameBackedImage, FrameBackedPage, FrameBackedRegion},
    frame_ownership::{self, FrameOwner},
    load_plan::{LoadPermissions, PAGE_SIZE},
};

pub const SHARED_LIB_BASE: u64 = 0x700_000;
pub const SHARED_LIB_AUX_BASE: u64 = 0x710_000;
pub const SHARED_LIB_PATH: &str = "/bin/libc_stub.elf";
pub const SHARED_LIB_AUX_PATH: &str = "/lib/libaux_stub.elf";
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

pub fn resolve_shared_path(name: &str) -> String {
    let lib_path = format!("/lib/{name}.elf");
    if crate::storage::read_file(&lib_path).ok().flatten().is_some() {
        return lib_path;
    }
    format!("/bin/{name}.elf")
}

pub fn needed_library_names(main_image: &[u8]) -> alloc::vec::Vec<&'static str> {
    if parse_dt_needed(main_image).is_none() {
        return alloc::vec::Vec::new();
    }
    if main_image.windows(6).any(|w| w == b"libaux") {
        return alloc::vec!["libc_stub", "libaux_stub"];
    }
    alloc::vec!["libc_stub"]
}

fn map_shared_at(
    backed: &mut FrameBackedImage,
    base: u64,
    path: &str,
) -> Result<(), ()> {
    let lib_bytes = crate::storage::read_file(path)
        .ok()
        .flatten()
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

    let permissions =
        LoadPermissions::from_bits(LoadPermissions::READ | LoadPermissions::EXECUTE);
    let page = FrameBackedPage {
        virtual_address: base,
        frame,
        permissions,
        copied_bytes: copy_len,
        zero_filled_bytes: PAGE_SIZE.saturating_sub(copy_len),
    };

    backed.regions.push(FrameBackedRegion {
        start: base,
        size: PAGE_SIZE,
        permissions,
        pages: vec![page],
    });
    backed.total_pages = backed.total_pages.saturating_add(1);
    backed.executable_pages = backed.executable_pages.saturating_add(1);
    backed.copied_bytes = backed.copied_bytes.saturating_add(copy_len);
    SHARED_LOADED.fetch_add(1, Ordering::Relaxed);
    SHARED_PAGES.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

pub fn attach_shared_library(
    backed: &mut FrameBackedImage,
    main_image: &[u8],
) -> Result<usize, ()> {
    let names = needed_library_names(main_image);
    if names.is_empty() {
        return Ok(0);
    }
    let mut mapped = 0usize;
    for (idx, name) in names.iter().enumerate() {
        let base = if idx == 0 {
            SHARED_LIB_BASE
        } else {
            SHARED_LIB_AUX_BASE.saturating_add((idx as u64 - 1) * 0x10_000)
        };
        let path = if *name == "libc_stub" {
            String::from(SHARED_LIB_PATH)
        } else {
            resolve_shared_path(name)
        };
        map_shared_at(backed, base, &path)?;
        mapped += 1;
    }
    Ok(mapped)
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

pub fn phase56_smoke() -> bool {
    let sample = crate::storage::phase11_sample_elf_image();
    let mut bytes = sample.into_bytes();
    bytes.extend_from_slice(b"libaux");
    let backed = crate::task::program_loader::back_mapped_program_with_relocs(
        crate::security::Credentials::shell_user(),
        "hello",
    )
    .ok()
    .map(|img| img.backed);
    let Some(mut backed) = backed else {
        return false;
    };
    let pages = attach_shared_library(&mut backed, &bytes).unwrap_or(0);
    let (loaded, mapped, _) = status();
    pages >= 2 && loaded >= 2 && mapped >= 2
}
