//! Desktop shell service (scope 354) — taskbar + launcher strip (1024×768 RGB).

use core::sync::atomic::{AtomicU64, Ordering};

use crate::framebuffer::{self, Pixel};

const TASKBAR_H: usize = 54;
const LAUNCHER_W: usize = 144;

static SHELL_FRAMES: AtomicU64 = AtomicU64::new(0);

pub fn taskbar_height() -> usize {
    TASKBAR_H
}

pub fn render_taskbar(back: &mut [Pixel]) {
    let y = framebuffer::HEIGHT.saturating_sub(TASKBAR_H);
    framebuffer::fill_rect_buf(back, 0, y, framebuffer::WIDTH, TASKBAR_H, framebuffer::COLOR_PANEL);
    framebuffer::draw_text_buf(back, 24, y + 15, "CLANOS SHELL", framebuffer::COLOR_TEXT);

    let mut x = LAUNCHER_W;
    for label in ["RUN", "LS", "PS"] {
        framebuffer::fill_rect_buf(back, x, y + 9, 108, 36, framebuffer::COLOR_TITLEBAR);
        framebuffer::draw_text_buf(back, x + 12, y + 15, label, framebuffer::COLOR_TEXT);
        x += 120;
    }
}

pub fn render_launcher(back: &mut [Pixel]) {
    let y = framebuffer::HEIGHT.saturating_sub(TASKBAR_H);
    framebuffer::fill_rect_buf(
        back,
        0,
        y,
        LAUNCHER_W,
        TASKBAR_H,
        framebuffer::COLOR_TITLEBAR,
    );
    framebuffer::draw_text_buf(back, 18, y + 15, "MENU", framebuffer::COLOR_TEXT);
}

pub fn bootstrap_windows() {
    if crate::window_manager::window_count() == 0 {
        let _ = crate::window_manager::create_window(72, 84, 420, 240, "CONSOLE");
        let _ = crate::window_manager::create_window(540, 108, 360, 216, "FILES");
    }
}

pub fn render_shell(back: &mut [Pixel]) {
    bootstrap_windows();
    render_launcher(back);
    render_taskbar(back);
    crate::window_manager::draw_windows(back);
    SHELL_FRAMES.fetch_add(1, Ordering::Relaxed);
}

pub fn frame_count() -> u64 {
    SHELL_FRAMES.load(Ordering::Relaxed)
}

pub fn smoke_desktop_shell() -> bool {
    bootstrap_windows();
    let before = frame_count();
    framebuffer::back_buffer_mut(|back| render_shell(back));
    crate::window_manager::window_count() >= 2 && frame_count() > before
}

pub fn smoke_desktop_integration() -> bool {
    crate::mouse::smoke_mouse()
        && crate::framebuffer::smoke_double_buffer()
        && smoke_desktop_shell()
        && crate::window_manager::smoke_wm_smoke()
        && crate::compositor::smoke_compositor_desktop()
}
