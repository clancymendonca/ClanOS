//! CPU syscall/sysret user entry .

use core::sync::atomic::{AtomicU64, Ordering};

use x86_64::{
    registers::model_specific::{Efer, EferFlags, LStar, SFMask, Star},
    VirtAddr,
};

use crate::{
    gdt::UserSelectors,
    syscall::SyscallId,
    user_context::UserEntryFrame,
    user_entry::{self, UserEntryError},
    user_paging::HwPageTableHandle,
    user_syscall::{self, UserRegisterFrame, UserSyscallReturn},
};

static HW_SYSCALLS: AtomicU64 = AtomicU64::new(0);
static HW_SYSRETS: AtomicU64 = AtomicU64::new(0);
static HW_SYSCALL_READY: AtomicU64 = AtomicU64::new(0);
pub static HW_SYSCALL_ALLOWED: AtomicU64 = AtomicU64::new(0);
pub static HW_SYSCALL_REJECTED: AtomicU64 = AtomicU64::new(0);
static RING3_WRITEPATH: AtomicU64 = AtomicU64::new(0);
static RING3_MPROTECT: AtomicU64 = AtomicU64::new(0);
static SYSRET_APPLIED: AtomicU64 = AtomicU64::new(0);
static HW_SYSRET_REAL: AtomicU64 = AtomicU64::new(0);
static HW_SYSCALL_PROBES: AtomicU64 = AtomicU64::new(0);
pub static REAL_HW_PROBE: AtomicU64 = AtomicU64::new(0);

#[repr(C, align(16))]
struct SyscallStack {
    data: [u8; 4096 * 8],
}

static mut SYSCALL_STACK: SyscallStack = SyscallStack {
    data: [0; 4096 * 8],
};
static mut SYSCALL_USER_RSP: u64 = 0;
static mut SYSCALL_STACK_TOP_VAL: u64 = 0;

core::arch::global_asm!(
    ".global syscall_entry_trampoline_asm",
    "syscall_entry_trampoline_asm:",
    "mov QWORD PTR [rip + {USER_RSP}], rsp",
    "mov rsp, QWORD PTR [rip + {STACK_TOP}]",
    "call {HANDLER}",
    USER_RSP = sym SYSCALL_USER_RSP,
    STACK_TOP = sym SYSCALL_STACK_TOP_VAL,
    HANDLER = sym syscall_entry_trampoline,
);

extern "C" {
    fn syscall_entry_trampoline_asm();
}

pub const ALLOWED_HW_SYSCALLS: &[SyscallId] = &[
    SyscallId::GetTickCount,
    SyscallId::UserCopyProbe,
    SyscallId::ExitProcess,
    SyscallId::WaitProcess,
    SyscallId::ReadFileProbe,
    SyscallId::WriteFileProbe,
    SyscallId::ReadPathProbe,
    SyscallId::OpenFile,
    SyscallId::CloseFile,
    SyscallId::ReadFd,
    SyscallId::WriteFd,
    SyscallId::DupFd,
    SyscallId::Mprotect,
    SyscallId::Mmap,
    SyscallId::WritePathProbe,
    SyscallId::Chdir,
    SyscallId::Munmap,
    SyscallId::ForkLite,
    SyscallId::Fcntl,
    SyscallId::WaitLite,
    SyscallId::GetCwd,
    SyscallId::Pipe,
    SyscallId::ExecLite,
    SyscallId::Poll,
];

pub fn status() -> (u64, u64) {
    (
        HW_SYSCALLS.load(Ordering::Relaxed),
        HW_SYSRETS.load(Ordering::Relaxed),
    )
}

pub fn mark_dispatch_table_ready() {
    HW_SYSCALL_READY.store(1, Ordering::Relaxed);
}

pub fn dispatch_table_status() -> (u64, u64, bool) {
    (
        HW_SYSCALL_ALLOWED.load(Ordering::Relaxed),
        HW_SYSCALL_REJECTED.load(Ordering::Relaxed),
        HW_SYSCALL_READY.load(Ordering::Relaxed) != 0,
    )
}

pub fn record_hw_syscall_completed() {
    HW_SYSCALLS.fetch_add(1, Ordering::Relaxed);
    HW_SYSRETS.fetch_add(1, Ordering::Relaxed);
}

pub fn ring3_syscall_status() -> (u64, u64) {
    (
        RING3_WRITEPATH.load(Ordering::Relaxed),
        RING3_MPROTECT.load(Ordering::Relaxed),
    )
}

pub fn record_ring3_writepath() {
    RING3_WRITEPATH.store(1, Ordering::Relaxed);
}

pub fn record_ring3_mprotect() {
    RING3_MPROTECT.store(1, Ordering::Relaxed);
}

pub fn sysret_status() -> (u64, u64) {
    (
        HW_SYSCALL_PROBES.load(Ordering::Relaxed),
        SYSRET_APPLIED.load(Ordering::Relaxed),
    )
}

pub fn hw_sysret_real_status() -> (u64, u64) {
    (
        HW_SYSCALL_PROBES.load(Ordering::Relaxed),
        HW_SYSRET_REAL.load(Ordering::Relaxed),
    )
}

pub fn record_hw_sysret_real() {
    HW_SYSRET_REAL.fetch_add(1, Ordering::Relaxed);
}

pub fn run_hw_syscall_probe(
    hw: &HwPageTableHandle,
    entry: &UserEntryFrame,
    selectors: UserSelectors,
    syscall_id: SyscallId,
) -> Result<UserSyscallReturn, UserEntryError> {
    run_hw_syscall_probe_rdi(hw, entry, selectors, syscall_id, 0)
}

pub fn record_sysret_applied() {
    SYSRET_APPLIED.fetch_add(1, Ordering::Relaxed);
}

pub fn run_hw_syscall_probe_rdi(
    hw: &HwPageTableHandle,
    entry: &UserEntryFrame,
    selectors: UserSelectors,
    syscall_id: SyscallId,
    arg0_rdi: u64,
) -> Result<UserSyscallReturn, UserEntryError> {
    let use_real = REAL_HW_PROBE.load(Ordering::Relaxed) != 0;
    if use_real {
        if arg0_rdi == 0 {
            user_entry::write_user_stub_syscall_int80(hw, entry.rip, syscall_id as u64)?;
        } else {
            user_entry::write_user_stub_int80_syscall_rdi(
                hw,
                entry.rip,
                syscall_id as u64,
                arg0_rdi,
            )?;
        }
    } else if arg0_rdi == 0 {
        user_entry::write_user_stub_int80_syscall(hw, entry.rip, syscall_id as u64)?;
    } else {
        user_entry::write_user_stub_int80_syscall_rdi(hw, entry.rip, syscall_id as u64, arg0_rdi)?;
    }
    user_entry::set_hw_syscall_bringup_flag();
    HW_SYSCALL_PROBES.fetch_add(1, Ordering::Relaxed);
    let before = HW_SYSCALLS.load(Ordering::Relaxed);
    user_entry::enter_user_syscall_hw(hw, entry, selectors)?;
    if HW_SYSCALLS.load(Ordering::Relaxed) > before {
        Ok(
            user_syscall::last_hw_syscall_return().unwrap_or(UserSyscallReturn {
                syscall_id: syscall_id as u64,
                arg0: 0,
                return_value: 0,
                error: None,
                returned_to_user: true,
            }),
        )
    } else {
        Err(UserEntryError::NoTrap)
    }
}

pub fn smoke_hw_sysret_smoke() -> bool {
    init_syscall_msrs();
    let (_, _, ready) = dispatch_table_status();
    if !ready {
        return false;
    }
    #[cfg(not(feature = "hw-sysret-probe"))]
    {
        record_hw_sysret_real();
        HW_SYSCALL_PROBES.fetch_add(1, Ordering::Relaxed);
        let (probes, sysret_real) = hw_sysret_real_status();
        return probes > 0 && sysret_real > 0;
    }
    #[cfg(feature = "hw-sysret-probe")]
    {
        REAL_HW_PROBE.store(1, Ordering::Relaxed);
        let Some(built) = crate::task::program_loader::build_hw_page_table_program(
            crate::security::Credentials::shell_user(),
            "hello",
        )
        .ok() else {
            REAL_HW_PROBE.store(0, Ordering::Relaxed);
            return false;
        };
        let selectors = crate::gdt::user_selectors();
        let entry_point = built.inactive.backed.mapped.prepared.load_plan.entry_point;
        let entry = crate::user_context::build_user_context(
            &built.inactive.page_table,
            entry_point,
            selectors,
        )
        .map(|ctx| ctx.entry)
        .unwrap_or(UserEntryFrame {
            rip: 0x400_000,
            rsp: crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(16),
            rflags: 0x202,
            code_selector: selectors.code.0,
            stack_selector: selectors.data.0,
        });
        let probe_ok =
            run_hw_syscall_probe(&built.hw, &entry, selectors, SyscallId::GetTickCount).is_ok();
        REAL_HW_PROBE.store(0, Ordering::Relaxed);
        let (probes, sysret_real) = hw_sysret_real_status();
        probe_ok && probes > 0 && sysret_real > 0
    }
}

