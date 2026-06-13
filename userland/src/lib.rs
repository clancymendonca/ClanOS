//! clan-rt — Clan OS userspace runtime stubs (epoch 2).
//! See docs/specs/ABI_CLAN_RT.md.
//!
//! Crate and manifest identifiers (`clan-rt`, `clan-exec-v1`) are stable ABI names.

#![no_std]
#![deny(unsafe_code)]

pub mod syscalls;

#[cfg(feature = "ring3")]
pub mod ring3_syscall;

#[cfg(feature = "ring3-heap")]
pub mod heap;

pub use syscalls::{SYS_CLOSE, SYS_EXIT, SYS_OPEN, SYS_READ, SYS_WRITE};

/// Legacy alias kept for host demo sources.
pub const SYS_WRITE_FD: u64 = SYS_WRITE;

/// Native syscall base (kernel G4).
pub const NATIVE_SYSCALL_BASE: u64 = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClanRtError {
    InvalidArg,
    Unsupported,
}

/// Forward stability: explicit recompile required each epoch until 1.0.
pub const ABI_FORWARD_POLICY: &str = "recompile-required";

pub const ABI_VERSION: &str = "clan-rt-0.1.0";

/// Demo entry — host builds print via std in demo binary.
pub fn demo_message() -> &'static str {
    "Clan OS demo: hello from userland\n"
}
