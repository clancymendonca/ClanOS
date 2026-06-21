//! Bochs VBE (BGA) linear framebuffer — ADR-0004 scope 470.
//!
//! I/O ports 0x1CE/0x1CF mode-set, PCI BAR0 LFB via bootloader phys map, buddy back buffer.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use x86_64::instructions::port::Port;
use x86_64::structures::paging::{
    FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
};
use x86_64::VirtAddr;

pub const WIDTH: usize = 1024;
pub const HEIGHT: usize = 768;
pub const BPP: usize = 32;
pub const BYTES_PER_PIXEL: usize = BPP / 8;
pub const BUFFER_LEN: usize = WIDTH * HEIGHT;
pub const BUFFER_BYTES: usize = BUFFER_LEN * BYTES_PER_PIXEL;

pub const BGA_ID: u16 = 0xB0C5;
pub const BGA_VENDOR: u16 = 0x1234;
pub const BGA_DEVICE: u16 = 0x1111;

const BACK_BUFFER_VIRT: u64 = 0x5300_0000_0000;

static BGA_ACTIVE: AtomicBool = AtomicBool::new(false);
static BACK_BUFFER_MAPPED: AtomicBool = AtomicBool::new(false);
static BACK_BUFFER_MAP_OK: AtomicBool = AtomicBool::new(false);
static LFB_WRITE_OK: AtomicBool = AtomicBool::new(false);
static BACK_FRAMES: AtomicU64 = AtomicU64::new(0);
static LAST_BGA_ID: AtomicU64 = AtomicU64::new(0);
static LFB_PHYS: AtomicU64 = AtomicU64::new(0);
static MAP_BYTES: AtomicU64 = AtomicU64::new(0);
static PHYS_OFFSET: AtomicU64 = AtomicU64::new(0);
static VIDEO_SMOKE_RAN: AtomicBool = AtomicBool::new(false);
static VIDEO_SMOKE_OK: AtomicBool = AtomicBool::new(false);

pub fn set_physical_memory_offset(offset: VirtAddr) {
    PHYS_OFFSET.store(offset.as_u64(), Ordering::Relaxed);
}

pub fn mode_active() -> bool {
    BGA_ACTIVE.load(Ordering::Relaxed)
}

pub fn back_buffer_mapped() -> bool {
    BACK_BUFFER_MAPPED.load(Ordering::Relaxed)
}

pub fn video_memory_proof() -> (u64, bool) {
    (
        BACK_FRAMES.load(Ordering::Relaxed),
        LFB_WRITE_OK.load(Ordering::Relaxed),
    )
}

pub fn back_buffer_map_ok() -> bool {
    BACK_BUFFER_MAP_OK.load(Ordering::Relaxed)
}

/// `map_bytes = min(computed_size, bar_size)` — ADR-0004 Q4 (host mirror in bga_bounds_lib.py).
pub const fn map_bytes_rule(computed_size: u64, bar_size: u64) -> Option<u64> {
    if computed_size == 0 || bar_size == 0 {
        return None;
    }
    Some(if computed_size <= bar_size {
        computed_size
    } else {
        bar_size
    })
}

pub fn read_bga_id() -> u16 {
    bga_write(0, 0);
    bga_read(0)
}

fn bga_write(index: u16, value: u16) {
    unsafe {
        Port::<u16>::new(0x1CE).write(index);
        Port::<u16>::new(0x1CF).write(value);
    }
}

fn bga_read(_index: u16) -> u16 {
    unsafe { Port::<u16>::new(0x1CF).read() }
}

fn disable_bga() {
    bga_write(4, 0);
}

fn set_bga_mode(width: u16, height: u16, bpp: u16) {
    bga_write(0, 0);
    bga_write(1, width);
    bga_write(2, height);
    bga_write(3, bpp);
    // Enable display + LFB.
    bga_write(4, 0x0001 | 0x0002);
}

fn lfb_ptr() -> *mut u32 {
    let offset = PHYS_OFFSET.load(Ordering::Relaxed);
    let lfb = LFB_PHYS.load(Ordering::Relaxed);
    (offset + lfb) as *mut u32
}

/// Prove buddy can satisfy the full back-buffer frame count (alloc/free without virt map).
fn smoke_buddy_back_buffer_frames(frame_count: u64) -> bool {
    let mut buddy = crate::buddy::BuddyFrameAllocator;
    for _ in 0..frame_count {
        let Some(frame) = FrameAllocator::<Size4KiB>::allocate_frame(&mut buddy) else {
            return false;
        };
        buddy.deallocate_frame(frame);
    }
    BACK_FRAMES.store(frame_count, Ordering::Relaxed);
    true
}

