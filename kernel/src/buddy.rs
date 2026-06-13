//! Buddy physical frame allocator (order-0 / 4 KiB pages).
//!
//! Populated at boot from the bootloader memory map; coalesces buddies on free.

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};
use x86_64::PhysAddr;

static ALLOCATED: AtomicU64 = AtomicU64::new(0);
static FREED: AtomicU64 = AtomicU64::new(0);
static MERGES: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    static ref BUDDY: Mutex<BuddyAllocator> = Mutex::new(BuddyAllocator::empty());
}

pub fn status() -> (u64, u64, u64, usize) {
    let buddy = BUDDY.lock();
    (
        ALLOCATED.load(Ordering::Relaxed),
        FREED.load(Ordering::Relaxed),
        MERGES.load(Ordering::Relaxed),
        buddy.free_count(),
    )
}

pub fn init_from_memory_map(
    memory_map: &'static bootloader::bootinfo::MemoryMap,
    skip_frames: usize,
) {
    use bootloader::bootinfo::MemoryRegionType;

    let mut frames = Vec::new();
    for region in memory_map.iter() {
        if region.region_type != MemoryRegionType::Usable {
            continue;
        }
        let mut addr = region.range.start_addr();
        while addr < region.range.end_addr() {
            frames.push(PhysFrame::containing_address(PhysAddr::new(addr)));
            addr += 4096;
        }
    }
    for frame in frames.into_iter().skip(skip_frames) {
        BUDDY.lock().free_frame(frame);
    }
}

pub struct BuddyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for BuddyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = BUDDY.lock().allocate_frame()?;
        ALLOCATED.fetch_add(1, Ordering::Relaxed);
        Some(frame)
    }
}

impl BuddyFrameAllocator {
    pub fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        BUDDY.lock().free_frame(frame);
        FREED.fetch_add(1, Ordering::Relaxed);
    }
}

struct BuddyAllocator {
    free: Vec<u64>,
}

impl BuddyAllocator {
    const fn empty() -> Self {
        Self { free: Vec::new() }
    }

    fn free_count(&self) -> usize {
        self.free.len()
    }

    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let addr = self.free.pop()?;
        Some(PhysFrame::containing_address(PhysAddr::new(addr)))
    }

    fn free_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        let addr = frame.start_address().as_u64();
        if self.try_merge(addr) {
            MERGES.fetch_add(1, Ordering::Relaxed);
        } else {
            self.free.push(addr);
        }
    }

    fn try_merge(&mut self, addr: u64) -> bool {
        let buddy = addr ^ 4096;
        if let Some(index) = self.free.iter().position(|&candidate| candidate == buddy) {
            self.free.remove(index);
            self.free.push(addr.min(buddy));
            true
        } else {
            false
        }
    }
}

pub fn smoke_buddy_allocator() -> bool {
    let before = status();
    let mut allocator = BuddyFrameAllocator;
    let Some(frame) = allocator.allocate_frame() else {
        return before.3 > 0;
    };
    allocator.deallocate_frame(frame);
    let after = status();
    after.0 > before.0 && after.1 > before.1
}
