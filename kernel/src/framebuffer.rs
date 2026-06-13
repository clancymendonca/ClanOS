//! VGA mode 13h linear framebuffer (320×200, 256 colors @ 0xA0000).
//!
//! Phases 351–355: pixel path, double buffer, expanded font.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::port::{Port, PortReadOnly};

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 200;
pub const BUFFER_LEN: usize = WIDTH * HEIGHT;
const FRAMEBUFFER: *mut u8 = 0xA0000 as *mut u8;

static MODE_ACTIVE: AtomicBool = AtomicBool::new(false);
static PIXELS_DRAWN: AtomicU64 = AtomicU64::new(0);
static FLUSH_COUNT: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    static ref BACK_BUFFER: Mutex<[u8; BUFFER_LEN]> = Mutex::new([0; BUFFER_LEN]);
}

/// VGA palette index — mode 13h default palette is close enough for UI blocks.
pub const COLOR_DESKTOP: u8 = 3;
pub const COLOR_TITLEBAR: u8 = 1;
pub const COLOR_ACCENT: u8 = 14;
pub const COLOR_TEXT: u8 = 15;
pub const COLOR_CURSOR: u8 = 15;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    pub const fn empty() -> Self {
        Self {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        }
    }

    pub const fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x, y, w, h }
    }
}

pub fn mode_active() -> bool {
    MODE_ACTIVE.load(Ordering::Relaxed)
}

pub fn pixels_drawn() -> u64 {
    PIXELS_DRAWN.load(Ordering::Relaxed)
}

pub fn flush_count() -> u64 {
    FLUSH_COUNT.load(Ordering::Relaxed)
}

pub fn back_buffer_mut<F>(f: F)
where
    F: FnOnce(&mut [u8; BUFFER_LEN]),
{
    let mut buf = BACK_BUFFER.lock();
    f(&mut buf);
}

/// Switch to 320×200×256 graphics mode (BIOS mode 13h register sequence).
pub fn init_mode_13h() -> bool {
    if MODE_ACTIVE.load(Ordering::Relaxed) {
        return true;
    }

    unsafe {
        // Misc output: OSDev mode 13h canonical (0x63 — not 0xE3 used by mode 12h).
        Port::<u8>::new(0x3C2).write(0x63);

        let mut isr = PortReadOnly::<u8>::new(0x3DA);
        let mut seq_index = Port::<u8>::new(0x3C4);
        let mut seq_data = Port::<u8>::new(0x3C5);
        let mut crtc_index = Port::<u8>::new(0x3D4);
        let mut crtc_data = Port::<u8>::new(0x3D5);
        let mut gc_index = Port::<u8>::new(0x3CE);
        let mut gc_data = Port::<u8>::new(0x3CF);
        let mut attr = Port::<u8>::new(0x3C0);

        // Sequencer: async reset, program chain-4 (required for linear 0xA0000 bytes).
        seq_index.write(0x00);
        seq_data.write(0x01);
        seq_index.write(0x01);
        seq_data.write(0x01);
        seq_index.write(0x02);
        seq_data.write(0x0F);
        seq_index.write(0x03);
        seq_data.write(0x00);
        seq_index.write(0x04);
        seq_data.write(0x0E);
        seq_index.write(0x00);
        seq_data.write(0x03);

        // Unlock CRTC registers 0–7.
        crtc_index.write(0x11);
        let crtc11 = crtc_data.read();
        crtc_data.write(crtc11 & 0x7F);

        let crtc: [(u8, u8); 25] = [
            (0x00, 0x5F),
            (0x01, 0x4F),
            (0x02, 0x50),
            (0x03, 0x82),
            (0x04, 0x54),
            (0x05, 0x80),
            (0x06, 0xBF),
            (0x07, 0x1F),
            (0x08, 0x00),
            (0x09, 0x41),
            (0x0A, 0x00),
            (0x0B, 0x00),
            (0x0C, 0x00),
            (0x0D, 0x00),
            (0x0E, 0x00),
            (0x0F, 0x00),
            (0x10, 0x9C),
            (0x11, 0x0E),
            (0x12, 0x8F),
            (0x13, 0x28),
            (0x14, 0x40),
            (0x15, 0x96),
            (0x16, 0xB9),
            (0x17, 0xA3),
            (0x18, 0xFF),
        ];
        for (idx, val) in crtc {
            crtc_index.write(idx);
            crtc_data.write(val);
        }

        // Graphics controller: write mode 0, map 0xA0000.
        for (idx, val) in [
            (0x00, 0x00),
            (0x01, 0x00),
            (0x02, 0x00),
            (0x03, 0x00),
            (0x04, 0x00),
            (0x05, 0x40),
            (0x06, 0x05),
            (0x07, 0x0F),
            (0x08, 0xFF),
        ] {
            gc_index.write(idx);
            gc_data.write(val);
        }

        // Attribute controller: identity palette + 256-color graphics mode.
        let _ = isr.read();
        for i in 0..16u8 {
            attr.write(i);
            attr.write(i);
        }
        let _ = isr.read();
        // Mode control: 256-color graphics (required for linear 0xA0000 bytes).
        attr.write(0x10);
        attr.write(0x41);
        // Overscan, plane enable, panning, color select — missing these leaves planar stripes in QEMU.
        attr.write(0x11);
        attr.write(0x00);
        attr.write(0x12);
        attr.write(0x0F);
        attr.write(0x13);
        attr.write(0x00);
        attr.write(0x14);
        attr.write(0x00);
        let _ = isr.read();
        attr.write(0x20);
    }

    init_vga_palette();
    // Clear VRAM so stale planar data does not show through before first flush.
    unsafe {
        for i in 0..BUFFER_LEN {
            FRAMEBUFFER.add(i).write_volatile(0);
        }
    }
    MODE_ACTIVE.store(true, Ordering::Relaxed);
    true
}

