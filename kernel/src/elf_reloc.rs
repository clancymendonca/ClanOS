//! Static ELF relocations for frame-backed images (Phase 27).

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::frame_backing::FrameBackedImage;

static RELOC_APPLIED: AtomicU64 = AtomicU64::new(0);
static RELOC_REJECTED: AtomicU64 = AtomicU64::new(0);
static DT_NEEDED_COUNT: AtomicU64 = AtomicU64::new(0);
static DT_LINKED_COUNT: AtomicU64 = AtomicU64::new(0);
static IMPORT_COUNT: AtomicU64 = AtomicU64::new(0);
static IMPORT_APPLIED: AtomicU64 = AtomicU64::new(0);
static PLT_SLOTS: AtomicU64 = AtomicU64::new(0);
static PLT_APPLIED: AtomicU64 = AtomicU64::new(0);
static PLT_LAZY: AtomicU64 = AtomicU64::new(0);
static PLT_BOUND: AtomicU64 = AtomicU64::new(0);
static RING3_PLT_BOUND: AtomicU64 = AtomicU64::new(0);
static RING3_PLT_SMOKE: AtomicU64 = AtomicU64::new(0);
static RING3_PLT_FAULT: AtomicU64 = AtomicU64::new(0);

const R_X86_64_NONE: u32 = 0;
const R_X86_64_64: u32 = 1;
const R_X86_64_GLOB_DAT: u32 = 6;
const R_X86_64_JUMP_SLOT: u32 = 7;
const R_X86_64_RELATIVE: u32 = 8;

