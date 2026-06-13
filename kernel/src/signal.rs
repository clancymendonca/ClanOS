//! Compat signal skeleton: `kill`, `sigaction_lite`, pending-bit probe, and
//! user-frame delivery on syscall return with `SigReturn` restoration (scope 409).

use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicU64, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::security::Credentials;
use crate::task::process::{self, ProcessId};

pub const MAX_SIGNO: u32 = 32;
pub const SIG_DFL: u64 = 0;
pub const SIG_IGN: u64 = 1;

pub const SIGKILL: u32 = 9;
pub const SIGUSR1: u32 = 10;
pub const SIGTERM: u32 = 15;

static KILL_DELIVERED: AtomicU64 = AtomicU64::new(0);
static SIGACTION_SET: AtomicU64 = AtomicU64::new(0);
static SIGNAL_IGNORED: AtomicU64 = AtomicU64::new(0);
static SIGNAL_QUEUED: AtomicU64 = AtomicU64::new(0);
static FRAME_DELIVERIES: AtomicU64 = AtomicU64::new(0);
static SIGRETURN_COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, Default)]
pub struct SignalState {
    pending: u32,
    handlers: [u64; MAX_SIGNO as usize],
    saved_rip: u64,
    saved_rflags: u64,
    last_delivered_signo: u32,
    in_handler: bool,
}

impl SignalState {
    pub fn handler(&self, signo: u32) -> u64 {
        self.handlers
            .get(signo as usize)
            .copied()
            .unwrap_or(SIG_DFL)
    }
}

lazy_static! {
    static ref SIGNAL_TABLE: Mutex<BTreeMap<ProcessId, SignalState>> =
        Mutex::new(BTreeMap::new());
}

pub fn status() -> (u64, u64, u64) {
    (
        KILL_DELIVERED.load(Ordering::Relaxed),
        SIGACTION_SET.load(Ordering::Relaxed),
        SIGNAL_QUEUED.load(Ordering::Relaxed),
    )
}

pub fn delivery_status() -> (u64, u64) {
    (
        FRAME_DELIVERIES.load(Ordering::Relaxed),
        SIGRETURN_COUNT.load(Ordering::Relaxed),
    )
}

pub fn init_process(pid: ProcessId) {
    SIGNAL_TABLE.lock().entry(pid).or_default();
}

pub fn drop_process(pid: ProcessId) {
    SIGNAL_TABLE.lock().remove(&pid);
}

pub fn inherit_from_parent(parent: ProcessId, child: ProcessId) {
    let state = SIGNAL_TABLE
        .lock()
        .get(&parent)
        .copied()
        .unwrap_or_default();
    SIGNAL_TABLE.lock().insert(child, state);
}

fn with_state_mut<F, R>(pid: ProcessId, f: F) -> Option<R>
where
    F: FnOnce(&mut SignalState) -> R,
{
    SIGNAL_TABLE.lock().get_mut(&pid).map(f)
}

fn process_alive(pid: ProcessId) -> bool {
    process::with_process_mut(pid, |proc| proc.exit_code().is_none())
        .unwrap_or(false)
}

fn first_pending_custom(state: &SignalState) -> Option<u32> {
    for signo in 1..MAX_SIGNO {
        if state.pending & (1u32 << signo) == 0 {
            continue;
        }
        let handler = state.handler(signo);
        if handler != SIG_DFL && handler != SIG_IGN {
            return Some(signo);
        }
    }
    None
}

fn apply_default(pid: ProcessId, signo: u32) -> Result<(), ()> {
    match signo {
        SIGKILL | SIGTERM => {
            let code = 128i32.saturating_add(signo as i32);
            if process::terminate_process(pid, code) {
                KILL_DELIVERED.fetch_add(1, Ordering::Relaxed);
                Ok(())
            } else {
                Err(())
            }
        }
        _ => queue_pending(pid, signo),
    }
}

fn queue_pending(pid: ProcessId, signo: u32) -> Result<(), ()> {
    with_state_mut(pid, |state| {
        state.pending |= 1u32 << signo;
        SIGNAL_QUEUED.fetch_add(1, Ordering::Relaxed);
    })
    .ok_or(())
}

