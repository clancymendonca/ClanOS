//! Desktop shell service (phase 354) — taskbar + launcher strip.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::framebuffer;
use crate::window_manager;

const TASKBAR_H: usize = 18;
const LAUNCHER_W: usize = 48;

static SHELL_FRAMES: AtomicU64 = AtomicU64::new(0);

pub fn taskbar_height() -> usize {
    TASKBAR_H
}

pub fn render_taskbar(back: &mut [u8]) {
    let y = framebuffer::HEIGHT.saturating_sub(TASKBAR_H);
    framebuffer::fill_rect_buf(back, 0, y, framebuffer::WIDTH, TASKBAR_H, 8);
    framebuffer::draw_text_buf(back, 8, y + 5, "AREOS SHELL", framebuffer::COLOR_TEXT);

    let mut x = LAUNCHER_W;
    for label in ["RUN", "LS", "PS"] {
        framebuffer::fill_rect_buf(back, x, y + 3, 36, 12, framebuffer::COLOR_TITLEBAR);
        framebuffer::draw_text_buf(back, x + 4, y + 5, label, framebuffer::COLOR_TEXT);
        x += 40;
    }
}

pub fn render_launcher(back: &mut [u8]) {
    let y = framebuffer::HEIGHT.saturating_sub(TASKBAR_H);
    framebuffer::fill_rect_buf(
        back,
        0,
        y,
        LAUNCHER_W,
        TASKBAR_H,
        framebuffer::COLOR_TITLEBAR,
    );
    framebuffer::draw_text_buf(back, 6, y + 5, "MENU", framebuffer::COLOR_TEXT);
}

pub fn bootstrap_windows() {
    if window_manager::window_count() == 0 {
        let _ = window_manager::create_window(24, 28, 140, 80, "CONSOLE");
        let _ = window_manager::create_window(180, 36, 120, 72, "FILES");
    }
}

pub fn render_shell(back: &mut [u8]) {
    bootstrap_windows();
    render_launcher(back);
    render_taskbar(back);
    window_manager::draw_windows(back);
    SHELL_FRAMES.fetch_add(1, Ordering::Relaxed);
}

pub fn frame_count() -> u64 {
    SHELL_FRAMES.load(Ordering::Relaxed)
}

pub fn phase354_shell_smoke() -> bool {
    bootstrap_windows();
    let before = frame_count();
    let mut back = [0u8; framebuffer::BUFFER_LEN];
    render_shell(&mut back);
    window_manager::window_count() >= 2 && frame_count() > before
}

pub fn phase375_desktop_smoke() -> bool {
    crate::mouse::phase352_mouse_smoke()
        && crate::framebuffer::phase353_double_buffer_smoke()
        && phase354_shell_smoke()
        && window_manager::phase356_wm_smoke()
        && crate::compositor::phase351_compositor_desktop_smoke()
}
