//! Bump allocator for ring-3 programs (static `.bss` region).

#![allow(unsafe_code)]

use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

/// Static heap size — kept small so release ELFs fit `exec_image::MAX_IMAGE_SIZE`.
pub const HEAP_SIZE: usize = 8192;

static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static HEAP_NEXT: AtomicUsize = AtomicUsize::new(0);

struct BumpAllocator;

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align().max(1);
        let size = layout.size();
        loop {
            let current = HEAP_NEXT.load(Ordering::Relaxed);
            let aligned = (current + align - 1) & !(align - 1);
            let end = aligned.saturating_add(size);
            if end > HEAP_SIZE {
                return core::ptr::null_mut();
            }
            if HEAP_NEXT
                .compare_exchange(current, end, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                let base = core::ptr::addr_of_mut!(HEAP) as *mut u8;
                return base.add(aligned);
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator — no individual free until `heap_reset`.
    }
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator;

pub fn heap_reset() {
    HEAP_NEXT.store(0, Ordering::Relaxed);
}

pub fn heap_used() -> usize {
    HEAP_NEXT.load(Ordering::Relaxed)
}
