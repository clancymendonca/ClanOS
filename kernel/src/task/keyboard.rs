//! Asynchronous keyboard input handler.
//!
//! The keyboard IRQ handler writes raw scancodes into a lock-free
//! `ArrayQueue`.  The async `print_keypresses` future drains that queue,
//! translates scancodes to key events with the `pc-keyboard` crate, and
//! prints printable characters to the VGA console.

/// Console output mirrored to VGA and serial (terminal stays usable in graphics mode).
macro_rules! con_print {
    ($($arg:tt)*) => {{
        $crate::print!($($arg)*);
        $crate::serial_print!($($arg)*);
    }};
}
macro_rules! con_println {
    () => {{
        $crate::println!();
        $crate::serial_println!();
    }};
    ($($arg:tt)*) => {{
        $crate::println!($($arg)*);
        $crate::serial_println!($($arg)*);
    }};
}
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::{
    stream::{Stream, StreamExt},
    task::AtomicWaker,
};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;

/// Maximum number of unprocessed scancodes to buffer.
const SCANCODE_QUEUE_SIZE: usize = 100;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore,)
    );
    static ref CONSOLE_LINE: Mutex<String> = Mutex::new(String::new());
}

/// Initialise the keyboard scancode queue.
///
/// Safe to call multiple times; only the first call initialises storage.
pub fn init_scancode_queue() {
    let _ = SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(SCANCODE_QUEUE_SIZE));
}

/// Called by the keyboard IRQ handler to push a raw scancode into the queue.
///
/// This function is designed to be safe to call from an interrupt context:
/// it never blocks and never allocates.
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if queue.push(scancode).is_err() {
            // Drop the scancode; the queue is full.
        } else {
            WAKER.wake();
        }
    } else {
        // Queue not yet initialised (very early boot); drop the scancode.
    }
}

/// An async `Stream` that yields raw scancodes from the IRQ handler.
pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        init_scancode_queue();
        ScancodeStream { _private: () }
    }
}

impl Default for ScancodeStream {
    fn default() -> Self {
        Self::new()
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        poll_serial_console();

        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialised");

        // Fast path: a scancode is already in the queue.
        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        // No scancode yet – register the waker so we are polled again when
        // the IRQ handler pushes a new scancode.
        WAKER.register(cx.waker());

        // Re-check after registering to avoid a TOCTOU race.
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

/// A future that reads keypresses from the keyboard and prints them.
///
/// This is the main keyboard task; spawn it with the executor at boot.
pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();

    while let Some(scancode) = scancodes.next().await {
        process_scancode(scancode, true);
    }
    con_println!("Keyboard stream ended.");
}

/// Poll queued scancodes and process keyboard-console commands.
///
/// This is used by preemption-mode context tasks where the async keyboard
/// task is not running.
pub fn poll_console_commands() {
    poll_serial_console();
    while let Some(scancode) = try_pop_scancode() {
        process_scancode(scancode, true);
    }
}

/// Drain host terminal bytes from COM1 (`-serial stdio`).
pub fn poll_serial_console() {
    while let Some(byte) = crate::serial::try_pop_byte() {
        if byte == b'\n' || byte == b'\r' {
            handle_console_char('\n', true);
        } else if let Some(ch) = char::from_u32(byte.into()) {
            if ch.is_ascii() && !ch.is_ascii_control() || ch == '\u{8}' || ch == '\u{7f}' {
                handle_console_char(ch, true);
            }
        }
    }
}

fn try_pop_scancode() -> Option<u8> {
    SCANCODE_QUEUE.try_get().ok().and_then(|queue| queue.pop())
}

fn process_scancode(scancode: u8, echo: bool) {
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => handle_console_char(character, echo),
                DecodedKey::RawKey(key) => {
                    if echo {
                        con_print!("{:?}", key);
                    }
                }
            }
        }
    }
}

