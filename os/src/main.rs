//! The main module and entrypoint
//!
//! Various facilities of the kernels are implemented as submodules. The most
//! important ones are:
//!
//! - [`trap`]: Handles all cases of switching from userspace to the kernel
//! - [`task`]: Task management
//! - [`syscall`]: System call handling and implementation
//!
//! The operating system also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality. (See its source code for
//! details.)
//!
//! We then call [`task::run_first_task()`] and for the first time go to
//! userspace.
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::arch::{asm, global_asm};
use log::*;
use riscv::register::*;

#[path = "boards/qemu.rs"]
mod board;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;
mod uart;
mod sync;
mod trap;
mod syscall;
mod loader;
mod config;
mod task;
mod timer;
mod mm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// clear BSS segment
pub fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    println!("[kernel] Clearing BSS segment from {:x} to {:x}", sbss as usize, ebss as usize);
    (sbss as usize..ebss as usize).for_each(|a| unsafe {
        (a as *mut u8).write_volatile(0)
    });
    println!("[kernel] BSS segment cleared");
}

// from M mode to S mode
#[unsafe(no_mangle)]
unsafe fn rust_boot() {
    // M mode

    // 设置 mstatus 寄存器及 mepc 寄存器
    mstatus::set_mpp(mstatus::MPP::Supervisor);
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

    asm!(
    "csrw mideleg, {mideleg}",
    "csrw medeleg, {medeleg}",
    "mret",
    medeleg = in(reg) !0,
    mideleg = in(reg) !0,
    options(noreturn),
    );
}

/// the rust entry-point of os
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    trap::enable_timer_interrupt();
    clear_bss();
    uart::init();
    logging::init();
    println!("[kernel] Hello, world!");
    mm::init();
    println!("[Kernel] Memory management initialized");
    mm::remap_test();
    trap::init();
    // loader::load_apps();
    println!("[kernel] Apps Loaded");
    timer::set_next_trigger();
    println!("[Kernel] Set first time interrupt");
    // batch::run_next_app();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}