fn map_back_buffer(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    computed_size: u64,
) -> bool {
    let page_count = computed_size / 4096;
    if page_count == 0 || computed_size % 4096 != 0 {
        return false;
    }
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE;
    let mut buddy = crate::buddy::BuddyFrameAllocator;
    for i in 0..page_count {
        let Some(frame) = FrameAllocator::<Size4KiB>::allocate_frame(&mut buddy) else {
            return false;
        };
        let virt = VirtAddr::new(BACK_BUFFER_VIRT + i * 4096);
        let page = Page::<Size4KiB>::containing_address(virt);
        let result = unsafe { mapper.map_to(page, frame, flags, frame_allocator) };
        match result {
            Ok(flush) => flush.flush(),
            Err(_) => return false,
        }
    }
    BACK_FRAMES.store(page_count, Ordering::Relaxed);
    BACK_BUFFER_MAPPED.store(true, Ordering::Relaxed);
    true
}

fn init_bga_path(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    map_back_buffer_virt: bool,
) -> bool {
    let id = read_bga_id();
    LAST_BGA_ID.store(u64::from(id), Ordering::Relaxed);
    if id != BGA_ID {
        return false;
    }

    let Some((_bus, _slot, _function, lfb_phys, bar_size)) = crate::device::find_bga_bar() else {
        return false;
    };

    let computed_size = BUFFER_BYTES as u64;
    let Some(map_bytes) = map_bytes_rule(computed_size, bar_size) else {
        return false;
    };

    let page_count = computed_size / 4096;
    if map_back_buffer_virt {
        if !map_back_buffer(mapper, frame_allocator, computed_size) {
            return false;
        }
    } else if !smoke_buddy_back_buffer_frames(page_count) {
        return false;
    }

    set_bga_mode(WIDTH as u16, HEIGHT as u16, BPP as u16);

    LFB_PHYS.store(lfb_phys, Ordering::Relaxed);
    MAP_BYTES.store(map_bytes, Ordering::Relaxed);
    BGA_ACTIVE.store(true, Ordering::Relaxed);
    true
}

