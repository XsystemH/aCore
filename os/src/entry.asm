    .section .text.entry
    .globl _start
_start:
    la sp, _stack_top
    call rust_boot
    call rust_main

    .section .bss
    .globl _stack_top
_stack_top:
    .space 4096

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: