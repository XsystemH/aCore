    .section .text.entry
    .globl _start
    .align 2
_start:
    la sp, boot_stack_top
    j rust_boot

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: