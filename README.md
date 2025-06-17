# ACore

## Mine vs Standard

| Standard      | Mine           |
|---------------|----------------|
| BootLoader    | ✅              |
| Allocator     | ✅(no [SLAB]()) |
| PageTable     | ✅              |
| Console       | ✅              |
| M&D           | ✅              |
| Process       | ✅              |
| [Syn]()       | ❌              |
| [FS]()        | ❌              |
| [MultiCore]() | ❌              |
| [Driver]()    | ❌              |

## Bootloader

### Initialize:

```
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
```

### M -> S

Refer from ACore Guidance.

> 注：从 M mode 到 S mode 的几乎所有操作都是与硬件直接相关的操作，抽象等级很低，不值得深究。

```Rust
unsafe fn rust_boot() {
    // M mode
    // 设置 mstatus 寄存器及 mepc 寄存器
    mstatus::set_mpp(mstatus::MPP::Supervisor);
    // mepc 寄存器设置为 rust_main 的入口地址
    mepc::write(rust_main as usize);
    // 暂时禁用页表
    satp::write(0);
    // 设置 PMP 允许全物理访问
    pmpaddr0::write(0x3fffffffffffffusize);
    pmpcfg0::write(0xf);
    // init timer in M mode
    timer::init_timer();
    // 全委托给 S-mode
    mideleg::set_stimer();
    mideleg::set_sext();
    mideleg::set_ssoft();
    // 全委托给 S-mode
    asm!(
    "csrw mideleg, {mideleg}",
    "csrw medeleg, {medeleg}",
    "mret", // 返回mepc寄存器中设置的地址, 即 rust_main
    medeleg = in(reg) !0,
    mideleg = in(reg) !0,
    options(noreturn),
    );
}
```

## Allocator

### Buddy Allocator

```Rust
use buddy_system_allocator::LockedHeap;
```

### Frame Allocator

In file `src/mm/frame_allocator.rs`