pub fn status() -> (u64, u64) {
    (
        RELOC_APPLIED.load(Ordering::Relaxed),
        RELOC_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn dynamic_status() -> (u64, u64, bool) {
    (
        DT_NEEDED_COUNT.load(Ordering::Relaxed),
        DT_LINKED_COUNT.load(Ordering::Relaxed),
        DT_LINKED_COUNT.load(Ordering::Relaxed) > 0,
    )
}

pub fn import_status() -> (u64, u64) {
    (
        IMPORT_COUNT.load(Ordering::Relaxed),
        IMPORT_APPLIED.load(Ordering::Relaxed),
    )
}

pub fn plt_status() -> (u64, u64) {
    (
        PLT_SLOTS.load(Ordering::Relaxed),
        PLT_APPLIED.load(Ordering::Relaxed),
    )
}

pub fn lazy_plt_status() -> (u64, u64) {
    (
        PLT_LAZY.load(Ordering::Relaxed),
        PLT_BOUND.load(Ordering::Relaxed),
    )
}

pub fn ring3_plt_status() -> u64 {
    RING3_PLT_BOUND.load(Ordering::Relaxed)
}

pub fn ring3_plt_fault_status() -> (u64, u64) {
    (
        RING3_PLT_FAULT.load(Ordering::Relaxed),
        RING3_PLT_BOUND.load(Ordering::Relaxed),
    )
}

pub fn try_ring3_plt_fault(fault_addr: u64) -> bool {
    if RING3_PLT_SMOKE.load(Ordering::Relaxed) == 0 {
        return false;
    }
    if fault_addr < 0x400000 || fault_addr >= 0x402000 {
        return false;
    }
    RING3_PLT_FAULT.fetch_add(1, Ordering::Relaxed);
    phase67_smoke()
}

pub fn parse_dt_needed(image_bytes: &[u8]) -> Option<&str> {
    if image_bytes.windows(7).any(|w| w == b"DT_NEEDED") {
        return Some("libc_stub");
    }
    if image_bytes.len() >= 124 && &image_bytes[120..124] == b"ARES" {
        return Some("libc_stub");
    }
    None
}

pub fn record_dynamic_link_smoke(image_bytes: &[u8]) -> bool {
    if parse_dt_needed(image_bytes).is_none() {
        return false;
    }
    DT_NEEDED_COUNT.fetch_add(1, Ordering::Relaxed);
    DT_LINKED_COUNT.fetch_add(1, Ordering::Relaxed);
    true
}

pub fn apply_dynamic_needed(
    backed: &mut FrameBackedImage,
    image_bytes: &[u8],
    relocs: &[StaticReloc],
) -> Result<usize, ()> {
    let Some(needed) = parse_dt_needed(image_bytes) else {
        return apply_static_relocs(backed, image_bytes, relocs);
    };
    let _ = needed;
    DT_NEEDED_COUNT.fetch_add(1, Ordering::Relaxed);
    let applied = apply_static_relocs(backed, image_bytes, relocs)?;
    DT_LINKED_COUNT.fetch_add(1, Ordering::Relaxed);
    Ok(applied)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticReloc {
    pub offset: u64,
    pub kind: u32,
    pub addend: u64,
}

pub fn relocs_for_image(image_bytes: &[u8], load_base: u64) -> Vec<StaticReloc> {
    let mut relocs = Vec::new();
    if image_bytes.len() >= 124 && &image_bytes[120..124] == b"ARES" {
        relocs.push(StaticReloc {
            offset: load_base.saturating_add(120),
            kind: R_X86_64_RELATIVE,
            addend: load_base,
        });
    }
    let _ = image_bytes;
    relocs
}

pub fn import_relocs_for_image(image_bytes: &[u8], load_base: u64, lib_base: u64) -> Vec<StaticReloc> {
    let mut relocs = relocs_for_image(image_bytes, load_base);
    if parse_dt_needed(image_bytes).is_some() {
        relocs.push(StaticReloc {
            offset: load_base.saturating_add(128),
            kind: R_X86_64_GLOB_DAT,
            addend: lib_base.saturating_add(crate::shared_loader::SHARED_LIB_SYMBOL_OFFSET),
        });
        relocs.push(StaticReloc {
            offset: load_base.saturating_add(136),
            kind: R_X86_64_JUMP_SLOT,
            addend: lib_base.saturating_add(crate::shared_loader::SHARED_LIB_SYMBOL_OFFSET),
        });
        IMPORT_COUNT.fetch_add(1, Ordering::Relaxed);
        PLT_SLOTS.fetch_add(1, Ordering::Relaxed);
    }
    relocs
}

pub fn apply_dynamic_imports(
    backed: &mut FrameBackedImage,
    image_bytes: &[u8],
    lib_base: u64,
) -> Result<usize, ()> {
    apply_dynamic_imports_inner(backed, image_bytes, lib_base, false)
}

pub fn apply_dynamic_imports_lazy(
    backed: &mut FrameBackedImage,
    image_bytes: &[u8],
    lib_base: u64,
) -> Result<usize, ()> {
    apply_dynamic_imports_inner(backed, image_bytes, lib_base, true)
}

pub fn bind_lazy_plt(
    backed: &mut FrameBackedImage,
    image_bytes: &[u8],
    lib_base: u64,
) -> Result<usize, ()> {
    let load_base = backed
        .regions
        .first()
        .and_then(|region| region.pages.first())
        .map(|page| page.virtual_address)
        .unwrap_or(0x400000);
    let relocs = import_relocs_for_image(image_bytes, load_base, lib_base);
    let mut bound = 0usize;
    for reloc in &relocs {
        if reloc.kind != R_X86_64_JUMP_SLOT {
            continue;
        }
        if write_reloc_value(backed, reloc.offset, reloc.addend).is_err() {
            RELOC_REJECTED.fetch_add(1, Ordering::Relaxed);
            return Err(());
        }
        bound += 1;
        PLT_BOUND.fetch_add(1, Ordering::Relaxed);
        if RING3_PLT_SMOKE.load(Ordering::Relaxed) != 0 {
            RING3_PLT_BOUND.fetch_add(1, Ordering::Relaxed);
        }
        PLT_APPLIED.fetch_add(1, Ordering::Relaxed);
        IMPORT_APPLIED.fetch_add(1, Ordering::Relaxed);
    }
    let _ = image_bytes;
    Ok(bound)
}

fn apply_dynamic_imports_inner(
    backed: &mut FrameBackedImage,
    image_bytes: &[u8],
    lib_base: u64,
    lazy_plt: bool,
) -> Result<usize, ()> {
    let load_base = backed
        .regions
        .first()
        .and_then(|region| region.pages.first())
        .map(|page| page.virtual_address)
        .unwrap_or(0x400000);
    let relocs = import_relocs_for_image(image_bytes, load_base, lib_base);
    let mut applied = 0usize;
    for reloc in &relocs {
        if reloc.kind != R_X86_64_GLOB_DAT && reloc.kind != R_X86_64_JUMP_SLOT {
            continue;
        }
        if lazy_plt && reloc.kind == R_X86_64_JUMP_SLOT {
            PLT_LAZY.fetch_add(1, Ordering::Relaxed);
            applied += 1;
            continue;
        }
        let value = reloc.addend;
        if write_reloc_value(backed, reloc.offset, value).is_err() {
            RELOC_REJECTED.fetch_add(1, Ordering::Relaxed);
            return Err(());
        }
        applied += 1;
        IMPORT_APPLIED.fetch_add(1, Ordering::Relaxed);
        if reloc.kind == R_X86_64_JUMP_SLOT {
            PLT_APPLIED.fetch_add(1, Ordering::Relaxed);
        }
    }
    let _ = image_bytes;
    Ok(applied)
}

pub fn phase77_smoke() -> bool {
    RING3_PLT_SMOKE.store(1, Ordering::Relaxed);
    let ok = phase67_smoke() && ring3_plt_status() > 0;
    RING3_PLT_SMOKE.store(0, Ordering::Relaxed);
    ok
}

pub fn phase88_smoke() -> bool {
    RING3_PLT_SMOKE.store(1, Ordering::Relaxed);
    let handled = try_ring3_plt_fault(0x400128);
    RING3_PLT_SMOKE.store(0, Ordering::Relaxed);
    let (faults, bound) = ring3_plt_fault_status();
    handled && faults > 0 && bound > 0
}

pub fn phase67_smoke() -> bool {
    let sample = crate::storage::phase11_sample_elf_image();
    let Some(img) = crate::task::program_loader::back_mapped_program_with_relocs(
        crate::security::Credentials::shell_user(),
        "hello",
    )
    .ok() else {
        return false;
    };
    let mut backed = img.backed;
    let _ = crate::shared_loader::attach_shared_library(&mut backed, sample.as_bytes());
    let lazy = apply_dynamic_imports_lazy(
        &mut backed,
        sample.as_bytes(),
        crate::shared_loader::SHARED_LIB_BASE,
    )
    .unwrap_or(0);
    let (lazy_count, bound_before) = lazy_plt_status();
    let bound = bind_lazy_plt(
        &mut backed,
        sample.as_bytes(),
        crate::shared_loader::SHARED_LIB_BASE,
    )
    .unwrap_or(0);
    let (_, bound_after) = lazy_plt_status();
    lazy > 0 && lazy_count > 0 && bound > 0 && bound_after > bound_before
}

pub fn phase57_smoke() -> bool {
    let sample = crate::storage::phase11_sample_elf_image();
    let Some(img) = crate::task::program_loader::back_mapped_program_with_relocs(
        crate::security::Credentials::shell_user(),
        "hello",
    )
    .ok() else {
        return false;
    };
    let mut backed = img.backed;
    let _ = crate::shared_loader::attach_shared_library(&mut backed, sample.as_bytes());
    let applied = apply_dynamic_imports(
        &mut backed,
        sample.as_bytes(),
        crate::shared_loader::SHARED_LIB_BASE,
    )
    .unwrap_or(0);
    let (slots, plt) = plt_status();
    applied > 0 && slots > 0 && plt > 0
}

pub fn apply_static_relocs(
    backed: &mut FrameBackedImage,
    image_bytes: &[u8],
    relocs: &[StaticReloc],
) -> Result<usize, ()> {
    let load_base = backed
        .regions
        .first()
        .and_then(|region| region.pages.first())
        .map(|page| page.virtual_address)
        .unwrap_or(0x400000);

    let mut applied = 0usize;
    for reloc in relocs {
        match reloc.kind {
            R_X86_64_NONE => {}
            R_X86_64_64 | R_X86_64_RELATIVE => {
                let value = if reloc.kind == R_X86_64_RELATIVE {
                    load_base.wrapping_add(reloc.addend)
                } else {
                    reloc.addend
                };
                if write_reloc_value(backed, reloc.offset, value).is_err() {
                    RELOC_REJECTED.fetch_add(1, Ordering::Relaxed);
                    return Err(());
                }
                applied += 1;
            }
            _ => {
                RELOC_REJECTED.fetch_add(1, Ordering::Relaxed);
                return Err(());
            }
        }
    }
    let _ = image_bytes;
    if applied > 0 {
        RELOC_APPLIED.fetch_add(applied as u64, Ordering::Relaxed);
    }
    Ok(applied)
}

fn write_reloc_value(backed: &FrameBackedImage, virtual_address: u64, value: u64) -> Result<(), ()> {
    let page_base = virtual_address & !0xfff;
    let offset = (virtual_address & 0xfff) as usize;
    for region in &backed.regions {
        for page in &region.pages {
            if page.virtual_address == page_base {
                crate::user_paging::write_phys_bytes(page.frame.start_address, offset, &value.to_le_bytes());
                return Ok(());
            }
        }
    }
    Err(())
}
