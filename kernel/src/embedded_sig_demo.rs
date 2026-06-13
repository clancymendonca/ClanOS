//! Embedded sig-demo ring-3 ELF (built by `kernel/build.rs`).

pub fn elf_bytes() -> &'static [u8] {
    include_bytes!(concat!(env!("OUT_DIR"), "/sig_demo.bin"))
}
