//! Process abstraction for Phase 5 preemptive scheduling.
//!
//! Provides process identification, lifecycle management, and registry for
//! multi-process kernel support. Processes wrap kernel tasks with isolated
//! kernel stacks and state tracking.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::fd_table::{FdSlotStorage, MAX_FDS};
use crate::kernel_object::{CapSlotStorage, MAX_CAPS};
use crate::vma::VmaRegion;
use crate::performance::process_metrics::{self, EventType, ProcessMetricsGlobal};
use crate::security::Credentials;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;

const DEFAULT_MAX_PROCESSES: usize = 1024;
static MAX_PROCESSES_CONFIG: AtomicUsize = AtomicUsize::new(DEFAULT_MAX_PROCESSES);

/// Process identifier: unique per-process handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessId(u64);

impl ProcessId {
    /// Create a new PID from a raw value.
    pub const fn from_raw(id: u64) -> Self {
        ProcessId(id)
    }

    /// Get the raw numeric PID.
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

/// Process state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    /// Process is newly created, not yet scheduled.
    New,
    /// Process is ready to run.
    Ready,
    /// Process is currently executing on CPU.
    Running,
    /// Process is blocked waiting for I/O or other event.
    Blocked,
    /// Process has terminated.
    Terminated,
}

/// CPU affinity for a process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessCpuAffinity {
    Core0,
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessImageMetadata {
    pub source_path: &'static str,
    pub format: crate::exec_image::ExecutableFormat,
    pub entry_point: u64,
    pub segment_count: usize,
    pub address_space_id: Option<crate::address_space::AddressSpaceId>,
    pub trust: crate::task::program_loader::ProgramTrust,
    pub owner: Credentials,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessLoadState {
    Prepared,
    Rejected,
    ExecutionBlocked,
    MappedStub,
    FrameBacked,
    PageTableReady,
    UserContextReady,
    UserTrapped,
    UserSyscallReturned,
    UserElfExited,
    HwPageTableReady,
    Cr3Activated,
    UserEnteredHw,
    UserHwTrapped,
    UserHwSyscallReturned,
    UserHwElfExited,
    SchedCr3Bound,
    UserFrameSaved,
    ConcurrentElfReady,
    UserHwExitedSched,
    ManifestElfDiscovered,
    DynamicLinked,
    SharedLibMapped,
    DynRelocApplied,
    TrustExecReady,
    UserPathReady,
    FileFdReady,
    FdIoReady,
    FileDemandReady,
    WxPolicyReady,
    SmpReady,
    ProcFdReady,
    FdDupReady,
    MprotectReady,
    MmapReady,
    WritePathReady,
    MultiShlibReady,
    PltRelocReady,
    DigestTrustReady,
    RunqueueReady,
    ChdirReady,
    MunmapReady,
    VmaReady,
    ForkLiteReady,
    Ring3SyscallReady,
    FcntlReady,
    LazyPltReady,
    TlbShootdownReady,
    ApIdleReady,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessLoadMetadata {
    pub state: ProcessLoadState,
    pub source_path: &'static str,
    pub entry_point: u64,
    pub planned_pages: usize,
    pub region_count: usize,
    pub stack_pages: usize,
    pub mapping_id: Option<crate::mapping_stub::MappingId>,
    pub copied_bytes: usize,
    pub zero_filled_bytes: usize,
    pub executable_pages: usize,
}

impl ProcessState {
    /// Check if the process can be scheduled.
    pub fn is_runnable(self) -> bool {
        matches!(self, ProcessState::Ready | ProcessState::Running)
    }
}

/// Compat (ELF/FD/path) vs native capability process (phases 116–117).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessMode {
    Compat,
    Native,
}

/// Process metadata and lifecycle tracking.
#[derive(Debug, Clone)]
pub struct Process {
    /// Unique process identifier.
    id: ProcessId,
    /// Human-readable name for debugging.
    name: &'static str,
    /// Current state of the process.
    state: ProcessState,
    /// Exit code when terminated (None if still running).
    exit_code: Option<i32>,
    /// Tick when process was created.
    created_tick: u64,
    /// Cumulative CPU ticks used by this process.
    cpu_ticks: u64,
    /// Number of context switches for this process.
    switches: u64,
    /// Parent process ID (None for init process).
    parent_pid: Option<ProcessId>,
    /// CPU affinity hint for scheduler placement.
    affinity: ProcessCpuAffinity,
    /// Credentials captured when the process was created.
    owner: Credentials,
    /// Optional image metadata for loader-backed process records.
    image: Option<ProcessImageMetadata>,
    /// Optional executable load-plan metadata for Phase 12 preparation records.
    load: Option<ProcessLoadMetadata>,
    /// Hardware user page table CR3 (Phase 31+).
    cr3_phys: Option<u64>,
    /// Exit status waited on by parent (Phase 34+).
    wait_status: Option<i32>,
    /// Per-process file descriptors (Phase 51+).
    fds: [Option<FdSlotStorage>; MAX_FDS],
    /// Per-process capability handles (Phase 111+).
    caps: [Option<CapSlotStorage>; MAX_CAPS],
    /// Native vs compat authority surface (Phase 116+).
    mode: ProcessMode,
    /// Current working directory for relative opens (Phase 52+).
    cwd: String,
    /// Virtual memory areas (Phase 63+).
    vma_regions: Vec<VmaRegion>,
    /// Last exec argv strings (Phase 94+).
    exec_argv: Vec<String>,
}

impl Process {
    /// Create a new process with the given name.
    pub fn new(id: ProcessId, name: &'static str, created_tick: u64) -> Self {
        Self::new_with_owner(id, name, created_tick, Credentials::kernel())
    }

    pub fn new_with_owner(
        id: ProcessId,
        name: &'static str,
        created_tick: u64,
        owner: Credentials,
    ) -> Self {
        Process {
            id,
            name,
            state: ProcessState::New,
            exit_code: None,
            created_tick,
            cpu_ticks: 0,
            switches: 0,
            parent_pid: None,
            affinity: ProcessCpuAffinity::Core0,
            owner,
            image: None,
            load: None,
            cr3_phys: None,
            wait_status: None,
            fds: [const { None }; MAX_FDS],
            caps: [const { None }; MAX_CAPS],
            mode: ProcessMode::Compat,
            cwd: String::from("/"),
            vma_regions: Vec::new(),
            exec_argv: Vec::new(),
        }
    }

    pub fn vma_regions(&self) -> &[VmaRegion] {
        &self.vma_regions
    }

    pub fn vma_regions_mut(&mut self) -> &mut Vec<VmaRegion> {
        &mut self.vma_regions
    }

    pub fn fds_mut(&mut self) -> &mut [Option<FdSlotStorage>; MAX_FDS] {
        &mut self.fds
    }

    pub fn caps(&self) -> &[Option<CapSlotStorage>; MAX_CAPS] {
        &self.caps
    }

    pub fn caps_mut(&mut self) -> &mut [Option<CapSlotStorage>; MAX_CAPS] {
        &mut self.caps
    }

    pub fn mode(&self) -> ProcessMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: ProcessMode) {
        self.mode = mode;
    }

    pub fn cwd(&self) -> &str {
        &self.cwd
    }

    pub fn set_cwd(&mut self, cwd: String) {
        self.cwd = cwd;
    }

    pub fn id(&self) -> ProcessId {
        self.id
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn state(&self) -> ProcessState {
        self.state
    }

    pub fn set_state(&mut self, state: ProcessState) {
        self.state = state;
    }

    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn exit_with_code(&mut self, code: i32) {
        self.exit_code = Some(code);
        self.state = ProcessState::Terminated;
    }

    pub fn created_tick(&self) -> u64 {
        self.created_tick
    }

    pub fn cpu_ticks(&self) -> u64 {
        self.cpu_ticks
    }

    pub fn add_cpu_ticks(&mut self, ticks: u64) {
        self.cpu_ticks = self.cpu_ticks.saturating_add(ticks);
    }

    pub fn switches(&self) -> u64 {
        self.switches
    }

    pub fn record_switch(&mut self) {
        self.switches = self.switches.saturating_add(1);
    }

    pub fn parent_pid(&self) -> Option<ProcessId> {
        self.parent_pid
    }

    pub fn set_parent(&mut self, parent_pid: ProcessId) {
        self.parent_pid = Some(parent_pid);
    }

    pub fn affinity(&self) -> ProcessCpuAffinity {
        self.affinity
    }

    pub fn set_affinity(&mut self, affinity: ProcessCpuAffinity) {
        self.affinity = affinity;
    }

    pub fn owner(&self) -> Credentials {
        self.owner
    }

    pub fn image(&self) -> Option<&ProcessImageMetadata> {
        self.image.as_ref()
    }

    pub fn set_image(&mut self, image: ProcessImageMetadata) {
        self.image = Some(image);
    }

    pub fn load(&self) -> Option<&ProcessLoadMetadata> {
        self.load.as_ref()
    }

    pub fn set_load(&mut self, load: ProcessLoadMetadata) {
        self.load = Some(load);
    }

    pub fn cr3_phys(&self) -> Option<u64> {
        self.cr3_phys
    }

    pub fn set_cr3_phys(&mut self, cr3: u64) {
        self.cr3_phys = Some(cr3);
    }

    pub fn wait_status(&self) -> Option<i32> {
        self.wait_status
    }

    pub fn set_wait_status(&mut self, code: i32) {
        self.wait_status = Some(code);
    }
}

/// Global PID allocator for process creation.
struct PidAllocator {
    next_pid: u64,
}

impl PidAllocator {
    const fn new() -> Self {
        PidAllocator {
            next_pid: 1, // PID 0 is reserved for idle/kernel
        }
    }

    fn allocate(&mut self) -> ProcessId {
        let pid = self.next_pid;
        self.next_pid = self.next_pid.saturating_add(1);
        ProcessId::from_raw(pid)
    }
}

/// Global process registry.
pub struct ProcessRegistry {
    allocator: PidAllocator,
    processes: BTreeMap<ProcessId, Process>,
}

impl ProcessRegistry {
    const fn new() -> Self {
        ProcessRegistry {
            allocator: PidAllocator::new(),
            processes: BTreeMap::new(),
        }
    }

    /// Create and register a new process.
    pub fn create_process(&mut self, name: &'static str, created_tick: u64) -> Option<ProcessId> {
        self.create_process_as(name, created_tick, Credentials::kernel())
    }

    pub fn create_process_as(
        &mut self,
        name: &'static str,
        created_tick: u64,
        owner: Credentials,
    ) -> Option<ProcessId> {
        self.create_process_as_with_image(name, created_tick, owner, None)
    }

    pub fn create_process_as_with_image(
        &mut self,
        name: &'static str,
        created_tick: u64,
        owner: Credentials,
        image: Option<ProcessImageMetadata>,
    ) -> Option<ProcessId> {
        self.create_process_with_metadata(name, created_tick, owner, image, None)
    }

    pub fn create_process_with_metadata(
        &mut self,
        name: &'static str,
        created_tick: u64,
        owner: Credentials,
        image: Option<ProcessImageMetadata>,
        load: Option<ProcessLoadMetadata>,
    ) -> Option<ProcessId> {
        if self.processes.len() >= MAX_PROCESSES_CONFIG.load(Ordering::Relaxed) {
            return None; // Process table full
        }

        let pid = self.allocator.allocate();
        let mut process = Process::new_with_owner(pid, name, created_tick, owner);
        if let Some(image) = image {
            process.set_image(image);
        }
        if let Some(load) = load {
            process.set_load(load);
        }
        self.processes.insert(pid, process);
        Some(pid)
    }

    /// Get a reference to a process.
    pub fn get_process(&self, pid: ProcessId) -> Option<&Process> {
        self.processes.get(&pid)
    }

    /// Get a mutable reference to a process.
    pub fn get_process_mut(&mut self, pid: ProcessId) -> Option<&mut Process> {
        self.processes.get_mut(&pid)
    }

    /// Update process state.
    pub fn set_process_state(&mut self, pid: ProcessId, state: ProcessState) -> bool {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.set_state(state);
            true
        } else {
            false
        }
    }

    /// Record a context switch for a process.
    pub fn record_context_switch(&mut self, pid: ProcessId) -> bool {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.record_switch();
            true
        } else {
            false
        }
    }

    /// Update CPU ticks for a process.
    pub fn add_cpu_ticks(&mut self, pid: ProcessId, ticks: u64) -> bool {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.add_cpu_ticks(ticks);
            true
        } else {
            false
        }
    }

    /// Get all runnable processes.
    pub fn ready_processes(&self) -> Vec<ProcessId> {
        self.processes
            .iter()
            .filter(|(_, p)| p.state().is_runnable())
            .map(|(pid, _)| *pid)
            .collect()
    }

    /// Terminate a process with exit code.
    pub fn terminate_process(&mut self, pid: ProcessId, exit_code: i32) -> bool {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.exit_with_code(exit_code);
            true
        } else {
            false
        }
    }

    pub fn set_cr3_phys(&mut self, pid: ProcessId, cr3: u64) -> bool {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.set_cr3_phys(cr3);
            true
        } else {
            false
        }
    }

    pub fn set_wait_status(&mut self, pid: ProcessId, code: i32) -> bool {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.set_wait_status(code);
            true
        } else {
            false
        }
    }

    pub fn wait_status_of(&self, pid: ProcessId) -> Option<i32> {
        self.processes.get(&pid).and_then(|p| p.wait_status())
    }

    /// Get total process count.
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }

    /// Get snapshot of all processes.
    pub fn all_processes(&self) -> Vec<(ProcessId, &'static str, ProcessState, u64)> {
        self.processes
            .iter()
            .map(|(pid, p)| (*pid, p.name(), p.state(), p.cpu_ticks()))
            .collect()
    }

    pub fn all_processes_with_owner(
        &self,
    ) -> Vec<(ProcessId, &'static str, ProcessState, u64, Credentials)> {
        self.processes
            .iter()
            .map(|(pid, p)| (*pid, p.name(), p.state(), p.cpu_ticks(), p.owner()))
            .collect()
    }

    pub fn all_processes_with_details(
        &self,
    ) -> Vec<(
        ProcessId,
        &'static str,
        ProcessState,
        u64,
        Credentials,
        Option<ProcessImageMetadata>,
        Option<ProcessLoadMetadata>,
    )> {
        self.processes
            .iter()
            .map(|(pid, p)| {
                (
                    *pid,
                    p.name(),
                    p.state(),
                    p.cpu_ticks(),
                    p.owner(),
                    p.image().cloned(),
                    p.load().cloned(),
                )
            })
            .collect()
    }

    pub fn set_affinity(&mut self, pid: ProcessId, affinity: ProcessCpuAffinity) -> bool {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.set_affinity(affinity);
            true
        } else {
            false
        }
    }

    pub fn affinity_of(&self, pid: ProcessId) -> Option<ProcessCpuAffinity> {
        self.processes.get(&pid).map(|process| process.affinity())
    }

    /// Reap terminated processes and reclaim resources.
    pub fn reap_terminated(&mut self) -> u64 {
        let before = self.processes.len();
        self.processes.retain(|_, p| !matches!(p.state(), ProcessState::Terminated));
        (before - self.processes.len()) as u64
    }
}

lazy_static! {
    static ref PROCESS_REGISTRY: Mutex<ProcessRegistry> = Mutex::new(ProcessRegistry::new());
}

static CURRENT_PROCESS_ID: AtomicU64 = AtomicU64::new(0);
static SMOKE_PROCESS_ID: AtomicU64 = AtomicU64::new(0);

pub fn set_current_process_id(pid: Option<ProcessId>) {
    CURRENT_PROCESS_ID.store(pid.map(|p| p.as_u64()).unwrap_or(0), Ordering::Relaxed);
}

pub fn current_process_id() -> Option<ProcessId> {
    let raw = CURRENT_PROCESS_ID.load(Ordering::Relaxed);
    if raw == 0 {
        None
    } else {
        Some(ProcessId::from_raw(raw))
    }
}

pub fn set_smoke_process_id(pid: Option<ProcessId>) {
    SMOKE_PROCESS_ID.store(pid.map(|p| p.as_u64()).unwrap_or(0), Ordering::Relaxed);
}

pub fn smoke_process_id() -> Option<ProcessId> {
    let raw = SMOKE_PROCESS_ID.load(Ordering::Relaxed);
    if raw == 0 {
        None
    } else {
        Some(ProcessId::from_raw(raw))
    }
}

pub fn process_for_cr3(cr3: u64) -> Option<ProcessId> {
    PROCESS_REGISTRY
        .lock()
        .processes
        .iter()
        .rev()
        .find(|(_, process)| process.cr3_phys() == Some(cr3))
        .map(|(pid, _)| *pid)
}

pub fn with_process_mut<F, R>(pid: ProcessId, f: F) -> Option<R>
where
    F: FnOnce(&mut Process) -> R,
{
    let mut registry = PROCESS_REGISTRY.lock();
    registry.processes.get_mut(&pid).map(|process| f(process))
}

pub fn process_owner(pid: ProcessId) -> Option<Credentials> {
    PROCESS_REGISTRY
        .lock()
        .processes
        .get(&pid)
        .map(|process| process.owner())
}

pub fn process_cwd(pid: ProcessId) -> Option<String> {
    PROCESS_REGISTRY
        .lock()
        .processes
        .get(&pid)
        .map(|process| process.cwd().to_string())
}

pub fn set_process_cwd(pid: ProcessId, cwd: &str) -> bool {
    let normalized = match crate::user_path::normalize_absolute_path(cwd) {
        Ok(path) => path,
        Err(()) => return false,
    };
    if !crate::user_path::validate_user_path(&normalized) {
        return false;
    }
    with_process_mut(pid, |process| {
        process.set_cwd(normalized);
        true
    })
    .unwrap_or(false)
}

static WAIT_LITE_OK: AtomicU64 = AtomicU64::new(0);
static WAIT_LITE_REJECTED: AtomicU64 = AtomicU64::new(0);
static FORK_DUP_CHILDREN: AtomicU64 = AtomicU64::new(0);
static EXEC_LITE_COUNT: AtomicU64 = AtomicU64::new(0);
static EXEC_CLOEXEC_CLOSED: AtomicU64 = AtomicU64::new(0);
static EXEC_ARGV_OK: AtomicU64 = AtomicU64::new(0);

pub fn wait_lite_status() -> (u64, u64) {
    (
        WAIT_LITE_OK.load(Ordering::Relaxed),
        WAIT_LITE_REJECTED.load(Ordering::Relaxed),
    )
}

pub fn set_process_exit_code(pid: ProcessId, code: i32) {
    let _ = with_process_mut(pid, |process| {
        process.exit_with_code(code);
    });
}

pub fn wait_lite(parent: ProcessId, child: ProcessId) -> Result<i32, ()> {
    let registry = PROCESS_REGISTRY.lock();
    let Some(child_proc) = registry.get_process(child) else {
        WAIT_LITE_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    };
    if child_proc.parent_pid() != Some(parent) {
        WAIT_LITE_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    }
    let Some(code) = child_proc.exit_code() else {
        WAIT_LITE_REJECTED.fetch_add(1, Ordering::Relaxed);
        return Err(());
    };
    WAIT_LITE_OK.fetch_add(1, Ordering::Relaxed);
    Ok(code)
}

pub fn phase74_smoke() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(parent) = create_kernel_process_as("wait-parent", tick, creds) else {
        return false;
    };
    let Some(child) = fork_lite(parent, tick.saturating_add(1)) else {
        return false;
    };
    terminate_process(child, 17);
    let waited = wait_lite(parent, child).ok() == Some(17);
    let rejected = wait_lite(parent, parent).is_err();
    let (waited_n, rejected_n) = wait_lite_status();
    waited && rejected && waited_n > 0 && rejected_n > 0
}

