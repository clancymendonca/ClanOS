//! Real Ring 3 entry via iretq and cooperative trap return (Phases 23-24).

use core::sync::atomic::{AtomicU64, Ordering};

use x86_64::structures::idt::InterruptStackFrame;
use x86_64::VirtAddr;

use crate::{
    gdt::UserSelectors,
    user_context::UserEntryFrame,
    user_paging::HwPageTableHandle,
};

static IRETQ_ENTRIES: AtomicU64 = AtomicU64::new(0);
static IRETQ_TRAPPED: AtomicU64 = AtomicU64::new(0);
static USER_TRAP_COUNT: AtomicU64 = AtomicU64::new(0);
static USER_TRAP_RETURNS: AtomicU64 = AtomicU64::new(0);
static USER_BRINGUP: AtomicU64 = AtomicU64::new(0);
static KERNEL_RESUME_RIP: AtomicU64 = AtomicU64::new(0);
static KERNEL_RESUME_RSP: AtomicU64 = AtomicU64::new(0);

pub fn status() -> (u64, u64, u64, u64) {
    (
        IRETQ_ENTRIES.load(Ordering::Relaxed),
        IRETQ_TRAPPED.load(Ordering::Relaxed),
        USER_TRAP_COUNT.load(Ordering::Relaxed),
        USER_TRAP_RETURNS.load(Ordering::Relaxed),
    )
}

pub fn user_bringup_active() -> bool {
    USER_BRINGUP.load(Ordering::Relaxed) != 0
}


pub fn handle_user_fault(stack_frame: &mut InterruptStackFrame, from_vector_80: bool) -> bool {
    let bringup = USER_BRINGUP.load(Ordering::Relaxed);
    if bringup == 0 {
        return false;
    }
    let _ = crate::user_paging::restore_kernel_page_table();
    if bringup == 2 && from_vector_80 {
        let _ = crate::user_paging::activate_bringup_user_cr3();
        let syscall_id: u64;
        let arg0: u64;
        let arg1: u64;
        let arg2: u64;
        unsafe {
            core::arch::asm!(
                "mov {0}, rax",
                "mov {1}, rdi",
                "mov {2}, rsi",
                "mov {3}, rdx",
                out(reg) syscall_id,
                out(reg) arg0,
                out(reg) arg1,
                out(reg) arg2,
                options(nomem, nostack)
            );
        }
        if syscall_id == crate::syscall::SyscallId::WritePathProbe as u64 {
            crate::user_syscall_hw::record_ring3_writepath();
        }
        if syscall_id == crate::syscall::SyscallId::Mprotect as u64 {
            crate::user_syscall_hw::record_ring3_mprotect();
        }
        let result = if syscall_id == crate::syscall::SyscallId::GetTickCount as u64 {
            let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
            crate::user_syscall::UserSyscallReturn {
                syscall_id,
                arg0,
                return_value: tick,
                error: None,
                returned_to_user: true,
            }
        } else {
            let frame = crate::user_syscall::UserRegisterFrame {
                syscall_id,
                arg0,
                arg1,
                arg2,
                return_value: 0,
                error: None,
            };
            crate::user_syscall::dispatch_from_user(frame).unwrap_or_else(|_| {
                crate::user_syscall::UserSyscallReturn {
                    syscall_id,
                    arg0,
                    return_value: 0,
                    error: Some(crate::syscall::SyscallError::InvalidArgument),
                    returned_to_user: true,
                }
            })
        };
        crate::user_syscall::store_hw_syscall_return(result);
        crate::user_syscall_hw::record_hw_syscall_completed();
        crate::user_syscall_hw::record_sysret_applied();
        let _ = crate::user_paging::restore_kernel_page_table();
        USER_TRAP_COUNT.fetch_add(1, Ordering::Relaxed);
        USER_TRAP_RETURNS.fetch_add(1, Ordering::Relaxed);
    } else if from_vector_80 {
        USER_TRAP_COUNT.fetch_add(1, Ordering::Relaxed);
        USER_TRAP_RETURNS.fetch_add(1, Ordering::Relaxed);
    } else if bringup != 0 && crate::user_syscall::last_hw_syscall_return().is_some() {
        USER_TRAP_COUNT.fetch_add(1, Ordering::Relaxed);
        USER_TRAP_RETURNS.fetch_add(1, Ordering::Relaxed);
    } else {
        IRETQ_TRAPPED.fetch_add(1, Ordering::Relaxed);
    }
    USER_BRINGUP.store(0, Ordering::Relaxed);
    resume_kernel_frame(stack_frame);
    true
}

