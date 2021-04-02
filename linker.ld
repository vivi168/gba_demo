ENTRY(_start)

MEMORY {
    ewram (w!x) : ORIGIN = 0x2000000, LENGTH = 256K
    iwram (w!x) : ORIGIN = 0x3000000, LENGTH = 32K
    rom (rx)    : ORIGIN = 0x8000000, LENGTH = 32M
}

SECTIONS {
    .text : ALIGN(4) {
        KEEP(target/crt0.o(.text))
        *(.text*)
        *(.rodata*)
    } > rom

    .data : ALIGN(4) {
        *(.data*)
    } > iwram AT > rom

    .bss : ALIGN(4) {
        *(.bss*);
    } > iwram

    .exram : ALIGN(4) {
        *(.exram*);
    } > ewram AT > rom

    /DISCARD/ : { *(*) }
}
