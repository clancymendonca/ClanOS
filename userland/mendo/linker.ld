/* Match Clan OS user ELF load base (see storage.rs /bin/hello fixtures). */
ENTRY(_start)

SECTIONS
{
    . = 0x400000;

    .text : ALIGN(4096) {
        KEEP(*(.text._start))
        *(.text .text.*)
    }

    .rodata : ALIGN(4096) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4096) {
        *(.data .data.*)
    }

    .bss : ALIGN(4096) {
        *(.bss .bss.*)
        *(COMMON)
    }
}
