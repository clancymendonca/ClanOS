//! Window manager (scopes 352–356) — focus model + damage regions.

use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

use crate::framebuffer::{self, Pixel, Rect};
use crate::mouse::MouseEvent;

const MAX_WINDOWS: usize = 4;
const TITLE_BAR_H: usize = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowId(u32);

#[derive(Debug, Clone)]
struct Window {
    id: WindowId,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    title: &'static str,
    body_color: Pixel,
}

struct WindowState {
    windows: [Option<Window>; MAX_WINDOWS],
    next_id: u32,
    focused: Option<WindowId>,
    damage: [Rect; 8],
    damage_count: usize,
}

static WM_OPS: AtomicU64 = AtomicU64::new(0);

lazy_static::lazy_static! {
    static ref STATE: Mutex<WindowState> = Mutex::new(WindowState {
        windows: [None, None, None, None],
        next_id: 1,
        focused: None,
        damage: [Rect::empty(); 8],
        damage_count: 0,
    });
}

pub fn window_count() -> usize {
    STATE.lock().windows.iter().filter(|w| w.is_some()).count()
}

pub fn focused_window() -> Option<WindowId> {
    STATE.lock().focused
}

fn mark_damage(state: &mut WindowState, rect: Rect) {
    if state.damage_count < state.damage.len() {
        state.damage[state.damage_count] = rect;
        state.damage_count += 1;
    }
}

pub fn create_window(
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    title: &'static str,
) -> Option<WindowId> {
    let mut state = STATE.lock();
    let slot_idx = state.windows.iter().position(|w| w.is_none())?;
    let id = WindowId(state.next_id);
    state.next_id += 1;
    state.windows[slot_idx] = Some(Window {
        id,
        x,
        y,
        w,
        h,
        title,
        body_color: framebuffer::COLOR_WINDOW_BODY,
    });
    state.focused = Some(id);
    mark_damage(
        &mut state,
        Rect::new(x.saturating_sub(2), y.saturating_sub(2), w + 4, h + 4),
    );
    WM_OPS.fetch_add(1, Ordering::Relaxed);
    Some(id)
}

pub fn focus_at(x: i16, y: i16) -> Option<WindowId> {
    let mut state = STATE.lock();
    let mut hit: Option<WindowId> = None;
    for win in state.windows.iter().filter_map(|w| w.as_ref()) {
        let xu = x as usize;
        let yu = y as usize;
        if xu >= win.x && xu < win.x + win.w && yu >= win.y && yu < win.y + win.h {
            hit = Some(win.id);
        }
    }
    if let Some(id) = hit {
        state.focused = Some(id);
        WM_OPS.fetch_add(1, Ordering::Relaxed);
    }
    hit
}

pub fn handle_mouse(event: MouseEvent) {
    if event.buttons & 0x01 != 0 {
        let _ = focus_at(event.x, event.y);
    }
}

pub fn draw_windows(back: &mut [Pixel]) {
    let state = STATE.lock();
    for win in state.windows.iter().filter_map(|w| w.as_ref()) {
        let focused = state.focused == Some(win.id);
        let border = if focused {
            framebuffer::COLOR_ACCENT
        } else {
            framebuffer::COLOR_PANEL
        };
        framebuffer::fill_rect_buf(back, win.x, win.y, win.w, win.h, win.body_color);
        framebuffer::fill_rect_buf(
            back,
            win.x,
            win.y.saturating_sub(TITLE_BAR_H),
            win.w,
            TITLE_BAR_H,
            border,
        );
        framebuffer::draw_text_buf(
            back,
            win.x + 12,
            win.y.saturating_sub(TITLE_BAR_H - 9),
            win.title,
            framebuffer::COLOR_TEXT,
        );
    }
}

pub fn damage_regions_flushed() -> usize {
    STATE.lock().damage_count
}

pub fn smoke_window_manager() -> bool {
    if window_count() < 2 {
        let _ = create_window(120, 120, 300, 180, "TERM");
        let _ = create_window(480, 150, 360, 210, "FILES");
    }
    crate::mouse::inject_event(135, 135, 0x01);
    window_count() >= 2 && focus_at(135, 135).is_some() && damage_regions_flushed() > 0
}

pub fn smoke_wm_smoke() -> bool {
    smoke_window_manager() && focused_window().is_some() && WM_OPS.load(Ordering::Relaxed) >= 1
}
