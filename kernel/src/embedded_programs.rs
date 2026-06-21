//! Embedded ring-3 corpus ELFs (built by `kernel/build.rs`).

pub const CORPUS_PROGRAMS: &[&str] = &["mendo", "ring3-io-demo", "hello-alloc", "sig-demo"];

pub fn is_corpus_program(name: &str) -> bool {
    CORPUS_PROGRAMS.contains(&name)
}

pub fn elf_bytes(name: &str) -> Option<&'static [u8]> {
    match name {
        "mendo" => Some(crate::embedded_mendo::elf_bytes()),
        "ring3-io-demo" => Some(crate::embedded_ring3_io_demo::elf_bytes()),
        "hello-alloc" => Some(crate::embedded_hello_alloc::elf_bytes()),
        "sig-demo" => Some(crate::embedded_sig_demo::elf_bytes()),
        _ => None,
    }
}
