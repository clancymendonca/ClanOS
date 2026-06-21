//! Compositor IPC stub (scope 145) — ABI_COMPOSITOR_IPC minimum contract.
//! : pixel frame submission via Bochs VBE RGB framebuffer (ADR-0004).

use core::sync::atomic::{AtomicU64, Ordering};

pub const COMPOSITOR_SCHEMA: &str = "compositor.ipc.v1";

static FRAME_SUBMITS: AtomicU64 = AtomicU64::new(0);
static UNKNOWN_CAPS_IGNORED: AtomicU64 = AtomicU64::new(0);
static PIXEL_FRAMES: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompositorCaps {
    pub a11y_version: u16,
    pub flags: u32,
}

pub fn submit_frame(caps: CompositorCaps) -> bool {
    FRAME_SUBMITS.fetch_add(1, Ordering::Relaxed);
    if caps.flags & 0x8000_0000 != 0 {
        UNKNOWN_CAPS_IGNORED.fetch_add(1, Ordering::Relaxed);
    }
    if caps.flags & 0x01 != 0 && crate::framebuffer::init_display() {
        crate::framebuffer::render_desktop_frame();
        PIXEL_FRAMES.fetch_add(1, Ordering::Relaxed);
    }
    true
}

pub fn pixel_frame_count() -> u64 {
    PIXEL_FRAMES.load(Ordering::Relaxed)
}

pub fn smoke_compositor() -> bool {
    let ok_known = submit_frame(CompositorCaps {
        a11y_version: 1,
        flags: 0x01,
    });
    let ok_unknown = submit_frame(CompositorCaps {
        a11y_version: 1,
        flags: 0x8000_0001,
    });
    ok_known
        && ok_unknown
        && FRAME_SUBMITS.load(Ordering::Relaxed) >= 2
        && pixel_frame_count() >= 1
        && crate::framebuffer::mode_active()
}

pub fn smoke_compositor_desktop() -> bool {
    smoke_compositor() && crate::framebuffer::mode_active()
}
