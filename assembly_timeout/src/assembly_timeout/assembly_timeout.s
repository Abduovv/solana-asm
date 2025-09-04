.equ NUM_ACCOUNTS,      0x00
.equ DATA_PTR_OFFSET,   0x10
.equ ACCOUNTS_PTR,      0x18
.equ SLOT_OFFSET,       0x00

.globl entrypoint
entrypoint:
    ldxdw   r0, [r1 + NUM_ACCOUNTS]
    jne     r0, 1, fail

    ldxdw   r2, [r1 + ACCOUNTS_PTR]
    ldxdw   r2, [r2 + 0x00]

    ldxdw   r3, [r2 + SLOT_OFFSET]

    ldxdw   r4, [r1 + DATA_PTR_OFFSET]
    ldxdw   r4, [r4 + 0x00]

    jle     r3, r4, success

fail:
    lddw    r0, 1
    exit

success:
    mov64   r0, 0
    exit
