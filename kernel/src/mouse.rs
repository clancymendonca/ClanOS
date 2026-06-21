//! PS/2 mouse input (scope 352) — IRQ 12 handler + focus events for window manager.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use crossbeam_queue::ArrayQueue;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::port::Port;

const MOUSE_QUEUE_SIZE: usize = 64;
const MOUSE_IRQ: u8 = crate::interrupts::PIC_2_OFFSET + 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseEvent {
    pub x: i16,
    pub y: i16,
    pub buttons: u8,
}

static MOUSE_INITIALIZED: AtomicBool = AtomicBool::new(false);
static MOUSE_EVENTS: AtomicU64 = AtomicU64::new(0);
static CURSOR_X: AtomicU64 = AtomicU64::new(512);
static CURSOR_Y: AtomicU64 = AtomicU64::new(384);

lazy_static! {
    static ref MOUSE_QUEUE: ArrayQueue<MouseEvent> = ArrayQueue::new(MOUSE_QUEUE_SIZE);
    static ref PACKET: Mutex<[u8; 3]> = Mutex::new([0; 3]);
    static ref PACKET_INDEX: Mutex<usize> = Mutex::new(0);
}

pub fn cursor_position() -> (i16, i16) {
    (
        CURSOR_X.load(Ordering::Relaxed) as i16,
        CURSOR_Y.load(Ordering::Relaxed) as i16,
    )
}

pub fn event_count() -> u64 {
    MOUSE_EVENTS.load(Ordering::Relaxed)
}

pub fn initialized() -> bool {
    MOUSE_INITIALIZED.load(Ordering::Relaxed)
}

fn wait_write_ready() {
    for _ in 0..100_000 {
        unsafe {
            if Port::<u8>::new(0x64).read() & 0x02 == 0 {
                return;
            }
        }
    }
}

fn write_command(data: u8) {
    wait_write_ready();
    unsafe {
        Port::<u8>::new(0x64).write(data);
    }
}

fn write_data(data: u8) {
    wait_write_ready();
    unsafe {
        Port::<u8>::new(0x64).write(0xD4);
    }
    wait_write_ready();
    unsafe {
        Port::<u8>::new(0x60).write(data);
    }
}

fn read_data() -> u8 {
    for _ in 0..100_000 {
        unsafe {
            if Port::<u8>::new(0x64).read() & 0x01 != 0 {
                return Port::<u8>::new(0x60).read();
            }
        }
    }
    0
}

/// Enable auxiliary PS/2 port + IRQ 12.
pub fn init() -> bool {
    if MOUSE_INITIALIZED.load(Ordering::Relaxed) {
        return true;
    }

    unsafe {
        write_command(0xA8);
        write_command(0x20);
        let status = read_data();
        write_command(0x60);
        write_data(status | 0x02);
        write_data(0xF6);
        read_data();
        write_data(0xF4);
        read_data();

        let mut mask_port = Port::<u8>::new(0xA1);
        let mask = mask_port.read();
        mask_port.write(mask & !(1 << 4));
    }

    MOUSE_INITIALIZED.store(true, Ordering::Relaxed);
    true
}

pub(crate) fn push_packet(byte: u8) {
    let mut idx = PACKET_INDEX.lock();
    let mut packet = PACKET.lock();
    packet[*idx] = byte;
    *idx += 1;
    if *idx < 3 {
        return;
    }
    *idx = 0;
    drop(idx);
    drop(packet);

    let packet = *PACKET.lock();
    let flags = packet[0];
    let dx = packet[1] as i16;
    let dy = packet[2] as i16;
    let x_sign = if flags & 0x10 != 0 { -256 } else { 0 };
    let y_sign = if flags & 0x20 != 0 { -256 } else { 0 };
    let x_delta = dx + x_sign;
    let y_delta = dy + y_sign;

    let (mut cx, mut cy) = cursor_position();
    cx = (cx + x_delta).clamp(0, crate::framebuffer::WIDTH as i16 - 1);
    cy = (cy + y_delta).clamp(0, crate::framebuffer::HEIGHT as i16 - 1);
    CURSOR_X.store(cx as u64, Ordering::Relaxed);
    CURSOR_Y.store(cy as u64, Ordering::Relaxed);

    let event = MouseEvent {
        x: cx,
        y: cy,
        buttons: flags & 0x07,
    };
    let _ = MOUSE_QUEUE.push(event);
    MOUSE_EVENTS.fetch_add(1, Ordering::Relaxed);
}

pub fn inject_event(x: i16, y: i16, buttons: u8) {
    CURSOR_X.store(
        x.clamp(0, crate::framebuffer::WIDTH as i16 - 1) as u64,
        Ordering::Relaxed,
    );
    CURSOR_Y.store(
        y.clamp(0, crate::framebuffer::HEIGHT as i16 - 1) as u64,
        Ordering::Relaxed,
    );
    let event = MouseEvent { x, y, buttons };
    let _ = MOUSE_QUEUE.push(event);
    MOUSE_EVENTS.fetch_add(1, Ordering::Relaxed);
}

pub fn poll_event() -> Option<MouseEvent> {
    MOUSE_QUEUE.pop()
}

pub fn mouse_irq_vector() -> u8 {
    MOUSE_IRQ
}

pub fn smoke_mouse() -> bool {
    init() && {
        inject_event(300, 240, 0x01);
        event_count() > 0 && poll_event().is_some()
    }
}
