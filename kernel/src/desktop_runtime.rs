//! Interactive desktop runtime — mouse poll + periodic framebuffer refresh.

use core::time::Duration;

use crate::task::timer::sleep;

pub fn boot_desktop() {
    if crate::framebuffer::init_mode_13h() {
        crate::desktop_shell::bootstrap_windows();
        crate::framebuffer::render_desktop_frame();
    }
}

pub async fn refresh_loop() {
    loop {
        while let Some(event) = crate::mouse::poll_event() {
            crate::window_manager::handle_mouse(event);
        }
        if crate::framebuffer::mode_active() {
            crate::framebuffer::render_desktop_frame();
        }
        sleep(Duration::from_millis(100)).await;
    }
}
