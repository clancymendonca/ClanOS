//! Native syscall dispatch (IDs 256+, kernel smokes only until phase 128 G4).

use crate::governance::NATIVE_SYSCALL_ID_BASE;
use crate::kernel_object::{self, CapError, Rights};
use crate::syscall::SyscallError;
use crate::task::process::ProcessId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NativeSyscallId {
    CapCreate = 256,
    CapClose = 257,
    CapTransfer = 258,
}

impl NativeSyscallId {
    pub fn from_raw(id: u64) -> Option<Self> {
        match id {
            x if x == Self::CapCreate as u64 => Some(Self::CapCreate),
            x if x == Self::CapClose as u64 => Some(Self::CapClose),
            x if x == Self::CapTransfer as u64 => Some(Self::CapTransfer),
            _ => None,
        }
    }
}

pub fn cap_error_to_syscall(err: CapError) -> SyscallError {
    match err {
        CapError::AmplificationDenied | CapError::AmbientDenied => SyscallError::InvalidArgument,
        _ => SyscallError::InvalidArgument,
    }
}

/// Kernel-only native syscall entry (not in HW allowlist).
pub fn invoke_native(
    id: u64,
    pid: ProcessId,
    arg0: u64,
    arg1: u64,
    _arg2: u64,
) -> Result<u64, SyscallError> {
    if id < NATIVE_SYSCALL_ID_BASE {
        return Err(SyscallError::InvalidSyscall);
    }
    let native = NativeSyscallId::from_raw(id).ok_or(SyscallError::InvalidSyscall)?;
    match native {
        NativeSyscallId::CapCreate => {
            let object_id = kernel_object::ObjectId::from_raw(arg0);
            let rights = Rights(arg1 as u32);
            kernel_object::mint_cap_for_process(pid, object_id, rights)
                .map(u64::from)
                .map_err(cap_error_to_syscall)
        }
        NativeSyscallId::CapClose => {
            kernel_object::close_cap_for_process(pid, arg0 as u32)
                .map(|_| 0)
                .map_err(cap_error_to_syscall)
        }
        NativeSyscallId::CapTransfer => {
            let to_pid = ProcessId::from_raw(arg1);
            kernel_object::cap_transfer_move(pid, arg0 as u32, to_pid)
                .map(u64::from)
                .map_err(cap_error_to_syscall)
        }
    }
}