pub fn dispatch(pid: ProcessId, signo: u32) -> Result<(), ()> {
    if signo == 0 || signo >= MAX_SIGNO {
        return Err(());
    }
    if !process_alive(pid) {
        return Err(());
    }
    if signo == SIGKILL {
        return apply_default(pid, signo);
    }
    let handler = with_state_mut(pid, |state| state.handler(signo))
        .unwrap_or(SIG_DFL);
    match handler {
        SIG_IGN => {
            SIGNAL_IGNORED.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
        SIG_DFL => apply_default(pid, signo),
        _ => queue_pending(pid, signo),
    }
}

/// Redirect syscall return to a user handler when a custom handler is pending.
pub fn try_deliver_on_syscall_return(
    pid: ProcessId,
    user_rip: u64,
    user_rflags: u64,
) -> Option<(u64, u64)> {
    with_state_mut(pid, |state| {
        if state.in_handler {
            return None;
        }
        let signo = first_pending_custom(state)?;
        state.pending &= !(1u32 << signo);
        state.saved_rip = user_rip;
        state.saved_rflags = user_rflags;
        state.last_delivered_signo = signo;
        state.in_handler = true;
        FRAME_DELIVERIES.fetch_add(1, Ordering::Relaxed);
        Some((state.handler(signo), user_rflags))
    })
    .flatten()
}

/// Restore saved user frame after `SigReturn` syscall dispatch succeeds.
pub fn take_sigreturn_restoration(pid: ProcessId) -> Option<(u64, u64)> {
    with_state_mut(pid, |state| {
        if !state.in_handler || state.saved_rip == 0 {
            return None;
        }
        let rip = state.saved_rip;
        let rflags = state.saved_rflags;
        state.saved_rip = 0;
        state.saved_rflags = 0;
        state.in_handler = false;
        state.last_delivered_signo = 0;
        SIGRETURN_COUNT.fetch_add(1, Ordering::Relaxed);
        Some((rip, rflags))
    })
    .flatten()
}

pub fn kill_checked(actor: Credentials, pid: ProcessId, signo: u32) -> Result<(), ()> {
    if signo >= MAX_SIGNO {
        return Err(());
    }
    if signo == 0 {
        return if process::get_process(pid).is_some() {
            Ok(())
        } else {
            Err(())
        };
    }
    let owner = process::process_owner(pid).ok_or(())?;
    if !crate::security::can_manage_process(actor, owner) {
        crate::security::record_denial(crate::security::AccessKind::Manage);
        return Err(());
    }
    dispatch(pid, signo)
}

pub fn kill_syscall(pid_raw: u64, signo: u64) -> Result<(), ()> {
    let pid = if pid_raw == 0 {
        process::current_process_id()
            .or_else(|| process::smoke_process_id())
            .ok_or(())?
    } else {
        ProcessId::from_raw(pid_raw)
    };
    kill_checked(
        crate::security::current_credentials(),
        pid,
        signo as u32,
    )
}

pub fn sigaction_lite(signo: u64, handler: u64) -> Result<u64, ()> {
    if signo == 0 || signo >= MAX_SIGNO as u64 || signo == SIGKILL as u64 {
        return Err(());
    }
    let pid = process::current_process_id()
        .or_else(|| process::smoke_process_id())
        .ok_or(())?;
    with_state_mut(pid, |state| {
        let idx = signo as usize;
        let old = state.handlers[idx];
        state.handlers[idx] = handler;
        SIGACTION_SET.fetch_add(1, Ordering::Relaxed);
        old
    })
    .ok_or(())
}

pub fn sigpending_syscall() -> Result<u64, ()> {
    let pid = process::current_process_id()
        .or_else(|| process::smoke_process_id())
        .ok_or(())?;
    with_state_mut(pid, |state| state.pending as u64)
        .ok_or(())
}

pub fn sigreturn_syscall() -> Result<u64, ()> {
    let pid = process::current_process_id()
        .or_else(|| process::smoke_process_id())
        .ok_or(())?;
    with_state_mut(pid, |state| {
        if !state.in_handler {
            return Err(());
        }
        Ok(state.last_delivered_signo as u64)
    })
    .ok_or(())?
    .map_err(|_| ())
}

pub fn smoke_signal_register() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = Credentials::shell_user();
    let Some(victim) = process::create_kernel_process_as("sig-victim", tick, creds) else {
        return false;
    };
    process::set_smoke_process_id(Some(victim));

    let old = sigaction_lite(SIGUSR1 as u64, SIG_IGN).ok() == Some(SIG_DFL);
    let ignore_ok = kill_checked(creds, victim, SIGUSR1).is_ok() && process_alive(victim);
    let term_ok = kill_checked(creds, victim, SIGTERM).is_ok() && !process_alive(victim);
    let exit_code = process::with_process_mut(victim, |proc| proc.exit_code())
        .flatten();
    let exit_ok = exit_code == Some(128 + SIGTERM as i32);

    process::set_smoke_process_id(None);
    let (kills, actions, _) = status();
    old && ignore_ok && term_ok && exit_ok && kills > 0 && actions > 0
}

pub fn smoke_signal_delivery() -> bool {
    const HANDLER_RIP: u64 = 0x4001_0000;
    const USER_RIP: u64 = 0x4000_2000;
    const USER_RFLAGS: u64 = 0x202;

    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = Credentials::shell_user();
    let Some(victim) = process::create_kernel_process_as("sig-deliver", tick, creds) else {
        return false;
    };
    process::set_smoke_process_id(Some(victim));

    let handler_set = sigaction_lite(SIGUSR1 as u64, HANDLER_RIP).ok() == Some(SIG_DFL);
    let queued = kill_checked(creds, victim, SIGUSR1).is_ok();
    let pending_before = sigpending_syscall().ok().unwrap_or(0);
    let pending_bit = (pending_before & (1u32 << SIGUSR1) as u64) != 0;

    let redirected = try_deliver_on_syscall_return(victim, USER_RIP, USER_RFLAGS)
        .map(|(rip, rflags)| rip == HANDLER_RIP && rflags == USER_RFLAGS)
        .unwrap_or(false);
    let pending_after = sigpending_syscall().ok().unwrap_or(0);
    let pending_cleared = pending_after & (1u32 << SIGUSR1) as u64 == 0;

    let signo_ack = sigreturn_syscall().ok() == Some(SIGUSR1 as u64);
    let restored = take_sigreturn_restoration(victim)
        .map(|(rip, rflags)| rip == USER_RIP && rflags == USER_RFLAGS)
        .unwrap_or(false);
    let no_reentry = try_deliver_on_syscall_return(victim, USER_RIP, USER_RFLAGS).is_none();

    process::set_smoke_process_id(None);
    let (deliveries, sigreturns) = delivery_status();

    handler_set
        && queued
        && pending_bit
        && redirected
        && pending_cleared
        && signo_ack
        && restored
        && no_reentry
        && deliveries > 0
        && sigreturns > 0
}
