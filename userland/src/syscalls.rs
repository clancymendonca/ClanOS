//! Clan OS compat syscall IDs for clan-rt (`docs/ABI_SYSCALL.md`).

/// `SyscallId::WriteFd` — write to an open file descriptor.
pub const SYS_WRITE: u64 = 69;
/// `SyscallId::ReadFd` — read from an open file descriptor.
pub const SYS_READ: u64 = 68;
/// `SyscallId::OpenFile` — open a path and return a file descriptor.
pub const SYS_OPEN: u64 = 66;
/// `SyscallId::CloseFile` — close a file descriptor.
pub const SYS_CLOSE: u64 = 67;
/// `SyscallId::ExitProcess` — terminate the current process.
pub const SYS_EXIT: u64 = 61;
/// `SyscallId::GetTickCount`
pub const SYS_GETTICK: u64 = 1;
/// `SyscallId::Kill`
pub const SYS_KILL: u64 = 83;
/// `SyscallId::SigActionLite`
pub const SYS_SIGACTION: u64 = 84;
/// `SyscallId::SigReturn`
pub const SYS_SIGRETURN: u64 = 86;
