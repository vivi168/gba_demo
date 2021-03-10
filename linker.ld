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
    } > rom = 0xff

    .data : ALIGN(4) {
        *(.data*)
    } > iwram AT > rom = 0xff

    .bss : ALIGN(4) {
        *(.bss*);
    } > iwram = 0xff

    /DISCARD/ : { *(*) }
}