fn handle_console_char(character: char, echo: bool) {
    match character {
        '\n' | '\r' => {
            if echo {
                con_println!("");
            }
            let command = {
                let mut line = CONSOLE_LINE.lock();
                let command = line.trim().to_string();
                line.clear();
                command
            };

            if command.is_empty() {
                return;
            }

            execute_console_command(&command);
        }
        '\u{8}' | '\u{7f}' => {
            let mut line = CONSOLE_LINE.lock();
            if !line.is_empty() {
                line.pop();
                if echo {
                    con_print!("\u{8} \u{8}");
                }
            }
        }
        c => {
            let mut line = CONSOLE_LINE.lock();
            line.push(c);
            if echo {
                con_print!("{}", c);
            }
        }
    }
}

fn execute_console_command(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    match parts.as_slice() {
        ["help"] => {
            con_println!("Console commands:");
            con_println!("  help");
            con_println!("  ps");
            con_println!("  kill <pid>");
            con_println!("  metrics");
            con_println!("  whoami");
            con_println!("  su <admin|user|guest>");
            con_println!("  run <program> [args...]  (mendo, ring3-io-demo, hello-alloc, demo-hello, …)");
            con_println!("  fork-run <corpus-program>");
            con_println!("  programs");
            con_println!("  bin list");
            con_println!("  bin info <program>");
            con_println!("  bin validate <program>");
            con_println!("  bin prepare <program>");
            con_println!("  bin map <program>");
            con_println!("  bin back <program>");
            con_println!("  bin pagetable <program>");
            con_println!("  bin userctx <program>");
            con_println!("  bin ring3 <program>");
            con_println!("  bin usyscall <program>");
            con_println!("  bin plans");
            con_println!("  bin mappings");
            con_println!("  frames");
            con_println!("  ls");
            con_println!("  cat <path>");
            con_println!("  touch <path>");
            con_println!("  write <path> <text>");
            con_println!("  rm <path>");
            con_println!("  stat <path>");
            con_println!("  chmod +x|-x <path>");
            con_println!("  mount");
            con_println!("  format");
            con_println!("  fsinfo");
            con_println!("  desktop");
            con_println!("  devices");
            con_println!("  blk list");
            con_println!("  blk info <id>");
            con_println!("  mount <block-id>");
            con_println!("  sched show");
            con_println!("  sched quantum <ticks>");
            con_println!("  sched fairness <ticks>");
            con_println!("  sched maxproc <count>");
        }
        ["ps"] => {
            let entries = crate::task::process::get_all_processes_with_details();
            if entries.is_empty() {
                con_println!("No processes registered");
            } else {
                con_println!("PID  STATE       CPU_TICKS  OWNER      IMAGE          LOAD       NAME");
                for (pid, name, state, ticks, owner, image, load) in entries {
                    let image_source = image
                        .as_ref()
                        .map(|image| image.source_path)
                        .unwrap_or("-");
                    let load_state = load
                        .as_ref()
                        .map(|load| match load.state {
                            crate::task::process::ProcessLoadState::Prepared => "prepared",
                            crate::task::process::ProcessLoadState::Rejected => "rejected",
                            crate::task::process::ProcessLoadState::ExecutionBlocked => "blocked",
                            crate::task::process::ProcessLoadState::MappedStub => "mapped",
                            crate::task::process::ProcessLoadState::FrameBacked => "backed",
                            crate::task::process::ProcessLoadState::PageTableReady => "ptable",
                            crate::task::process::ProcessLoadState::UserContextReady => "uctx",
                            crate::task::process::ProcessLoadState::UserTrapped => "trapped",
                            crate::task::process::ProcessLoadState::UserSyscallReturned => "sysret",
                            crate::task::process::ProcessLoadState::UserElfExited => "elf-exit",
                            crate::task::process::ProcessLoadState::HwPageTableReady => "hw-ptable",
                            crate::task::process::ProcessLoadState::Cr3Activated => "cr3-active",
                            crate::task::process::ProcessLoadState::UserEnteredHw => "hw-enter",
                            crate::task::process::ProcessLoadState::UserHwTrapped => "hw-trap",
                            crate::task::process::ProcessLoadState::UserHwSyscallReturned => "hw-syscall",
                            crate::task::process::ProcessLoadState::UserHwElfExited => "hw-elf-exit",
                            crate::task::process::ProcessLoadState::SchedCr3Bound => "sched-cr3",
                            crate::task::process::ProcessLoadState::UserFrameSaved => "user-frame",
                            crate::task::process::ProcessLoadState::ConcurrentElfReady => "multi-elf",
                            crate::task::process::ProcessLoadState::UserHwExitedSched => "hw-exit-sched",
                            crate::task::process::ProcessLoadState::ManifestElfDiscovered => "manifest-elf",
                            crate::task::process::ProcessLoadState::DynamicLinked => "dynamic",
                            crate::task::process::ProcessLoadState::SharedLibMapped => "shared-lib",
                            crate::task::process::ProcessLoadState::DynRelocApplied => "dyn-reloc",
                            crate::task::process::ProcessLoadState::TrustExecReady => "trust-exec",
                            crate::task::process::ProcessLoadState::UserPathReady => "user-path",
                            crate::task::process::ProcessLoadState::FileFdReady => "file-fd",
                            crate::task::process::ProcessLoadState::FdIoReady => "fd-io",
                            crate::task::process::ProcessLoadState::FileDemandReady => "file-demand",
                            crate::task::process::ProcessLoadState::WxPolicyReady => "wx-policy",
                            crate::task::process::ProcessLoadState::SmpReady => "smp",
                            crate::task::process::ProcessLoadState::ProcFdReady => "proc-fd",
                            crate::task::process::ProcessLoadState::FdDupReady => "fd-dup",
                            crate::task::process::ProcessLoadState::MprotectReady => "mprotect",
                            crate::task::process::ProcessLoadState::MmapReady => "mmap",
                            crate::task::process::ProcessLoadState::WritePathReady => "write-path",
                            crate::task::process::ProcessLoadState::MultiShlibReady => "multi-shlib",
                            crate::task::process::ProcessLoadState::PltRelocReady => "plt-reloc",
                            crate::task::process::ProcessLoadState::DigestTrustReady => "digest-trust",
                            crate::task::process::ProcessLoadState::RunqueueReady => "runqueue",
                            crate::task::process::ProcessLoadState::ChdirReady => "chdir",
                            crate::task::process::ProcessLoadState::MunmapReady => "munmap",
                            crate::task::process::ProcessLoadState::VmaReady => "vma",
                            crate::task::process::ProcessLoadState::ForkLiteReady => "fork-lite",
                            crate::task::process::ProcessLoadState::Ring3SyscallReady => "ring3-syscall",
                            crate::task::process::ProcessLoadState::FcntlReady => "fcntl",
                            crate::task::process::ProcessLoadState::LazyPltReady => "lazy-plt",
                            crate::task::process::ProcessLoadState::TlbShootdownReady => "tlb-shootdown",
                            crate::task::process::ProcessLoadState::ApIdleReady => "ap-idle",
                        })
                        .unwrap_or("-");
                    con_println!(
                        "{:<4} {:<11?} {:<9} {:<10} {:<14} {:<10} {}",
                        pid.as_u64(),
                        state,
                        ticks,
                        owner.role.name(),
                        image_source,
                        load_state,
                        name
                    );
                }
            }
        }
        ["kill", pid] => match parse_pid(pid) {
            Ok(raw_pid) => {
                let pid = crate::task::process::ProcessId::from_raw(raw_pid);
                if crate::task::process::terminate_process_checked(
                    crate::security::current_credentials(),
                    pid,
                    0,
                ) {
                    con_println!("Terminated PID {}", raw_pid);
                } else {
                    con_println!("PID {} not found or permission denied", raw_pid);
                }
            }
            Err(err) => con_println!("Invalid pid ({}): {}", err, pid),
        },
        ["whoami"] => {
            let credentials = crate::security::current_credentials();
            con_println!(
                "user={} role={}",
                credentials.user.as_u64(),
                credentials.role.name()
            );
        }
        ["su", role] => match *role {
            "admin" => {
                crate::security::set_current_credentials(crate::security::Credentials::admin());
                con_println!("Switched to admin");
            }
            "user" => {
                crate::security::set_current_credentials(crate::security::Credentials::shell_user());
                con_println!("Switched to user");
            }
            "guest" => {
                crate::security::set_current_credentials(crate::security::Credentials::guest());
                con_println!("Switched to guest");
            }
            _ => con_println!("Unknown role: {}", role),
        },
        ["metrics"] => {
            let scheduler = crate::task::scheduler::stats();
            let (creates, terms, preemptions, fairness_violations) =
                crate::performance::process_metrics::ProcessMetricsGlobal::global_snapshot();
            con_println!(
                "Metrics: ticks={}, req={}, points={}, preemptions={}, creates={}, terms={}, fairness_violations={}",
                scheduler.timer_ticks,
                scheduler.reschedule_requests,
                scheduler.reschedule_points,
                preemptions,
                creates,
                terms,
                fairness_violations
            );
        }
        ["run", program, args @ ..] => match crate::task::userspace::run_program(program, args) {
            Ok(output) => con_println!("{}", output),
            Err(err) => con_println!("run error: {}", err),
        },
        ["fork-run", program] => {
            let credentials = crate::security::current_credentials();
            match crate::corpus_runner::fork_run_corpus(credentials, program) {
                Ok(output) => con_println!("{}", output),
                Err(crate::task::program_loader::ProgramLoadError::NotFound) => {
                    con_println!("fork-run error: program not found")
                }
                Err(_) => con_println!("fork-run error: execution failed"),
            }
        }
        ["programs"] | ["bin", "list"] => {
            let programs = crate::task::program_loader::discover_programs();
            if programs.is_empty() {
                con_println!("No stored programs discovered");
            } else {
                for program in programs {
                    let marker = match program.kind {
                        crate::task::program_loader::ProgramKind::BuiltinAlias => "builtin",
                        crate::task::program_loader::ProgramKind::Elf64Image => "elf64-image",
                    };
                    con_println!(
                        "{} [{}] -> {} ({})",
                        program.name, marker, program.entry, program.source_path
                    );
                }
            }
            let status = crate::task::program_loader::status();
            con_println!(
                "Program loader: programs={}, images={}/{}, invalid_images={}, prepared={}, planned_pages={}, mapped={}, mapped_pages={}, launches={}, failed_launches={}",
                status.program_count,
                status.valid_image_count,
                status.image_count,
                status.invalid_image_count,
                status.prepared_image_count,
                status.total_planned_pages,
                status.mapped_image_count,
                status.total_mapped_pages,
                status.launch_count,
                status.failed_launch_count
            );
        }
        ["bin", "info", program] => match crate::task::program_loader::program_info(program) {
            Ok(info) => {
                let planned = info
                    .image
                    .as_ref()
                    .and_then(|image| crate::load_plan::build_load_plan(image).ok());
                con_println!(
                    "Program {}: path={}, kind={:?}, entry={}, image={:?}, segments={}, planned_pages={}, planned_regions={}, trust={:?}, exec_supported={}, description={}",
                    info.name,
                    info.source_path,
                    info.kind,
                    info.entry,
                    info.image_path,
                    info.image.as_ref().map(|image| image.segments.len()).unwrap_or(0),
                    planned.as_ref().map(|plan| plan.total_pages).unwrap_or(0),
                    planned.as_ref().map(|plan| plan.regions.len()).unwrap_or(0),
                    info.trust,
                    info.kind == crate::task::program_loader::ProgramKind::BuiltinAlias,
                    info.description
                );
            }
            Err(err) => con_println!("program info error: {:?}", err),
        },
        ["bin", "validate", program] => match crate::task::program_loader::validate_program_image(
            crate::security::current_credentials(),
            program,
        ) {
            Ok(image) => con_println!(
                "Program {} image valid: format={:?}, entry=0x{:x}, segments={}, source={}",
                image.name,
                image.format,
                image.entry_point,
                image.segments.len(),
                image.source_path
            ),
            Err(err) => con_println!("program validate error: {:?}", err),
        },
        ["bin", "prepare", program] => match crate::task::program_loader::prepare_program_image(
            crate::security::current_credentials(),
            program,
        ) {
            Ok(prepared) => con_println!(
                "Prepared {}: entry=0x{:x}, regions={}, pages={}, stack_pages={}",
                prepared.image.name,
                prepared.load_plan.entry_point,
                prepared.load_plan.regions.len(),
                prepared.load_plan.total_pages,
                prepared.load_plan.stack_pages
            ),
            Err(err) => con_println!("program prepare error: {:?}", err),
        },
        ["bin", "map", program] => match crate::task::program_loader::map_prepared_program(
            crate::security::current_credentials(),
            program,
        ) {
            Ok(mapped) => con_println!(
                "Mapped {}: id={}, pages={}, copied={}, zeroed={}, state={:?}",
                mapped.mapped.image_name,
                mapped.mapped.id.as_u64(),
                mapped.mapped.total_pages,
                mapped.mapped.copied_bytes,
                mapped.mapped.zero_filled_bytes,
                mapped.mapped.state
            ),
            Err(err) => con_println!("program map error: {:?}", err),
        },
        ["bin", "back", program] => match crate::task::program_loader::back_mapped_program(
            crate::security::current_credentials(),
            program,
        ) {
            Ok(backed) => con_println!(
                "Frame-backed {}: mapping={}, pages={}, copied={}, zeroed={}, state={:?}",
                backed.backed.image_name,
                backed.backed.mapping_id.as_u64(),
                backed.backed.total_pages,
                backed.backed.copied_bytes,
                backed.backed.zero_filled_bytes,
                backed.backed.state
            ),
            Err(err) => con_println!("program frame-back error: {:?}", err),
        },
        ["bin", "pagetable", program] => match crate::task::program_loader::build_user_page_table(
            crate::security::current_credentials(),
            program,
        ) {
            Ok(table) => con_println!(
                "Inactive page table {}: asid={}, pages={}, exec={}, writable={}, readonly={}, cr3_ready={}",
                table.page_table.id.as_u64(),
                table.page_table.address_space_id.as_u64(),
                table.page_table.mapped_pages,
                table.page_table.executable_pages,
                table.page_table.writable_pages,
                table.page_table.read_only_pages,
                table.page_table.cr3_switch_ready
            ),
            Err(err) => con_println!("program page-table error: {:?}", err),
        },
        ["bin", "userctx", program] => match crate::task::program_loader::prepare_user_context(
            crate::security::current_credentials(),
            program,
        ) {
            Ok(userctx) => con_println!(
                "User context: page_table={}, rip=0x{:x}, rsp=0x{:x}, cs={}, ss={}, ring3_entered={}",
                userctx.context.page_table_id.as_u64(),
                userctx.context.entry.rip,
                userctx.context.entry.rsp,
                userctx.context.entry.code_selector,
                userctx.context.entry.stack_selector,
                userctx.context.ring3_entered
            ),
            Err(err) => con_println!("program user-context error: {:?}", err),
        },
        ["bin", "ring3", program] => match crate::task::program_loader::enter_controlled_ring3_trampoline(
            crate::security::current_credentials(),
            program,
        ) {
            Ok(entry) => con_println!(
                "Ring3 trampoline: rip=0x{:x}, rsp=0x{:x}, trap_vector={}, entered={}, trapped={}",
                entry.result.entry_rip,
                entry.result.user_rsp,
                entry.result.trap_vector,
                entry.result.ring3_entered,
                entry.result.trapped_back
            ),
            Err(err) => con_println!("program ring3 error: {:?}", err),
        },
        ["bin", "usyscall", program] => match crate::task::program_loader::run_user_syscall_probe(
            crate::security::current_credentials(),
            program,
        ) {
            Ok(probe) => con_println!(
                "User syscall: id={}, return={}, error={:?}, returned={}",
                probe.syscall_return.syscall_id,
                probe.syscall_return.return_value,
                probe.syscall_return.error,
                probe.syscall_return.returned_to_user
            ),
            Err(err) => con_println!("program user-syscall error: {:?}", err),
        },
        ["bin", "plans"] | ["loadplans"] => {
            let status = crate::task::program_loader::status();
            con_println!(
                "Load plans: prepared={}, rejected={}, planned_pages={}, mapped={}, mapped_pages={}, backed={}, backed_pages={}, page_tables={}, ptable_pages={}, user_contexts={}, ring3_entries={}, traps={}, user_syscalls={}, returns={}, elf_exec={}, elf_exits={}, exec_blocked={}",
                status.prepared_image_count,
                status.rejected_load_plan_count,
                status.total_planned_pages,
                status.mapped_image_count,
                status.total_mapped_pages,
                status.frame_backed_image_count,
                status.total_frame_backed_pages,
                status.user_page_table_count,
                status.total_user_page_table_pages,
                status.user_context_count,
                status.ring3_entry_count,
                status.ring3_trap_count,
                status.user_syscall_count,
                status.user_syscall_return_count,
                status.user_elf_execution_count,
                status.user_elf_exit_count,
                status.unsupported_execution_count
            );
        }
        ["bin", "mappings"] => {
            for mapping in crate::mapping_stub::list_mappings() {
                con_println!(
                    "Mapping {}: image={}, asid={}, pages={}, copied={}, zeroed={}, state={:?}",
                    mapping.id.as_u64(),
                    mapping.image_name,
                    mapping.address_space_id.as_u64(),
                    mapping.total_pages,
                    mapping.copied_bytes,
                    mapping.zero_filled_bytes,
                    mapping.state
                );
            }
        }
        ["frames"] => {
            let status = crate::frame_ownership::status();
            con_println!(
                "Frames: initialized={}, tracked={}, available={}, allocated={}, allocations={}, releases={}, failures={}",
                status.initialized,
                status.tracked_frames,
                status.available_frames,
                status.allocated_frames,
                status.allocation_count,
                status.release_count,
                status.failed_allocation_count
            );
        }
        ["ls"] => match crate::storage::list_files() {
            Ok(files) => {
                for file in files {
                    con_println!("{}", file);
                }
            }
            Err(err) => con_println!("ls error: {}", err),
        },
        ["cat", path] => {
            let creds = crate::security::current_credentials();
            match crate::vfs::read_bytes_for(creds, path) {
                Ok(Some(bytes)) => {
                    let text = core::str::from_utf8(&bytes).unwrap_or("<binary>");
                    con_println!("{}", text);
                }
                Ok(None) => con_println!("No such file: {}", path),
                Err(_) => con_println!("cat error: read failed"),
            }
        }
        ["touch", path] => match crate::storage::create_file_checked(
            crate::security::current_credentials(),
            path,
        ) {
            Ok(()) => con_println!("Created {}", path),
            Err(crate::storage::StorageError::AlreadyExists) => con_println!("File already exists: {}", path),
            Err(err) => con_println!("touch error: {}", err),
        },
        ["write", path, contents @ ..] if !contents.is_empty() => {
            let text = join_parts(contents);
            match crate::storage::write_file_checked(
                crate::security::current_credentials(),
                path,
                &text,
            ) {
                Ok(()) => con_println!("Wrote {}", path),
                Err(err) => con_println!("write error: {}", err),
            }
        }
        ["write", ..] => con_println!("Usage: write <path> <text>"),
        ["rm", path] => match crate::storage::delete_file_checked(
            crate::security::current_credentials(),
            path,
        ) {
            Ok(()) => con_println!("Removed {}", path),
            Err(err) => con_println!("rm error: {}", err),
        },
        ["stat", path] => match crate::storage::stat_file(path) {
            Ok(Some(metadata)) => con_println!(
                "File {}: owner={}, mode={:03b}, len={}",
                metadata.path,
                metadata.owner.as_u64(),
                metadata.mode.bits(),
                metadata.len
            ),
            Ok(None) => con_println!("No such file: {}", path),
            Err(err) => con_println!("stat error: {}", err),
        },
        ["chmod", flag, path] => match *flag {
            "+x" => match crate::storage::chmod_execute_checked(
                crate::security::current_credentials(),
                path,
                true,
            ) {
                Ok(()) => con_println!("Enabled execute on {}", path),
                Err(err) => con_println!("chmod error: {}", err),
            },
            "-x" => match crate::storage::chmod_execute_checked(
                crate::security::current_credentials(),
                path,
                false,
            ) {
                Ok(()) => con_println!("Disabled execute on {}", path),
                Err(err) => con_println!("chmod error: {}", err),
            },
            _ => con_println!("Usage: chmod +x|-x <path>"),
        },
        ["mount"] => match crate::storage::remount() {
            Ok(()) => con_println!("Storage mounted"),
            Err(err) => con_println!("mount error: {}", err),
        },
        ["mount", block_id] => match parse_block_id(block_id) {
            Ok(id) => match crate::storage::mount_block_device(id) {
                Ok(()) => con_println!("Mounted block device {}", id),
                Err(err) => con_println!("mount error: {}", err),
            },
            Err(err) => con_println!("Invalid block id ({}): {}", err, block_id),
        },
        ["format"] => match crate::storage::format() {
            Ok(()) => con_println!("Storage formatted"),
            Err(err) => con_println!("format error: {}", err),
        },
        ["fsinfo"] => match crate::storage::info() {
            Ok(info) => con_println!(
                "FS: mounted={}, files={}/{}, free_slots={}, capacity_bytes={}, max_file_size={}, backend={}, driver_backed={}",
                info.mounted,
                info.file_count,
                info.max_files,
                info.free_slots,
                info.capacity_bytes,
                info.max_file_size,
                info.backend_name,
                info.driver_backed
            ),
            Err(err) => con_println!("fsinfo error: {}", err),
        },
        ["desktop"] => {
            crate::framebuffer::render_desktop_frame();
            con_println!("Desktop frame rendered (320x200 mode 13h).");
        }
        ["devices"] => {
            let summary = crate::device::summary();
            con_println!(
                "Devices: total={}, pci={}, block={}, storage={}",
                summary.total, summary.pci, summary.block, summary.storage
            );
            for device in crate::device::list_devices() {
                con_println!(
                    "  id={} kind={:?} state={:?} name={} vendor={:?} device={:?} class={:?} subclass={:?}",
                    device.id.as_u64(),
                    device.kind,
                    device.state,
                    device.name,
                    device.vendor_id,
                    device.device_id,
                    device.class_code,
                    device.subclass
                );
            }
        }
        ["blk", "list"] => {
            for device in crate::block::list_block_devices() {
                con_println!(
                    "  id={} name={} backend={:?} sectors={} sector_size={} readonly={} driver_backed={}",
                    device.id.as_u64(),
                    device.name,
                    device.backend,
                    device.sector_count,
                    device.sector_size,
                    device.read_only,
                    device.driver_backed
                );
            }
        }
        ["blk", "info", block_id] => match parse_block_id(block_id) {
            Ok(id) => {
                let found = crate::block::list_block_devices()
                    .into_iter()
                    .find(|device| device.id.as_u64() == id);
                match found {
                    Some(device) => con_println!(
                        "Block {}: name={}, backend={:?}, sectors={}, sector_size={}, readonly={}, driver_backed={}",
                        id,
                        device.name,
                        device.backend,
                        device.sector_count,
                        device.sector_size,
                        device.read_only,
                        device.driver_backed
                    ),
                    None => con_println!("Block device {} not found", id),
                }
            }
            Err(err) => con_println!("Invalid block id ({}): {}", err, block_id),
        },
        ["sched", "show"] => {
            let config = crate::task::scheduler::runtime_config();
            con_println!(
                "Scheduler config: quantum_ticks={}, fairness_check_ticks={}, max_processes={}",
                config.quantum_ticks, config.fairness_check_interval_ticks, config.max_processes
            );
        }
        ["sched", "quantum", value] => match value.parse::<u64>() {
            Ok(ticks) => {
                let mut config = crate::task::scheduler::runtime_config();
                config.quantum_ticks = ticks;
                match crate::task::scheduler::apply_runtime_config(config) {
                    Ok(_) => con_println!("Updated scheduler quantum to {} ticks", config.quantum_ticks),
                    Err(err) => con_println!("Rejected scheduler update: {:?}", err),
                }
            }
            Err(_) => con_println!("Invalid quantum value: {}", value),
        },
        ["sched", "fairness", value] => match value.parse::<u64>() {
            Ok(ticks) => {
                let mut config = crate::task::scheduler::runtime_config();
                config.fairness_check_interval_ticks = ticks;
                match crate::task::scheduler::apply_runtime_config(config) {
                    Ok(_) => con_println!(
                        "Updated fairness check interval to {} ticks",
                        config.fairness_check_interval_ticks
                    ),
                    Err(err) => con_println!("Rejected scheduler update: {:?}", err),
                }
            }
            Err(_) => con_println!("Invalid fairness value: {}", value),
        },
        ["sched", "maxproc", value] => match value.parse::<usize>() {
            Ok(max_proc) => {
                let mut config = crate::task::scheduler::runtime_config();
                config.max_processes = max_proc;
                match crate::task::scheduler::apply_runtime_config(config) {
                    Ok(_) => con_println!("Updated max processes to {}", config.max_processes),
                    Err(err) => con_println!("Rejected scheduler update: {:?}", err),
                }
            }
            Err(_) => con_println!("Invalid maxproc value: {}", value),
        },
        _ => {
            con_println!("Unknown command: {}", command);
            con_println!("Type 'help' for available commands");
        }
    }
}