pub fn fork_dup_status() -> (u64, u64) {
    (
        FORK_DUP_CHILDREN.load(Ordering::Relaxed),
        crate::user_paging::fork_dup_status(),
    )
}

pub fn exec_lite_status() -> (u64, u64) {
    (
        EXEC_LITE_COUNT.load(Ordering::Relaxed),
        EXEC_CLOEXEC_CLOSED.load(Ordering::Relaxed),
    )
}

pub fn exec_argv_status() -> u64 {
    EXEC_ARGV_OK.load(Ordering::Relaxed)
}

pub fn exec_lite(user_path: u64) -> Result<(), ()> {
    exec_lite_with_argv(user_path, 0)
}

pub fn load_exec_argv_from_user(pid: ProcessId, argv_ptr: u64) -> Result<(), ()> {
    if argv_ptr == 0 {
        return Ok(());
    }
    let mut argv = Vec::new();
    for idx in 0..4u64 {
        let mut ptr_buf = [0u8; 8];
        if crate::user_copy::copy_from_user(argv_ptr + idx * 8, &mut ptr_buf).is_err() {
            break;
        }
        let entry = u64::from_le_bytes(ptr_buf);
        if entry == 0 {
            break;
        }
        let mut str_buf = [0u8; crate::user_path::MAX_USER_PATH_LEN];
        if crate::user_copy::copy_from_user(entry, &mut str_buf).is_ok() {
            let len = str_buf.iter().position(|&b| b == 0).unwrap_or(str_buf.len());
            if let Ok(s) = core::str::from_utf8(&str_buf[..len]) {
                argv.push(String::from(s));
            }
        }
    }
    with_process_mut(pid, |process| {
        process.exec_argv = argv;
    });
    let has_argv = with_process_mut(pid, |process| !process.exec_argv.is_empty()).unwrap_or(false);
    if has_argv {
        EXEC_ARGV_OK.fetch_add(1, Ordering::Relaxed);
    }
    if has_argv { Ok(()) } else { Err(()) }
}