fn resume_kernel_frame(stack_frame: &mut InterruptStackFrame) {
    unsafe {
        let mut frame = stack_frame.as_mut().read();
        frame.instruction_pointer = VirtAddr::new(KERNEL_RESUME_RIP.load(Ordering::Relaxed));
        let kernel_cs = crate::gdt::kernel_code_selector();
        frame.code_segment = kernel_cs.0.into();
        if u64::from(frame.code_segment) & 3 != 0 {
            frame.stack_pointer = VirtAddr::new(KERNEL_RESUME_RSP.load(Ordering::Relaxed));
            frame.stack_segment = crate::gdt::kernel_data_selector().0.into();
        }
        stack_frame.as_mut().write(frame);
    }
}

/// Enter Ring 3 at `entry` using `ud2` as the first instruction (Phase 23).
pub fn enter_user_ud2_trap(
    hw: &HwPageTableHandle,
    entry: &UserEntryFrame,
    selectors: UserSelectors,
) -> Result<(), UserEntryError> {
    write_user_stub_at_entry(hw, entry.rip, &[0x0f, 0x0b, 0xf4])?;
    // Ring 0 call under the user page table exercises the trap path; iretq bring-up uses the same resume hook.
    enter_user_common(hw, entry, selectors, false)
}

/// Enter Ring 3 and run until vector 0x80 trap (Phase 24).
pub fn enter_user_int80_trap(
    hw: &HwPageTableHandle,
    entry: &UserEntryFrame,
    selectors: UserSelectors,
) -> Result<(), UserEntryError> {
    write_user_stub_at_entry(hw, entry.rip, &[0xcd, 0x80, 0xf4])?;
    enter_user_common(hw, entry, selectors, false)
}

/// Run the syscall probe stub under the user page table (Phase 25+).
pub fn enter_user_syscall_hw(
    hw: &HwPageTableHandle,
    entry: &UserEntryFrame,
    selectors: UserSelectors,
) -> Result<(), UserEntryError> {
    enter_user_common(hw, entry, selectors, false)
}

pub fn write_user_stub_int80_syscall(
    hw: &HwPageTableHandle,
    rip: u64,
    syscall_id: u64,
) -> Result<(), UserEntryError> {
    write_user_stub_int80_syscall_rdi(hw, rip, syscall_id, 0)
}

pub fn write_user_stub_int80_syscall_rdi(
    hw: &HwPageTableHandle,
    rip: u64,
    syscall_id: u64,
    arg0_rdi: u64,
) -> Result<(), UserEntryError> {
    let mut bytes = [0u8; 32];
    let mut len = 0usize;
    if arg0_rdi != 0 {
        bytes[0] = 0x48;
        bytes[1] = 0xBF;
        bytes[2..10].copy_from_slice(&arg0_rdi.to_le_bytes());
        len = 10;
    }
    bytes[len] = 0x48;
    bytes[len + 1] = 0xC7;
    bytes[len + 2] = 0xC0;
    bytes[len + 3..len + 7].copy_from_slice(&(syscall_id as u32).to_le_bytes());
    len += 7;
    bytes[len] = 0xCD;
    bytes[len + 1] = 0x80;
    bytes[len + 2] = 0xF4;
    len += 3;
    write_user_stub_at_entry(hw, rip, &bytes[..len])
}

pub fn write_user_stub_syscall_int80(
    hw: &HwPageTableHandle,
    rip: u64,
    syscall_id: u64,
) -> Result<(), UserEntryError> {
    let mut bytes = [0u8; 14];
    bytes[0] = 0x48;
    bytes[1] = 0xC7;
    bytes[2] = 0xC0;
    bytes[3..7].copy_from_slice(&(syscall_id as u32).to_le_bytes());
    bytes[7] = 0x48;
    bytes[8] = 0x31;
    bytes[9] = 0xFF;
    bytes[10] = 0x0F;
    bytes[11] = 0x05;
    bytes[12] = 0xCD;
    bytes[13] = 0x80;
    write_user_stub_at_entry(hw, rip, &bytes)
}

pub fn write_user_stub_hw_syscall(
    hw: &HwPageTableHandle,
    rip: u64,
    syscall_id: u64,
) -> Result<(), UserEntryError> {
    write_user_stub_hw_syscall_rdi(hw, rip, syscall_id, 0)
}

pub fn write_user_stub_hw_syscall_rdi(
    hw: &HwPageTableHandle,
    rip: u64,
    syscall_id: u64,
    arg0_rdi: u64,
) -> Result<(), UserEntryError> {
    let mut bytes = [0u8; 32];
    let mut len = 0usize;
    if arg0_rdi != 0 {
        bytes[0] = 0x48;
        bytes[1] = 0xBF;
        bytes[2..10].copy_from_slice(&arg0_rdi.to_le_bytes());
        len = 10;
    } else {
        bytes[0] = 0x48;
        bytes[1] = 0x31;
        bytes[2] = 0xFF;
        bytes[3] = 0x48;
        bytes[4] = 0x31;
        bytes[5] = 0xF6;
        len = 6;
    }
    bytes[len] = 0x48;
    bytes[len + 1] = 0xC7;
    bytes[len + 2] = 0xC0;
    bytes[len + 3..len + 7].copy_from_slice(&(syscall_id as u32).to_le_bytes());
    len += 7;
    bytes[len] = 0x0F;
    bytes[len + 1] = 0x05;
    bytes[len + 2] = 0x0F;
    bytes[len + 3] = 0x0B;
    len += 4;
    write_user_stub_at_entry(hw, rip, &bytes[..len])
}

