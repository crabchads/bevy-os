.global _start
_start:
    ldr x30, =stack_top
    mov sp, x30
    bl kernel_main
    b .

.equ PSCI_SYSTEM_OFF, 0x84000008
.globl system_off
system_off:
    ldr x0, =PSCI_SYSTEM_OFF
    hvc #0