fn parse_pid(value: &str) -> Result<u64, &'static str> {
    let pid = value.parse::<u64>().map_err(|_| "not-a-number")?;
    if pid == 0 {
        return Err("reserved-pid");
    }
    Ok(pid)
}

fn parse_block_id(value: &str) -> Result<u64, &'static str> {
    let id = value.parse::<u64>().map_err(|_| "not-a-number")?;
    if id == 0 {
        return Err("reserved-id");
    }
    Ok(id)
}

fn join_parts(parts: &[&str]) -> String {
    let mut out = String::new();
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            out.push(' ');
        }
        out.push_str(part);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::parse_pid;

    #[test_case]
    fn parse_pid_rejects_reserved_pid_zero() {
        assert_eq!(parse_pid("0"), Err("reserved-pid"));
    }

    #[test_case]
    fn parse_pid_rejects_non_numeric() {
        assert_eq!(parse_pid("abc"), Err("not-a-number"));
    }

    #[test_case]
    fn parse_pid_accepts_positive_ids() {
        assert_eq!(parse_pid("42"), Ok(42));
    }

    #[test_case]
    fn parse_block_id_rejects_zero() {
        assert_eq!(super::parse_block_id("0"), Err("reserved-id"));
    }

    #[test_case]
    fn join_parts_preserves_spaces_between_words() {
        assert_eq!(super::join_parts(&["hello", "scope", "7"]), "hello scope 7");
    }

    #[test_case]
    fn scheduler_console_updates_apply() {
        let baseline = crate::task::scheduler::SchedulerRuntimeConfig {
            quantum_ticks: 5,
            fairness_check_interval_ticks: 10,
            max_processes: 256,
        };
        let _ = crate::task::scheduler::apply_runtime_config(baseline);

        super::execute_console_command("sched quantum 7");
        super::execute_console_command("sched fairness 13");
        super::execute_console_command("sched maxproc 321");

        let config = crate::task::scheduler::runtime_config();
        assert_eq!(config.quantum_ticks, 7);
        assert_eq!(config.fairness_check_interval_ticks, 13);
        assert_eq!(config.max_processes, 321);
    }

    #[test_case]
    fn scheduler_console_invalid_update_rolls_back() {
        let baseline = crate::task::scheduler::SchedulerRuntimeConfig {
            quantum_ticks: 6,
            fairness_check_interval_ticks: 12,
            max_processes: 256,
        };
        let _ = crate::task::scheduler::apply_runtime_config(baseline);

        super::execute_console_command("sched quantum 0");
        super::execute_console_command("sched maxproc 0");

        let config = crate::task::scheduler::runtime_config();
        assert_eq!(config, baseline);
    }
}
