//! Post-351 milestones — desktop epoch 15 (375) and functional OS gate (400).

use core::sync::atomic::{AtomicU64, Ordering};

static DESKTOP_READY: AtomicU64 = AtomicU64::new(0);
static FUNCTIONAL_OS: AtomicU64 = AtomicU64::new(0);

pub fn phase352_mouse_smoke() -> bool {
    crate::mouse::phase352_mouse_smoke()
}

pub fn phase353_compositor_buffer_smoke() -> bool {
    crate::framebuffer::phase353_double_buffer_smoke()
        && crate::window_manager::phase353_window_smoke()
}

pub fn phase354_shell_smoke() -> bool {
    crate::desktop_shell::phase354_shell_smoke()
}

pub fn phase355_font_smoke() -> bool {
    crate::framebuffer::phase355_font_smoke()
}

pub fn phase375_milestone_smoke() -> bool {
    DESKTOP_READY.fetch_add(1, Ordering::Relaxed);
    phase351_desktop_smoke()
        && phase352_mouse_smoke()
        && phase353_compositor_buffer_smoke()
        && phase354_shell_smoke()
        && phase355_font_smoke()
        && crate::desktop_shell::phase375_desktop_smoke()
}

pub fn phase376_userland_smoke() -> bool {
    crate::userland_install::phase376_userland_smoke()
}

pub fn phase386_network_smoke() -> bool {
    crate::network_stack::phase386_network_smoke()
}

pub fn phase396_package_smoke() -> bool {
    crate::userland_install::phase396_package_smoke()
}

pub fn phase399_native_app_smoke() -> bool {
    crate::userland_install::phase399_native_app_smoke()
}

/// Milestone 400 — full-fledged OS gate.
pub fn phase400_milestone_smoke() -> bool {
    FUNCTIONAL_OS.fetch_add(1, Ordering::Relaxed);
    phase375_milestone_smoke()
        && phase376_userland_smoke()
        && phase386_network_smoke()
        && phase396_package_smoke()
        && phase399_native_app_smoke()
}

fn phase351_desktop_smoke() -> bool {
    crate::post150::phase351_desktop_smoke()
}
