//! Sector cache in front of the active block device .

use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

use crate::storage::SECTOR_SIZE;

const CACHE_SLOTS: usize = 32;

#[derive(Clone, Copy)]
struct CacheEntry {
    sector: usize,
    data: [u8; SECTOR_SIZE],
    valid: bool,
}

impl CacheEntry {
    const fn empty() -> Self {
        Self {
            sector: usize::MAX,
            data: [0; SECTOR_SIZE],
            valid: false,
        }
    }
}

struct SectorCache {
    entries: [CacheEntry; CACHE_SLOTS],
    cursor: usize,
}

impl SectorCache {
    const fn new() -> Self {
        Self {
            entries: [CacheEntry::empty(); CACHE_SLOTS],
            cursor: 0,
        }
    }

    fn lookup(&self, sector: usize) -> Option<[u8; SECTOR_SIZE]> {
        for entry in &self.entries {
            if entry.valid && entry.sector == sector {
                return Some(entry.data);
            }
        }
        None
    }

    fn insert(&mut self, sector: usize, data: [u8; SECTOR_SIZE]) {
        let idx = self.cursor % CACHE_SLOTS;
        self.cursor = self.cursor.wrapping_add(1);
        self.entries[idx] = CacheEntry {
            sector,
            data,
            valid: true,
        };
    }

    fn invalidate(&mut self, sector: usize) {
        for entry in &mut self.entries {
            if entry.valid && entry.sector == sector {
                entry.valid = false;
            }
        }
    }
}

static HITS: AtomicU64 = AtomicU64::new(0);
static MISSES: AtomicU64 = AtomicU64::new(0);

lazy_static::lazy_static! {
    static ref CACHE: Mutex<SectorCache> = Mutex::new(SectorCache::new());
}

pub fn status() -> (u64, u64) {
    (
        HITS.load(Ordering::Relaxed),
        MISSES.load(Ordering::Relaxed),
    )
}

pub fn read_sector(
    sector: usize,
    buffer: &mut [u8; SECTOR_SIZE],
    raw_read: impl FnOnce(usize, &mut [u8; SECTOR_SIZE]) -> Result<(), crate::block::BlockError>,
) -> Result<(), crate::block::BlockError> {
    if let Some(data) = CACHE.lock().lookup(sector) {
        HITS.fetch_add(1, Ordering::Relaxed);
        buffer.copy_from_slice(&data);
        return Ok(());
    }
    MISSES.fetch_add(1, Ordering::Relaxed);
    raw_read(sector, buffer)?;
    CACHE.lock().insert(sector, *buffer);
    Ok(())
}

pub fn write_sector(
    sector: usize,
    buffer: &[u8; SECTOR_SIZE],
    raw_write: impl FnOnce(usize, &[u8; SECTOR_SIZE]) -> Result<(), crate::block::BlockError>,
) -> Result<(), crate::block::BlockError> {
    CACHE.lock().invalidate(sector);
    raw_write(sector, buffer)
}

pub fn smoke_block_cache() -> bool {
    let before = status();
    let mut buf = [0u8; SECTOR_SIZE];
    buf[0] = 0xBC;
    if crate::block::write_active_sector(3, &buf).is_err() {
        return false;
    }
    let mut read = [0u8; SECTOR_SIZE];
    if crate::block::read_active_sector(3, &mut read).is_err() {
        return false;
    }
    if crate::block::read_active_sector(3, &mut read).is_err() {
        return false;
    }
    let after = status();
    read[0] == 0xBC && after.0 > before.0 && after.1 >= before.1
}
