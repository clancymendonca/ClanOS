//! Run embedded ring-3 corpus ELFs from the interactive shell.

use alloc::{format, string::String, vec::Vec};
use core::sync::atomic::{AtomicU64, Ordering};

use spin::Mutex;

use crate::task::program_loader::{ProgramLoadError, UserElfExecution};
use crate::task::process::ProcessId;

static CORPUS_OUTPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static CORPUS_EXECS: AtomicU64 = AtomicU64::new(0);

pub fn corpus_exec_count() -> u64 {
    CORPUS_EXECS.load(Ordering::Relaxed)
}

pub fn append_corpus_output(bytes: &[u8]) {
    let mut out = CORPUS_OUTPUT.lock();
    out.extend_from_slice(bytes);
}

pub fn take_corpus_output() -> Vec<u8> {
    core::mem::take(&mut *CORPUS_OUTPUT.lock())
}

pub fn is_corpus_program(name: &str) -> bool {
    crate::embedded_programs::is_corpus_program(name) || name == "ring3-io-demo-ext2"
}

pub fn execute_corpus(
    credentials: crate::security::Credentials,
    name: &str,
) -> Result<UserElfExecution, ProgramLoadError> {
    if name == "ring3-io-demo-ext2" {
        return execute_ext2_corpus(credentials, name);
    }
    let bytes = crate::embedded_programs::elf_bytes(name).ok_or(ProgramLoadError::NotFound)?;
    execute_corpus_bytes(credentials, name, bytes)
}

pub fn fork_run_corpus(
    credentials: crate::security::Credentials,
    name: &str,
) -> Result<String, ProgramLoadError> {
    let tick =
        crate::performance::metrics::TICK_COUNTER.load(core::sync::atomic::Ordering::Relaxed);
    let parent = crate::task::process::create_kernel_process_as("fork-parent", tick, credentials)
        .ok_or(ProgramLoadError::UnsupportedExecution)?;
    let child = crate::task::process::fork_lite(parent, tick.saturating_add(1))
        .ok_or(ProgramLoadError::UnsupportedExecution)?;
    run_in_process(child, || execute_corpus(credentials, name)).map(|execution| {
        format!(
            "fork-run child={} exit={}\n{}",
            child.as_u64(),
            execution.exit_code,
            execution.output
        )
    })
}

fn execute_ext2_corpus(
    credentials: crate::security::Credentials,
    name: &str,
) -> Result<UserElfExecution, ProgramLoadError> {
    if !crate::ext2::is_mounted() {
        return Err(ProgramLoadError::Storage);
    }
    let bytes = crate::ext2::read_file("ring3-io-demo.elf").map_err(|_| ProgramLoadError::Storage)?;
    execute_corpus_bytes(credentials, name, &bytes)
}

fn corpus_process_name(name: &str) -> &'static str {
    match name {
        "mendo" => "mendo",
        "ring3-io-demo" => "ring3-io-demo",
        "hello-alloc" => "hello-alloc",
        "sig-demo" => "sig-demo",
        "ring3-io-demo-ext2" => "ring3-io-ext2",
        _ => "corpus-run",
    }
}

fn execute_corpus_bytes(
    credentials: crate::security::Credentials,
    name: &str,
    bytes: &[u8],
) -> Result<UserElfExecution, ProgramLoadError> {
    if let Ok(program) = crate::task::program_loader::resolve_program(name) {
        crate::task::program_loader::verify_system_signed_elf_payload(&program, bytes)?;
    }
    CORPUS_OUTPUT.lock().clear();
    let tick =
        crate::performance::metrics::TICK_COUNTER.load(core::sync::atomic::Ordering::Relaxed);
    let pid = crate::task::process::create_kernel_process_as(
        corpus_process_name(name),
        tick,
        credentials,
    )
        .ok_or(ProgramLoadError::UnsupportedExecution)?;
    run_in_process(pid, || {
        crate::task::program_loader::execute_corpus_elf(credentials, name, bytes, pid)
    })
    .map(|execution| {
        CORPUS_EXECS.fetch_add(1, Ordering::Relaxed);
        execution
    })
}

fn run_in_process<R>(pid: ProcessId, f: impl FnOnce() -> R) -> R {
    crate::task::process::set_current_process_id(Some(pid));
    crate::task::process::set_smoke_process_id(Some(pid));
    crate::fd_table::install_standard_fds(pid);
    let result = f();
    crate::task::process::set_current_process_id(None);
    crate::task::process::set_smoke_process_id(None);
    result
}

pub fn smoke_corpus_mendo() -> bool {
    execute_corpus(
        crate::security::Credentials::shell_user(),
        "mendo",
    )
    .map(|e| e.exit_code == 0 && e.output.contains("mendo"))
    .unwrap_or(false)
}
