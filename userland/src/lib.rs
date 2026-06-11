//! ares-rt — userspace runtime stubs (epoch 2).
//! See docs/ABI_ARES_RT.md.

#![forbid(unsafe_code)]

/// Compat syscall write (host demo uses stdio).
pub const SYS_WRITE: u64 = 1;

/// Native syscall base (kernel G4).
pub const NATIVE_SYSCALL_BASE: u64 = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AresRtError {
    InvalidArg,
    Unsupported,
}

/// Forward stability: explicit recompile required each epoch until 1.0.
pub const ABI_FORWARD_POLICY: &str = "recompile-required";

pub const ABI_VERSION: &str = "ares-rt-0.1.0";

/// Demo entry — host builds print via std in demo binary.
pub fn demo_message() -> &'static str {
    "ares-rt demo: hello from userland\n"
}
