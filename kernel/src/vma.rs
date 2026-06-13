//! Per-process virtual memory area registry .

use core::sync::atomic::{AtomicU64, Ordering};

use crate::task::process::{self, ProcessId};

static REGIONS_REGISTERED: AtomicU64 = AtomicU64::new(0);
static OVERLAPS_REJECTED: AtomicU64 = AtomicU64::new(0);
static VMA_SPLITS: AtomicU64 = AtomicU64::new(0);
static VMA_COALESCED: AtomicU64 = AtomicU64::new(0);
static MMAP_GAPS_USED: AtomicU64 = AtomicU64::new(0);

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

pub fn split_status() -> (u64, u64) {
    (
        VMA_SPLITS.load(Ordering::Relaxed),
        REGIONS_REGISTERED.load(Ordering::Relaxed),
    )
}

pub fn coalesce_status() -> (u64, u64) {
    (
        VMA_COALESCED.load(Ordering::Relaxed),
        REGIONS_REGISTERED.load(Ordering::Relaxed),
    )
}

pub fn mmap_gap_status() -> u64 {
    MMAP_GAPS_USED.load(Ordering::Relaxed)
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

pub fn truncate_region(pid: ProcessId, base: u64, len: u64) -> bool {
    process::with_process_mut(pid, |process| {
        let regions = process.vma_regions_mut();
        if let Some(region) = regions.iter_mut().find(|region| region.base == base) {
            if region.len <= len {
                return false;
            }
            region.base = region.base.saturating_add(len);
            region.len = region.len.saturating_sub(len);
            return true;
        }
        if let Some(idx) = regions.iter().position(|region| {
            let region_end = region.base.saturating_add(region.len);
            let unmap_end = base.saturating_add(len);
            region.base < unmap_end && region_end > base
        }) {
            let region = regions.remove(idx);
            let tail_base = base.saturating_add(len);
            let tail_len = region
                .base
                .saturating_add(region.len)
                .saturating_sub(tail_base);
            if tail_len > 0 {
                regions.push(VmaRegion {
                    base: tail_base,
                    len: tail_len,
                    prot: region.prot,
                    backing: region.backing,
                });
            }
            let head_len = base.saturating_sub(region.base);
            if head_len > 0 {
                regions.push(VmaRegion {
                    base: region.base,
                    len: head_len,
                    prot: region.prot,
                    backing: region.backing,
                });
            }
            VMA_SPLITS.fetch_add(1, Ordering::Relaxed);
            return true;
        }
        false
    })
    .unwrap_or(false)
}

pub fn next_anon_hint(pid: ProcessId) -> u64 {
    let base = crate::mmap::MMAP_ANON_BASE;
    let limit = crate::mmap::MMAP_ANON_LIMIT;
    let mut regions: alloc::vec::Vec<VmaRegion> = process::with_process_mut(pid, |p| {
        p.vma_regions()
            .iter()
            .filter(|region| region.backing == VmaBacking::Anon)
            .cloned()
            .collect()
    })
    .unwrap_or_default();
    regions.sort_by_key(|region| region.base);
    let mut cursor = base;
    for region in &regions {
        if cursor.saturating_add(0x1000) <= region.base && region.base < limit {
            return cursor;
        }
        let end = region.base.saturating_add(region.len);
        if end > cursor {
            cursor = end;
        }
    }
    if cursor.saturating_add(0x1000) <= limit {
        cursor
    } else {
        base
    }
}

pub fn coalesce_adjacent(pid: ProcessId) -> u64 {
    let mut merged = 0u64;
    process::with_process_mut(pid, |process| {
        let regions = process.vma_regions_mut();
        regions.sort_by_key(|region| region.base);
        let mut idx = 0usize;
        while idx + 1 < regions.len() {
            let same = regions[idx].backing == regions[idx + 1].backing
                && regions[idx].prot == regions[idx + 1].prot
                && regions[idx].base.saturating_add(regions[idx].len) == regions[idx + 1].base;
            if same {
                let merged_len = regions[idx].len.saturating_add(regions[idx + 1].len);
                let base = regions[idx].base;
                let prot = regions[idx].prot;
                let backing = regions[idx].backing;
                regions[idx] = VmaRegion {
                    base,
                    len: merged_len,
                    prot,
                    backing,
                };
                regions.remove(idx + 1);
                merged = merged.saturating_add(1);
                continue;
            }
            idx += 1;
        }
    });
    if merged > 0 {
        VMA_COALESCED.fetch_add(merged, Ordering::Relaxed);
    }
    merged
}

pub fn smoke_vma_regions() -> bool {
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

pub fn smoke_vma_split() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = process::create_kernel_process_as("vma-split", tick, creds) else {
        return false;
    };
    process::set_smoke_process_id(Some(pid));
    let region = VmaRegion {
        base: 0x604000,
        len: 0x2000,
        prot: 2,
        backing: VmaBacking::Anon,
    };
    let registered = register_region(pid, region).is_ok();
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(creds, "hello").ok()
    else {
        return false;
    };
    let cr3 = built.hw.cr3_phys;
    let mapped = crate::user_paging::map_demand_zero_page(cr3, 0x604000).is_ok()
        && crate::user_paging::map_demand_zero_page(cr3, 0x605000).is_ok();
    let unmapped = crate::mmap::munmap_range(cr3, 0x605000, 0x1000).is_ok();
    let head_ok = process::with_process_mut(pid, |p| {
        p.vma_regions()
            .iter()
            .any(|region| region.base == 0x604000 && region.len == 0x1000)
    })
    .unwrap_or(false);
    process::set_smoke_process_id(None);
    let (splits, _) = split_status();
    registered && mapped && unmapped && head_ok && splits > 0
}

pub fn smoke_mmap_gap() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = process::create_kernel_process_as("mmap-gap", tick, creds) else {
        return false;
    };
    process::set_smoke_process_id(Some(pid));
    let low = VmaRegion {
        base: 0x600000,
        len: 0x1000,
        prot: 2,
        backing: VmaBacking::Anon,
    };
    let high = VmaRegion {
        base: 0x603000,
        len: 0x1000,
        prot: 2,
        backing: VmaBacking::Anon,
    };
    let _ = register_region(pid, low);
    let _ = register_region(pid, high);
    let hint = next_anon_hint(pid);
    let gap_used = hint == 0x601000;
    if gap_used {
        MMAP_GAPS_USED.fetch_add(1, Ordering::Relaxed);
    }
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(creds, "hello").ok()
    else {
        return false;
    };
    let cr3 = built.hw.cr3_phys;
    let mapped = crate::mmap::mmap_anonymous(cr3, 2, 0).ok() == Some(hint);
    let translated = crate::user_paging::translate_hw_page(cr3, hint).is_some();
    process::set_smoke_process_id(None);
    gap_used && mapped && translated && mmap_gap_status() > 0
}

pub fn smoke_vma_coalesce() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = process::create_kernel_process_as("vma-coalesce", tick, creds) else {
        return false;
    };
    process::set_smoke_process_id(Some(pid));
    let _ = register_region(
        pid,
        VmaRegion {
            base: 0x602000,
            len: 0x1000,
            prot: 2,
            backing: VmaBacking::Anon,
        },
    );
    let _ = register_region(
        pid,
        VmaRegion {
            base: 0x603000,
            len: 0x1000,
            prot: 2,
            backing: VmaBacking::Anon,
        },
    );
    let merged = coalesce_adjacent(pid);
    let single = process::with_process_mut(pid, |p| {
        p.vma_regions()
            .iter()
            .any(|region| region.base == 0x602000 && region.len == 0x2000)
    })
    .unwrap_or(false);
    process::set_smoke_process_id(None);
    merged > 0 && single && coalesce_status().0 > 0
}