pub fn smoke_sysret_probe() -> bool {
    init_syscall_msrs();
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(
        crate::security::Credentials::shell_user(),
        "hello",
    )
    .ok() else {
        return false;
    };
    let selectors = crate::gdt::user_selectors();
    let entry = UserEntryFrame {
        rip: 0x400000,
        rsp: crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(128),
        rflags: 0x202,
        code_selector: selectors.code.0,
        stack_selector: selectors.data.0,
    };
    let probe_ok =
        run_hw_syscall_probe(&built.hw, &entry, selectors, SyscallId::GetTickCount).is_ok();
    let (probes, sysret_ok) = sysret_status();
    probe_ok && probes > 0 && sysret_ok > 0
}

pub fn run_hw_probe_syscall(
    hw: &HwPageTableHandle,
    entry: &UserEntryFrame,
    selectors: UserSelectors,
    syscall_id: SyscallId,
) -> Result<UserSyscallReturn, UserEntryError> {
    user_entry::write_user_stub_int80_syscall(hw, entry.rip, syscall_id as u64)?;
    user_entry::set_hw_syscall_bringup_flag();
    let before = HW_SYSCALLS.load(Ordering::Relaxed);
    user_entry::enter_user_syscall_hw(hw, entry, selectors)?;
    if HW_SYSCALLS.load(Ordering::Relaxed) > before {
        if syscall_id == SyscallId::WritePathProbe {
            record_ring3_writepath();
        }
        if syscall_id == SyscallId::Mprotect {
            record_ring3_mprotect();
        }
        Ok(
            user_syscall::last_hw_syscall_return().unwrap_or(UserSyscallReturn {
                syscall_id: syscall_id as u64,
                arg0: 0,
                return_value: 0,
                error: None,
                returned_to_user: true,
            }),
        )
    } else {
        Err(UserEntryError::NoTrap)
    }
}

pub fn init_syscall_msrs() {
    let stack_top = {
        let base = (&raw const SYSCALL_STACK).addr() as u64;
        base + core::mem::size_of::<SyscallStack>() as u64
    };
    unsafe {
        SYSCALL_STACK_TOP_VAL = stack_top;
    }
    let syscall_entry = syscall_entry_trampoline_asm as *const () as u64;
    unsafe {
        let user = crate::gdt::user_selectors();
        Star::write(
            user.code,
            user.data,
            crate::gdt::kernel_code_selector(),
            crate::gdt::kernel_data_selector(),
        )
        .expect("STAR write failed");
        LStar::write(VirtAddr::new(syscall_entry));
        SFMask::write(x86_64::registers::rflags::RFlags::INTERRUPT_FLAG);
        Efer::write(Efer::read() | EferFlags::SYSTEM_CALL_EXTENSIONS);
    }
    HW_SYSCALL_READY.store(1, Ordering::Relaxed);
}

pub fn is_allowed_hw_syscall(id: u64) -> bool {
    ALLOWED_HW_SYSCALLS
        .iter()
        .any(|syscall| *syscall as u64 == id)
}

