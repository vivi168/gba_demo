_start:
    b header_end
    .space 188
header_end:
    ldr r0, =AgbMain
    bx r0