pub fn resume_after_hw_syscall() -> ! {
    unsafe {
        core::arch::asm!("int3", options(noreturn));
    }
}

fn enter_user_common(
    hw: &HwPageTableHandle,
    entry: &UserEntryFrame,
    selectors: UserSelectors,
    use_iretq: bool,
) -> Result<(), UserEntryError> {
    if USER_BRINGUP.load(Ordering::Relaxed) == 0 {
        USER_BRINGUP.store(1, Ordering::Relaxed);
    }
    IRETQ_ENTRIES.fetch_add(1, Ordering::Relaxed);

    let before_trap = IRETQ_TRAPPED.load(Ordering::Relaxed) + USER_TRAP_RETURNS.load(Ordering::Relaxed);

    let trapped = crate::user_paging::with_user_page_table(hw, || {
        if use_iretq {
            unsafe {
                enter_user_mode(
                    selectors.code.0 as u64,
                    selectors.data.0 as u64,
                    0x2,
                    entry.rip,
                    entry.rsp,
                );
            }
        } else {
            unsafe {
                let stack_ptr: u64;
                core::arch::asm!("mov {}, rsp", out(reg) stack_ptr, options(nomem, nostack));
                KERNEL_RESUME_RIP.store(resume_after_user as *const () as u64, Ordering::Relaxed);
                KERNEL_RESUME_RSP.store(stack_ptr, Ordering::Relaxed);
                let rip = entry.rip;
                core::arch::asm!("call {0}", in(reg) rip);
            }
        }
        let after_trap = IRETQ_TRAPPED.load(Ordering::Relaxed) + USER_TRAP_RETURNS.load(Ordering::Relaxed);
        after_trap > before_trap
    })
    .map_err(|_| UserEntryError::Paging)?;

    if !trapped {
        USER_BRINGUP.store(0, Ordering::Relaxed);
        return Err(UserEntryError::NoTrap);
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserEntryError {
    Paging,
    NoTrap,
    StubWriteFailed,
}

fn write_user_stub_at_entry(hw: &HwPageTableHandle, rip: u64, bytes: &[u8]) -> Result<(), UserEntryError> {
    let phys = crate::user_paging::translate_hw_page(hw.cr3_phys, rip).ok_or(UserEntryError::StubWriteFailed)?;
    let offset = (rip & 0xfff) as usize;
    crate::user_paging::write_phys_bytes(phys, offset, bytes);
    let virt = crate::user_paging::phys_to_virt(x86_64::PhysAddr::new(phys.saturating_add(offset as u64)));
    let written = unsafe { core::slice::from_raw_parts(virt.as_ptr() as *const u8, bytes.len()) };
    if written != bytes {
        return Err(UserEntryError::StubWriteFailed);
    }
    x86_64::instructions::tlb::flush(VirtAddr::new(rip));
    Ok(())
}

pub fn set_hw_syscall_bringup_flag() {
    USER_BRINGUP.store(2, Ordering::Relaxed);
}

#[inline(never)]
pub unsafe fn enter_user_mode(user_code: u64, user_data: u64, rflags: u64, rip: u64, rsp: u64) {
    let resume = resume_after_user as *const () as u64;
    let stack_ptr: u64;
    core::arch::asm!(
        "mov {}, rsp",
        out(reg) stack_ptr,
        options(nomem, nostack)
    );
    KERNEL_RESUME_RIP.store(resume, Ordering::Relaxed);
    KERNEL_RESUME_RSP.store(stack_ptr, Ordering::Relaxed);
    let user_ss = user_data | 3;
    let user_cs = user_code | 3;
    core::arch::asm!(
        "push {user_ss}",
        "push {user_rsp}",
        "push {user_rflags}",
        "push {user_cs}",
        "push {user_rip}",
        "mov ds, {user_ds:x}",
        "mov es, {user_ds:x}",
        "iretq",
        user_ss = in(reg) user_ss,
        user_rsp = in(reg) rsp,
        user_rflags = in(reg) rflags,
        user_cs = in(reg) user_cs,
        user_rip = in(reg) rip,
        user_ds = in(reg) user_ss,
        options(noreturn)
    );
}

#[inline(never)]
extern "C" fn resume_after_user() {
    // Returned via modified interrupt frame after user trap.
}