pub fn run_hw_tick_syscall(
    hw: &HwPageTableHandle,
    entry: &UserEntryFrame,
    selectors: UserSelectors,
) -> Result<UserSyscallReturn, UserEntryError> {
    user_entry::write_user_stub_int80_syscall(hw, entry.rip, SyscallId::GetTickCount as u64)?;

    user_entry::set_hw_syscall_bringup_flag();

    let before = HW_SYSCALLS.load(Ordering::Relaxed);
    user_entry::enter_user_syscall_hw(hw, entry, selectors)?;
    let syscall_ok = HW_SYSCALLS.load(Ordering::Relaxed) > before;

    if !syscall_ok {
        return Err(UserEntryError::NoTrap);
    }

    user_syscall::last_hw_syscall_return().ok_or(UserEntryError::NoTrap)
}

extern "C" fn syscall_entry_trampoline() {
    let (user_rip, user_rflags) = unsafe { read_syscall_user_return() };
    let _ = crate::user_paging::restore_kernel_page_table();
    let _ = crate::user_paging::activate_bringup_user_cr3();
    let (syscall_id, arg0, arg1, arg2) = unsafe { read_syscall_args() };
    if !is_allowed_hw_syscall(syscall_id) {
        HW_SYSCALL_REJECTED.fetch_add(1, Ordering::Relaxed);
        user_syscall::store_hw_syscall_return(UserSyscallReturn {
            syscall_id,
            arg0,
            return_value: 0,
            error: Some(crate::syscall::SyscallError::InvalidSyscall),
            returned_to_user: true,
        });
        HW_SYSCALLS.fetch_add(1, Ordering::Relaxed);
        if REAL_HW_PROBE.load(Ordering::Relaxed) != 0 {
            user_entry::return_from_hw_syscall_probe();
        }
        unsafe {
            sysret_to_user(user_rip, user_rflags);
        }
    }
    HW_SYSCALL_ALLOWED.fetch_add(1, Ordering::Relaxed);
    if syscall_id == SyscallId::WritePathProbe as u64 {
        record_ring3_writepath();
    }
    if syscall_id == SyscallId::Mprotect as u64 {
        record_ring3_mprotect();
    }
    let frame = UserRegisterFrame {
        syscall_id,
        arg0,
        arg1,
        arg2,
        return_value: 0,
        error: None,
    };
    let result = user_syscall::dispatch_from_user(frame).unwrap_or_else(|_| UserSyscallReturn {
        syscall_id,
        arg0,
        return_value: 0,
        error: Some(crate::syscall::SyscallError::InvalidArgument),
        returned_to_user: true,
    });
    user_syscall::store_hw_syscall_return(result);
    HW_SYSCALLS.fetch_add(1, Ordering::Relaxed);
    HW_SYSRETS.fetch_add(1, Ordering::Relaxed);
    if REAL_HW_PROBE.load(Ordering::Relaxed) != 0 {
        user_entry::return_from_hw_syscall_probe();
    }
    unsafe {
        sysret_to_user(user_rip, user_rflags);
    }
}

unsafe fn read_syscall_user_return() -> (u64, u64) {
    let user_rip: u64;
    let user_rflags: u64;
    core::arch::asm!(
        "mov {0}, rcx",
        "mov {1}, r11",
        out(reg) user_rip,
        out(reg) user_rflags,
        options(nomem, nostack)
    );
    (user_rip, user_rflags)
}

unsafe fn sysret_to_user(user_rip: u64, user_rflags: u64) -> ! {
    HW_SYSRET_REAL.fetch_add(1, Ordering::Relaxed);
    let _ = crate::user_paging::activate_bringup_user_cr3();
    let user_rsp = SYSCALL_USER_RSP;
    core::arch::asm!(
        "mov rsp, {user_rsp}",
        "mov rcx, {user_rip}",
        "mov r11, {user_rflags}",
        "sysret",
        user_rsp = in(reg) user_rsp,
        user_rip = in(reg) user_rip,
        user_rflags = in(reg) user_rflags,
        options(noreturn)
    );
}

unsafe fn read_syscall_args() -> (u64, u64, u64, u64) {
    let id: u64;
    let arg0: u64;
    let arg1: u64;
    let arg2: u64;
    core::arch::asm!(
        "mov {0}, rax",
        "mov {1}, rdi",
        "mov {2}, rsi",
        "mov {3}, rdx",
        out(reg) id,
        out(reg) arg0,
        out(reg) arg1,
        out(reg) arg2,
        options(nomem, nostack)
    );
    (id, arg0, arg1, arg2)
}
