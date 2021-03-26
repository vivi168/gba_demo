CPU_WRAM        = 0x03000000
CPU_WRAM_END    = CPU_WRAM + 0x8000
INTR_VECTOR_BUF = CPU_WRAM_END - 0x4
INTR_CHECK_BUF  = CPU_WRAM_END - 0x8

REG_BASE = 0x04000000
REG_IF  = REG_BASE + 0x202

V_BLANK_INTR_FLAG = 0x0001
STAT_V_BLANK_IF_ENABLE = 0x0008

.ARM
_start:
    b start_vector
    .space 188
start_vector:
    ldr r1, =INTR_VECTOR_BUF
    ldr r0, =intr_main
    str r0, [r1]

    /* call AgbMain in thumb mode */
    ldr r0, =AgbMain
    bx r0

    b start_vector

.ARM
intr_main:
    mov r1, #V_BLANK_INTR_FLAG

    ldr r0, =REG_IF
    strh r1, [r0]

    ldr r0, =VBlankInterrupt
    bx r0

    bx lr

.THUMB
.global vblankWait
vblankWait:
    swi 5
    bx lr
