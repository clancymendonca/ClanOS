//! Saved hardware user trap frames for scheduler preemption (Phase 32).

use core::sync::atomic::{AtomicU64, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;

static FRAME_SAVES: AtomicU64 = AtomicU64::new(0);
static FRAME_RESUMES: AtomicU64 = AtomicU64::new(0);
static FRAME_PREEMPTED: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, Default)]
pub struct UserHwFrame {
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub iteration: u64,
}

lazy_static! {
    static ref SAVED_FRAME: Mutex<Option<UserHwFrame>> = Mutex::new(None);
}

pub fn status() -> (u64, u64, bool) {
    (
        FRAME_SAVES.load(Ordering::Relaxed),
        FRAME_RESUMES.load(Ordering::Relaxed),
        FRAME_PREEMPTED.load(Ordering::Relaxed) != 0,
    )
}

pub fn save_frame(frame: UserHwFrame) {
    *SAVED_FRAME.lock() = Some(frame);
    FRAME_SAVES.fetch_add(1, Ordering::Relaxed);
}

pub fn take_saved_frame() -> Option<UserHwFrame> {
    SAVED_FRAME.lock().take()
}

pub fn resume_saved_frame() -> Option<UserHwFrame> {
    let frame = take_saved_frame()?;
    FRAME_RESUMES.fetch_add(1, Ordering::Relaxed);
    Some(frame)
}

pub fn mark_preempted() {
    FRAME_PREEMPTED.store(1, Ordering::Relaxed);
}

pub fn phase32_smoke() -> bool {
    save_frame(UserHwFrame {
        rip: 0x400000,
        rsp: 0x7ffff000,
        rflags: 0x202,
        rax: 1,
        rbx: 2,
        rcx: 3,
        rdx: 4,
        iteration: 0,
    });
    mark_preempted();
    let _ = crate::task::scheduler::yield_now();
    let resumed = resume_saved_frame();
    resumed
        .map(|f| f.rip == 0x400000 && f.rsp == 0x7ffff000)
        .unwrap_or(false)
}
