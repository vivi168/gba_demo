CPU_WRAM     = 0x03000000
CPU_WRAM_END = CPU_WRAM + 0x8000

REG_BASE = 0x04000000
REG_IF   = REG_BASE + 0x202

V_BLANK_INTR_FLAG = 0x0001
PSR_IRQ_MODE      = 0x12
PSR_SYS_MODE      = 0x1f

.ARM
_start:
    b start_vector
    .space 188
start_vector:
    mov r0, #PSR_IRQ_MODE
    msr cpsr, r0
    ldr sp, sp_irq

    mov r0, #PSR_SYS_MODE
    msr cpsr, r0
    ldr sp, sp_usr

    /* call AgbMain in thumb mode */
    ldr r0, =AgbMain
    bx r0

    b start_vector

sp_usr: .word CPU_WRAM_END - 0x100
sp_irq: .word CPU_WRAM_END - 0x60

.ARM
.global InterruptMain
InterruptMain:
    mov r1, #V_BLANK_INTR_FLAG

    ldr r0, =REG_IF
    strh r1, [r0]

    ldr r1, =InterruptTable
    ldr r0, [r1]
    bx r0

    bx lr

.THUMB
.global VBlankWait
VBlankWait:
    swi 5
    bx lr

.THUMB
.global CpuSet
CpuSet:
    swi 0x0b
    bx lr

.THUMB
.global CpuFastSet
CpuFastSet:
    swi 0x0c
    bx lr
