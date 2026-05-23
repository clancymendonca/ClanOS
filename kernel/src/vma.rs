//! Per-process virtual memory area registry (Phase 63).

use core::sync::atomic::{AtomicU64, Ordering};

use crate::task::process::{self, ProcessId};

static REGIONS_REGISTERED: AtomicU64 = AtomicU64::new(0);
static OVERLAPS_REJECTED: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmaBacking {
    Anon,
    File,
    Image,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmaRegion {
    pub base: u64,
    pub len: u64,
    pub prot: u64,
    pub backing: VmaBacking,
}

pub fn status() -> (u64, u64) {
    (
        REGIONS_REGISTERED.load(Ordering::Relaxed),
        OVERLAPS_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn overlaps(pid: ProcessId, base: u64, len: u64) -> bool {
    let end = base.saturating_add(len);
    process::with_process_mut(pid, |process| {
        process.vma_regions().iter().any(|region| {
            let region_end = region.base.saturating_add(region.len);
            base < region_end && end > region.base
        })
    })
    .unwrap_or(false)
}

pub fn register_region(pid: ProcessId, region: VmaRegion) -> Result<(), ()> {
    if overlaps(pid, region.base, region.len) {
        OVERLAPS_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    match process::with_process_mut(pid, |process| {
        process.vma_regions_mut().push(region);
        REGIONS_REGISTERED.fetch_add(1, Ordering::Relaxed);
    }) {
        Some(()) => Ok(()),
        None => Err(()),
    }
}

pub fn unregister_region(pid: ProcessId, base: u64) -> bool {
    process::with_process_mut(pid, |process| {
        let regions = process.vma_regions_mut();
        if let Some(idx) = regions.iter().position(|region| region.base == base) {
            regions.remove(idx);
            return true;
        }
        false
    })
    .unwrap_or(false)
}

pub fn next_anon_hint(pid: ProcessId) -> u64 {
    let mut hint = crate::mmap::MMAP_ANON_BASE;
    if let Some(regions) = process::with_process_mut(pid, |p| p.vma_regions().to_vec()) {
        for region in regions {
            if region.backing == VmaBacking::Anon {
                let end = region.base.saturating_add(region.len);
                if end > hint {
                    hint = end;
                }
            }
        }
    }
    if hint >= crate::mmap::MMAP_ANON_LIMIT {
        hint = crate::mmap::MMAP_ANON_BASE;
    }
    hint
}

pub fn phase63_smoke() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = process::create_kernel_process_as("vma-smoke", tick, creds) else {
        return false;
    };
    let first = VmaRegion {
        base: 0x600000,
        len: 0x1000,
        prot: 2,
        backing: VmaBacking::Anon,
    };
    let second = VmaRegion {
        base: 0x601000,
        len: 0x1000,
        prot: 2,
        backing: VmaBacking::Anon,
    };
    let ok_first = register_region(pid, first).is_ok();
    let ok_second = register_region(pid, second).is_ok();
    let overlap = register_region(
        pid,
        VmaRegion {
            base: 0x600800,
            len: 0x1000,
            prot: 2,
            backing: VmaBacking::Anon,
        },
    )
    .is_err();
    let (regions, rejected) = status();
    ok_first && ok_second && overlap && regions >= 2 && rejected > 0
}