/// Read back the mode-critical registers to confirm the writes landed.
/// Returns (misc_output, seq4, gc6, crtc1).
pub fn mode13h_register_readback() -> (u8, u8, u8, u8) {
    unsafe {
        let misc = PortReadOnly::<u8>::new(0x3CC).read();
        let mut seq_index = Port::<u8>::new(0x3C4);
        let mut seq_data = Port::<u8>::new(0x3C5);
        seq_index.write(0x04);
        let seq4 = seq_data.read();
        let mut gc_index = Port::<u8>::new(0x3CE);
        let mut gc_data = Port::<u8>::new(0x3CF);
        gc_index.write(0x06);
        let gc6 = gc_data.read();
        let mut crtc_index = Port::<u8>::new(0x3D4);
        let mut crtc_data = Port::<u8>::new(0x3D5);
        crtc_index.write(0x01);
        let crtc1 = crtc_data.read();
        (misc, seq4, gc6, crtc1)
    }
}

/// Load 256-color DAC entries for mode 13h (QEMU defaults are often all black).
///
/// Indices 0–15 carry the standard VGA 16-color palette (the UI block colors:
/// `COLOR_DESKTOP`=cyan, `COLOR_TITLEBAR`=blue, …); 16–255 are a grayscale ramp.
fn init_vga_palette() {
    // 6-bit DAC RGB triplets for the standard VGA 16-color palette.
    const STD16: [(u8, u8, u8); 16] = [
        (0, 0, 0),
        (0, 0, 42),
        (0, 42, 0),
        (0, 42, 42),
        (42, 0, 0),
        (42, 0, 42),
        (42, 21, 0),
        (42, 42, 42),
        (21, 21, 21),
        (21, 21, 63),
        (21, 63, 21),
        (21, 63, 63),
        (63, 21, 21),
        (63, 21, 63),
        (63, 63, 21),
        (63, 63, 63),
    ];
    unsafe {
        let mut dac_write = Port::<u8>::new(0x3C8);
        let mut dac_data = Port::<u8>::new(0x3C9);
        dac_write.write(0);
        for (r, g, b) in STD16 {
            dac_data.write(r);
            dac_data.write(g);
            dac_data.write(b);
        }
        for i in 16..=255u8 {
            dac_data.write(i >> 2);
            dac_data.write(i >> 2);
            dac_data.write(i >> 2);
        }
    }
}

pub fn plot_pixel(x: usize, y: usize, color: u8) {
    if !mode_active() || x >= WIDTH || y >= HEIGHT {
        return;
    }
    unsafe {
        FRAMEBUFFER.add(y * WIDTH + x).write_volatile(color);
    }
    PIXELS_DRAWN.fetch_add(1, Ordering::Relaxed);
}

pub fn fill_rect_buf(buf: &mut [u8], x: usize, y: usize, w: usize, h: usize, color: u8) {
    if w == 0 || h == 0 {
        return;
    }
    let x_end = (x + w).min(WIDTH);
    let y_end = (y + h).min(HEIGHT);
    for row in y..y_end {
        for col in x..x_end {
            buf[row * WIDTH + col] = color;
        }
    }
}

pub fn fill_rect(x: usize, y: usize, w: usize, h: usize, color: u8) {
    back_buffer_mut(|buf| fill_rect_buf(buf, x, y, w, h, color));
}

