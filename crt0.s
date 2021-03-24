.ARM

_start:
    b header_end
    .space 188
header_end:
    bl registerVblank

    /* call AgbMain in thumb mode */
    ldr r0, =AgbMain
    bx r0

.ARM
interruptHandler:
    /* only vblank interrupt is enabled */
    /* so it's pretty straight forward */
    /* acknowledge REG_IF (4000202) */
    mov r1, #0x4000000
    mov r2, #1
    str r2, [r1, #0x202]

    /* acknowledge REG_IFBIOS (4000202) */
    /* to be able to use swi 5 */
    str r2, [r1, #-8]

    /* ldr r3, [r1, #-8] */
    /* ldr r2, [r3] */
    /* orr r2, #1 */
    /* str r2, [r1, #-8] */

    bx lr

.ARM
.global registerVblank
registerVblank:
    /* enable v-blank interrupt */
    mov r1, #0x4000000

    /* REG_IME (4000208) = 0, disable all interrupts */
    mov r2, #0
    str r2, [r1, #0x208]

    /* REG_INTERRUPT (3007FFC) = interruptHandler */
    ldr r2, =interruptHandler
    str r2, [r1, #-4]

    /* REG_DIPSTAT (4000004) |= 8 (4th bit -> enable vblank) */
    ldr r2, [r1, #4]
    orr r2, #8
    str r2, [r1, #4]

    /* REG_IE (4000200) |= 1 (1st bit -> enable vblank) */
    ldr r2, [r1, #0x200]
    orr r2, #1
    str r2, [r1, #0x200]

    bx lr

.THUMB
.global vblankWait
vblankWait:
    swi 5
