//! Interactive desktop runtime — mouse poll + periodic framebuffer refresh.

use core::time::Duration;

use crate::task::timer::sleep;

pub fn boot_desktop() {
    if !crate::framebuffer::mode_active() {
        let _ = crate::framebuffer::init_display();
    }
    if crate::framebuffer::mode_active() {
        crate::desktop_shell::bootstrap_windows();
        crate::framebuffer::render_desktop_frame();
        if !crate::bga::mode_active() {
            let (misc, seq4, gc6, crtc1) = crate::framebuffer::mode13h_register_readback();
            crate::serial_println!(
                "Desktop: fallback mode13h 320x200 misc={:#04x} seq4={:#04x} gc6={:#04x} crtc1={:#04x}",
                misc,
                seq4,
                gc6,
                crtc1
            );
        }
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