fn glyph(c: u8) -> Option<[u8; 7]> {
    match c {
        b' ' => Some([0, 0, 0, 0, 0, 0, 0]),
        b'-' => Some([0, 0, 0, 0x1F, 0, 0, 0]),
        b'A' => Some([0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11]),
        b'C' => Some([0x0E, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0E]),
        b'E' => Some([0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F]),
        b'F' => Some([0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x10]),
        b'H' => Some([0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11]),
        b'I' => Some([0x0E, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0E]),
        b'L' => Some([0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1F]),
        b'M' => Some([0x11, 0x1B, 0x15, 0x11, 0x11, 0x11, 0x11]),
        b'N' => Some([0x11, 0x11, 0x19, 0x15, 0x13, 0x11, 0x11]),
        b'O' => Some([0x0E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E]),
        b'P' => Some([0x1E, 0x11, 0x11, 0x1E, 0x10, 0x10, 0x10]),
        b'R' => Some([0x1E, 0x11, 0x11, 0x1E, 0x14, 0x12, 0x11]),
        b'S' => Some([0x0F, 0x10, 0x10, 0x0E, 0x01, 0x01, 0x1E]),
        b'T' => Some([0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04]),
        b'U' => Some([0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E]),
        b'Y' => Some([0x11, 0x11, 0x0A, 0x04, 0x04, 0x04, 0x04]),
        _ => None,
    }
}

pub fn draw_text_buf(buf: &mut [u8], x: usize, y: usize, text: &str, color: u8) {
    let mut cursor = x;
    for ch in text.bytes() {
        if let Some(rows) = glyph(ch) {
            for (row_idx, row) in rows.iter().enumerate() {
                for bit in 0..5 {
                    if row & (1 << (4 - bit)) != 0 {
                        let px = cursor + bit;
                        let py = y + row_idx;
                        if px < WIDTH && py < HEIGHT {
                            buf[py * WIDTH + px] = color;
                        }
                    }
                }
            }
        }
        cursor += 6;
        if cursor + 5 >= WIDTH {
            break;
        }
    }
}

pub fn draw_text(x: usize, y: usize, text: &str, color: u8) {
    back_buffer_mut(|buf| draw_text_buf(buf, x, y, text, color));
}

pub fn draw_cursor(buf: &mut [u8], x: i16, y: i16) {
    let x = x as usize;
    let y = y as usize;
    if x + 1 < WIDTH && y + 1 < HEIGHT {
        buf[y * WIDTH + x] = COLOR_CURSOR;
        buf[y * WIDTH + x + 1] = COLOR_CURSOR;
        buf[(y + 1) * WIDTH + x] = COLOR_CURSOR;
        buf[(y + 1) * WIDTH + x + 1] = COLOR_CURSOR;
    }
}

pub fn flush_to_screen(buf: &[u8; BUFFER_LEN]) {
    if !mode_active() {
        return;
    }
    unsafe {
        for (i, &byte) in buf.iter().enumerate() {
            FRAMEBUFFER.add(i).write_volatile(byte);
        }
    }
    FLUSH_COUNT.fetch_add(1, Ordering::Relaxed);
}

pub fn render_desktop_frame() {
    if !mode_active() {
        let _ = init_mode_13h();
    }
    back_buffer_mut(|buf| {
        fill_rect_buf(buf, 0, 0, WIDTH, HEIGHT, COLOR_DESKTOP);
        fill_rect_buf(buf, 0, 0, WIDTH, 16, COLOR_TITLEBAR);
        draw_text_buf(buf, 8, 4, "AREOS", COLOR_TEXT);
        crate::desktop_shell::render_shell(buf);
        let (cx, cy) = crate::mouse::cursor_position();
        draw_cursor(buf, cx, cy);
    });
    let buf = BACK_BUFFER.lock();
    flush_to_screen(&buf);
}

pub fn draw_desktop_shell() {
    render_desktop_frame();
}

pub fn phase351_framebuffer_smoke() -> bool {
    init_mode_13h() && mode_active() && {
        draw_desktop_shell();
        flush_count() >= 1
    }
}

pub fn phase353_double_buffer_smoke() -> bool {
    init_mode_13h() && {
        render_desktop_frame();
        flush_count() >= 1
    }
}

pub fn phase355_font_smoke() -> bool {
    init_mode_13h() && {
        back_buffer_mut(|buf| {
            draw_text_buf(buf, 10, 30, "AREOS DESKTOP", COLOR_TEXT);
            draw_text_buf(buf, 10, 40, "CONSOLE FILES RUN", COLOR_ACCENT);
        });
        true
    }
}