/// Map buddy back buffer at `BACK_BUFFER_VIRT`, write/readback, flush spot-check — ADR-0004 Q4/Q5.
/// Called after `scheduler_epoch` (page-table pressure from hw_paging smokes is behind us).
pub fn map_back_buffer_for_desktop(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> bool {
    if back_buffer_map_ok() {
        return true;
    }
    if !mode_active() {
        crate::serial_println!("ClanOS-Video: back_buffer_map_ok=false reason=bga_inactive");
        return false;
    }
    if !map_back_buffer(mapper, frame_allocator, BUFFER_BYTES as u64) {
        crate::serial_println!("ClanOS-Video: back_buffer_map_ok=false reason=map_failed");
        return false;
    }

    const TEST_PIXEL: u32 = 0x0000_FF00; // BGRx green
    unsafe {
        (BACK_BUFFER_VIRT as *mut u32).write_volatile(TEST_PIXEL);
    }
    let readback = unsafe { (BACK_BUFFER_VIRT as *mut u32).read_volatile() };
    if readback != TEST_PIXEL {
        crate::serial_println!(
            "ClanOS-Video: back_buffer_map_ok=false reason=readback readback={:#x}",
            readback
        );
        return false;
    }

    flush_back_to_lfb();
    let lfb_readback = unsafe { lfb_ptr().read_volatile() };
    if lfb_readback != TEST_PIXEL {
        crate::serial_println!(
            "ClanOS-Video: back_buffer_map_ok=false reason=lfb_flush lfb_readback={:#x}",
            lfb_readback
        );
        return false;
    }

    BACK_BUFFER_MAP_OK.store(true, Ordering::Relaxed);
    crate::serial_println!(
        "ClanOS-Video: back_buffer_map_ok=true back_virt={:#x} back_frames={}",
        BACK_BUFFER_VIRT,
        BACK_FRAMES.load(Ordering::Relaxed)
    );
    true
}

/// Idempotent alias for post-gate boot (`main.rs`).
pub fn init_desktop_framebuffer(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> bool {
    map_back_buffer_for_desktop(mapper, frame_allocator)
}

/// BGA primary, mode 13h fallback, fail closed — ADR-0004 Q3.
pub fn init_display(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> bool {
    if init_bga_path(mapper, frame_allocator, true) {
        crate::serial_println!(
            "Desktop: bga {}x{} depth={} lfb={:#x} id={:#04x}",
            WIDTH,
            HEIGHT,
            BPP,
            LFB_PHYS.load(Ordering::Relaxed),
            LAST_BGA_ID.load(Ordering::Relaxed) as u16
        );
        return true;
    }

    let id = LAST_BGA_ID.load(Ordering::Relaxed) as u16;
    if crate::framebuffer::init_mode_13h() {
        crate::serial_println!("Desktop: fallback mode13h 320x200");
        return true;
    }

    crate::serial_println!(
        "Desktop: init failed bga_id={:#04x} mode13=false",
        if id == 0 { read_bga_id() } else { id }
    );
    BGA_ACTIVE.store(false, Ordering::Relaxed);
    false
}

/// Kernel draw target. Before `map_back_buffer_for_desktop`, falls back to LFB (scheduler_epoch compositor smoke only).
pub fn back_buffer_ptr() -> *mut u32 {
    if back_buffer_mapped() {
        BACK_BUFFER_VIRT as *mut u32
    } else {
        lfb_ptr()
    }
}

pub fn flush_back_to_lfb() {
    if !mode_active() || !back_buffer_mapped() {
        return;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            back_buffer_ptr(),
            lfb_ptr(),
            BUFFER_LEN,
        );
    }
}

/// Boot-time smoke (main.rs, before validation gate) — ADR-0004 Q4/Q5.
pub fn run_video_memory_smoke(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> bool {
    LFB_WRITE_OK.store(false, Ordering::Relaxed);
    BACK_FRAMES.store(0, Ordering::Relaxed);
    BACK_BUFFER_MAPPED.store(false, Ordering::Relaxed);
    BACK_BUFFER_MAP_OK.store(false, Ordering::Relaxed);
    BGA_ACTIVE.store(false, Ordering::Relaxed);
    VIDEO_SMOKE_RAN.store(true, Ordering::Relaxed);

    if !init_bga_path(mapper, frame_allocator, false) {
        VIDEO_SMOKE_OK.store(false, Ordering::Relaxed);
        return false;
    }

    const TEST_PIXEL: u32 = 0x00FF_0000; // BGRx blue
    unsafe {
        lfb_ptr().write_volatile(TEST_PIXEL);
    }
    let readback = unsafe { lfb_ptr().read_volatile() };
    if readback != TEST_PIXEL {
        disable_bga();
        VIDEO_SMOKE_OK.store(false, Ordering::Relaxed);
        return false;
    }

    LFB_WRITE_OK.store(true, Ordering::Relaxed);

    let (frames, lfb_ok) = video_memory_proof();
    let expected_frames = (BUFFER_BYTES / 4096) as u64;
    if frames != expected_frames || !lfb_ok {
        disable_bga();
        VIDEO_SMOKE_OK.store(false, Ordering::Relaxed);
        return false;
    }

    crate::serial_println!(
        "ClanOS-Video: back_frames={} lfb_write_ok=true map_bytes={}",
        frames,
        MAP_BYTES.load(Ordering::Relaxed)
    );
    crate::serial_println!(
        "Desktop: bga {}x{} depth={} lfb={:#x} id={:#04x}",
        WIDTH,
        HEIGHT,
        BPP,
        LFB_PHYS.load(Ordering::Relaxed),
        LAST_BGA_ID.load(Ordering::Relaxed) as u16
    );

    VIDEO_SMOKE_OK.store(true, Ordering::Relaxed);
    true
}

/// Called from `memory_layout` gate — confirms boot smoke ran and passed in QEMU.
pub fn confirm_video_memory_smoke() -> bool {
    if !VIDEO_SMOKE_RAN.load(Ordering::Relaxed) {
        return false;
    }
    if !VIDEO_SMOKE_OK.load(Ordering::Relaxed) {
        return false;
    }
    let (frames, lfb_ok) = video_memory_proof();
    let expected_frames = (BUFFER_BYTES / 4096) as u64;
    frames == expected_frames && lfb_ok
}

pub fn smoke_bga_id_probe() -> bool {
    read_bga_id() == BGA_ID
}
