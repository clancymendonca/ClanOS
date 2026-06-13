//! Embedded ring-3 I/O demo ELF (built by `kernel/build.rs`).

pub fn elf_bytes() -> &'static [u8] {
    include_bytes!(concat!(env!("OUT_DIR"), "/ring3_io_demo.bin"))
}