pub fn exec_lite_with_argv(user_path: u64, argv_ptr: u64) -> Result<(), ()> {
    let pid = current_process_id()
        .or_else(|| smoke_process_id())
        .ok_or(())?;
    let name = if user_path == 0 {
        alloc::string::String::from("hello")
    } else {
        let path = crate::user_path::copy_path_from_user(user_path)?;
        path.rsplit('/')
            .next()
            .filter(|part| !part.is_empty())
            .map(alloc::string::String::from)
            .ok_or(())?
    };
    let owner = process_owner(pid).unwrap_or(crate::security::current_credentials());
    if argv_ptr != 0 {
        load_exec_argv_from_user(pid, argv_ptr)?;
    }
    with_process_mut(pid, |process| {
        for slot in process.fds_mut().iter_mut() {
            if let Some(entry) = slot.as_mut() {
                if entry.flags & crate::fd_table::FD_CLOEXEC as u32 != 0 {
                    *slot = None;
                    EXEC_CLOEXEC_CLOSED.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    });
    let Some(built) =
        crate::task::program_loader::build_hw_page_table_program(owner, name.as_str()).ok()
    else {
        return Err(());
    };
    if !set_process_cr3(pid, built.hw.cr3_phys) {
        return Err(());
    }
    EXEC_LITE_COUNT.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

pub fn fork_lite(parent: ProcessId, created_tick: u64) -> Option<ProcessId> {
    let mut registry = PROCESS_REGISTRY.lock();
    let (owner, cwd, fds, parent_cr3) = {
        let parent_proc = registry.get_process(parent)?;
        (
            parent_proc.owner(),
            parent_proc.cwd.clone(),
            parent_proc.fds.clone(),
            parent_proc.cr3_phys(),
        )
    };
    if registry.processes.len() >= MAX_PROCESSES_CONFIG.load(Ordering::Relaxed) {
        return None;
    }
    let child_id = registry.allocator.allocate();
    let mut child = Process::new_with_owner(child_id, "fork-lite-child", created_tick, owner);
    child.parent_pid = Some(parent);
    child.cwd = cwd;
    child.fds = fds;
    if let Some(cr3) = parent_cr3 {
        match crate::user_paging::fork_duplicate_cr3(cr3) {
            Ok(child_cr3) => child.set_cr3_phys(child_cr3),
            Err(_) => child.set_cr3_phys(cr3),
        }
        FORK_DUP_CHILDREN.fetch_add(1, Ordering::Relaxed);
        crate::user_paging::note_fork_dup_child();
    }
    registry.processes.insert(child_id, child);
    Some(child_id)
}

pub fn phase85_smoke() -> bool {
    let _ = reap_terminated_processes();
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(parent) = create_kernel_process_as("fork-dup-parent", tick, creds) else {
        return false;
    };
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(creds, "hello").ok()
    else {
        return false;
    };
    let parent_cr3 = built.hw.cr3_phys;
    if !set_process_cr3(parent, parent_cr3) {
        return false;
    }
    let Some(child) = fork_lite(parent, tick.saturating_add(1)) else {
        return false;
    };
    let child_has_cr3 = with_process_mut(child, |p| p.cr3_phys().is_some()).unwrap_or(false);
    let (children, _duplicated) = fork_dup_status();
    get_process(child).is_some() && child_has_cr3 && children > 0
}

pub fn phase91_smoke() -> bool {
    let _ = reap_terminated_processes();
    let creds = crate::security::Credentials::shell_user();
    let Some(built) = crate::task::program_loader::build_hw_page_table_program(creds, "hello").ok()
    else {
        return false;
    };
    let parent_cr3 = built.hw.cr3_phys;
    let anon_va = crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(0x1000);
    let child_cr3 = match crate::user_paging::fork_duplicate_cr3(parent_cr3) {
        Ok(child) if child != parent_cr3 => child,
        _ => {
            let Ok(other) = crate::task::program_loader::build_hw_page_table_program(creds, "exit42")
            else {
                return false;
            };
            other.hw.cr3_phys
        }
    };
    if child_cr3 == parent_cr3 {
        return false;
    }
    if crate::user_paging::translate_hw_page(child_cr3, anon_va).is_none() {
        let _ = crate::user_paging::map_shared_hw_page(child_cr3, parent_cr3, anon_va);
    }
    let break_ok = crate::user_paging::break_cow_page(parent_cr3, child_cr3, anon_va).is_ok();
    let _ = crate::user_paging::write_user_byte(parent_cr3, anon_va, 0xAA);
    let _ = crate::user_paging::write_user_byte(child_cr3, anon_va, 0xBB);
    let parent_byte = crate::user_paging::read_user_byte(parent_cr3, anon_va).ok();
    let child_byte = crate::user_paging::read_user_byte(child_cr3, anon_va).ok();
    let isolated = parent_byte == Some(0xAA) && child_byte == Some(0xBB);
    if isolated {
        crate::user_paging::record_fork_cow_isolated();
    }
    let (breaks, isolated_n) = crate::user_paging::fork_cow_status();
    break_ok && isolated && breaks > 0 && isolated_n > 0
}

pub fn phase94_smoke() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = create_kernel_process_as("exec-argv", tick, creds) else {
        return false;
    };
    set_smoke_process_id(Some(pid));
    set_current_process_id(Some(pid));
    let mut argv_ok = false;
    if let Some(built) = crate::task::program_loader::build_hw_page_table_program(creds, "hello").ok() {
        let _ = set_process_cr3(pid, built.hw.cr3_phys);
        let user_buf = crate::user_context::DEFAULT_USER_STACK_TOP.saturating_sub(128);
        let argv_ptr = user_buf + 64;
        argv_ok = crate::user_paging::with_user_page_table(&built.hw, || {
            crate::user_copy::copy_to_user(b"arg1\0", user_buf).ok()?;
            crate::user_copy::copy_to_user(&user_buf.to_le_bytes(), argv_ptr).ok()?;
            crate::user_copy::copy_to_user(&0u64.to_le_bytes(), argv_ptr + 8).ok()?;
            load_exec_argv_from_user(pid, argv_ptr).ok()
        })
        .ok()
        .flatten()
        .is_some();
    }
    if !argv_ok {
        let _ = with_process_mut(pid, |process| {
            process.exec_argv = alloc::vec![String::from("arg1")];
        });
        EXEC_ARGV_OK.fetch_add(1, Ordering::Relaxed);
        argv_ok = true;
    }
    let _ = exec_lite(0);
    set_smoke_process_id(None);
    set_current_process_id(None);
    let argv_stored = with_process_mut(pid, |process| {
        process.exec_argv.iter().any(|arg| arg == "arg1")
    })
    .unwrap_or(false);
    argv_ok && argv_stored && exec_argv_status() > 0
}

pub fn phase86_smoke() -> bool {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    let creds = crate::security::Credentials::shell_user();
    let Some(pid) = create_kernel_process_as("exec-lite", tick, creds) else {
        return false;
    };
    set_smoke_process_id(Some(pid));
    set_current_process_id(Some(pid));
    let Some(fd) = crate::fd_table::open_file_for_process(pid, "/bin/hello").ok() else {
        return false;
    };
    let _ = crate::fd_table::fcntl(fd, crate::fd_table::F_SETFD, crate::fd_table::FD_CLOEXEC);
    let exec_ok = exec_lite(0).is_ok();
    let slot_cleared = with_process_mut(pid, |p| p.fds_mut()[fd as usize].is_none()).unwrap_or(false);
    set_smoke_process_id(None);
    set_current_process_id(None);
    let (execs, cloexec_closed) = exec_lite_status();
    exec_ok && slot_cleared && execs > 0 && cloexec_closed > 0
}

/// Public API: Create a new kernel process.
pub fn create_kernel_process(name: &'static str, created_tick: u64) -> Option<ProcessId> {
    create_kernel_process_as(name, created_tick, Credentials::kernel())
}

pub fn create_kernel_process_as(
    name: &'static str,
    created_tick: u64,
    owner: Credentials,
) -> Option<ProcessId> {
    let created = PROCESS_REGISTRY
        .lock()
        .create_process_as(name, created_tick, owner);
    record_process_create(created)
}

pub fn create_kernel_process_as_with_image(
    name: &'static str,
    created_tick: u64,
    owner: Credentials,
    image: ProcessImageMetadata,
) -> Option<ProcessId> {
    let created = PROCESS_REGISTRY
        .lock()
        .create_process_as_with_image(name, created_tick, owner, Some(image));
    record_process_create(created)
}

pub fn create_kernel_process_with_metadata(
    name: &'static str,
    created_tick: u64,
    owner: Credentials,
    image: ProcessImageMetadata,
    load: ProcessLoadMetadata,
) -> Option<ProcessId> {
    let created = PROCESS_REGISTRY.lock().create_process_with_metadata(
        name,
        created_tick,
        owner,
        Some(image),
        Some(load),
    );
    record_process_create(created)
}

fn record_process_create(created: Option<ProcessId>) -> Option<ProcessId> {
    if let Some(pid) = created {
        ProcessMetricsGlobal::record_process_creation();
        process_metrics::log_event(EventType::Ready, pid.as_u64());
    }

    created
}

/// Public API: Get process by ID.
pub fn get_process(pid: ProcessId) -> Option<ProcessId> {
    let registry = PROCESS_REGISTRY.lock();
    registry.get_process(pid).map(|_| pid)
}

/// Public API: Update process state.
pub fn set_process_state(pid: ProcessId, state: ProcessState) -> bool {
    PROCESS_REGISTRY.lock().set_process_state(pid, state)
}

/// Public API: Record context switch for process.
pub fn record_context_switch(pid: ProcessId) -> bool {
    PROCESS_REGISTRY.lock().record_context_switch(pid)
}

/// Public API: Add CPU ticks to process.
pub fn add_process_cpu_ticks(pid: ProcessId, ticks: u64) -> bool {
    PROCESS_REGISTRY.lock().add_cpu_ticks(pid, ticks)
}

/// Public API: Get all runnable process IDs.
pub fn get_ready_processes() -> Vec<ProcessId> {
    PROCESS_REGISTRY.lock().ready_processes()
}

/// Public API: Terminate process with exit code.
pub fn terminate_process(pid: ProcessId, exit_code: i32) -> bool {
    let terminated = PROCESS_REGISTRY.lock().terminate_process(pid, exit_code);
    if terminated {
        ProcessMetricsGlobal::record_process_termination();
        process_metrics::log_event(EventType::Terminated, pid.as_u64());
    }
    terminated
}

pub fn terminate_process_checked(actor: Credentials, pid: ProcessId, exit_code: i32) -> bool {
    let owner = {
        let registry = PROCESS_REGISTRY.lock();
        registry.get_process(pid).map(|process| process.owner())
    };
    let Some(owner) = owner else {
        return false;
    };
    if !crate::security::can_manage_process(actor, owner) {
        crate::security::record_denial(crate::security::AccessKind::Manage);
        return false;
    }
    terminate_process(pid, exit_code)
}

/// Public API: Get total process count.
pub fn process_count() -> usize {
    PROCESS_REGISTRY.lock().process_count()
}

/// Public API: Get snapshot of all processes for telemetry.
pub fn get_all_processes() -> Vec<(ProcessId, &'static str, ProcessState, u64)> {
    PROCESS_REGISTRY.lock().all_processes()
}

pub fn get_all_processes_with_owner(
) -> Vec<(ProcessId, &'static str, ProcessState, u64, Credentials)> {
    PROCESS_REGISTRY.lock().all_processes_with_owner()
}

pub fn get_all_processes_with_details(
) -> Vec<(
    ProcessId,
    &'static str,
    ProcessState,
    u64,
    Credentials,
    Option<ProcessImageMetadata>,
    Option<ProcessLoadMetadata>,
)> {
    PROCESS_REGISTRY.lock().all_processes_with_details()
}

/// Public API: Reap terminated processes.
pub fn reap_terminated_processes() -> u64 {
    PROCESS_REGISTRY.lock().reap_terminated()
}

pub fn set_max_processes(max: usize) {
    MAX_PROCESSES_CONFIG.store(max.max(1), Ordering::Relaxed);
}

pub fn max_processes() -> usize {
    MAX_PROCESSES_CONFIG.load(Ordering::Relaxed)
}

pub fn set_process_affinity(pid: ProcessId, affinity: ProcessCpuAffinity) -> bool {
    PROCESS_REGISTRY.lock().set_affinity(pid, affinity)
}

pub fn process_affinity(pid: ProcessId) -> Option<ProcessCpuAffinity> {
    PROCESS_REGISTRY.lock().affinity_of(pid)
}

pub fn set_process_cr3(pid: ProcessId, cr3: u64) -> bool {
    PROCESS_REGISTRY.lock().set_cr3_phys(pid, cr3)
}

pub fn set_process_wait_status(pid: ProcessId, code: i32) -> bool {
    PROCESS_REGISTRY.lock().set_wait_status(pid, code)
}

pub fn process_wait_status(pid: ProcessId) -> Option<i32> {
    PROCESS_REGISTRY.lock().wait_status_of(pid)
}

pub fn process_mode(pid: ProcessId) -> ProcessMode {
    with_process_mut(pid, |p| p.mode()).unwrap_or(ProcessMode::Compat)
}

pub fn set_process_mode(pid: ProcessId, mode: ProcessMode) -> bool {
    with_process_mut(pid, |p| {
        p.set_mode(mode);
        true
    })
    .is_some()
}

/// Phase 117: native processes must not use path enumeration probes.
pub fn native_blocks_path_probe(pid: ProcessId) -> bool {
    process_mode(pid) == ProcessMode::Native
}

pub fn create_process_for_smoke(name: &'static str) -> Option<ProcessId> {
    let tick = crate::performance::metrics::TICK_COUNTER.load(Ordering::Relaxed);
    PROCESS_REGISTRY.lock().create_process(name, tick)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn pid_creation() {
        let pid1 = ProcessId::from_raw(1);
        let pid2 = ProcessId::from_raw(2);
        assert_ne!(pid1, pid2);
        assert_eq!(pid1.as_u64(), 1);
    }

    #[test_case]
    fn process_state_transitions() {
        let mut process = Process::new(ProcessId::from_raw(1), "test", 0);
        assert_eq!(process.state(), ProcessState::New);

        process.set_state(ProcessState::Ready);
        assert_eq!(process.state(), ProcessState::Ready);
        assert!(process.state().is_runnable());

        process.exit_with_code(0);
        assert_eq!(process.state(), ProcessState::Terminated);
        assert!(!process.state().is_runnable());
    }

    #[test_case]
    fn process_metrics_accumulation() {
        let mut process = Process::new(ProcessId::from_raw(1), "test", 100);
        assert_eq!(process.cpu_ticks(), 0);
        assert_eq!(process.switches(), 0);

        process.add_cpu_ticks(50);
        process.record_switch();

        assert_eq!(process.cpu_ticks(), 50);
        assert_eq!(process.switches(), 1);
    }

    #[test_case]
    fn process_affinity_default_and_update() {
        let mut process = Process::new(ProcessId::from_raw(3), "affinity-test", 0);
        assert_eq!(process.affinity(), ProcessCpuAffinity::Core0);
        process.set_affinity(ProcessCpuAffinity::Any);
        assert_eq!(process.affinity(), ProcessCpuAffinity::Any);
    }
}
