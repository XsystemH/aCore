//! Constants used in rCore

pub const UART_BASE: usize = 0x10000000;
pub const UART_SIZE: usize = 0x100;
pub const USER_STACK_SIZE: usize = 4096;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;
pub const MAX_APP_NUM: usize = 4;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;

pub const PAGE_SIZE: usize = 1 << PAGE_SIZE_BITS; // 4KB
pub const PAGE_SIZE_BITS: usize = 0xc;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;
/*
#[cfg(feature = "board_k210")]
pub const CLOCK_FREQ: usize = 403000000 / 62;

#[cfg(feature = "board_qemu")]
pub const CLOCK_FREQ: usize = 12500000;
*/

pub const CLINT_BASE:     usize = 0x2000000;
pub const CLINT_SIZE: usize = 0x10000;  // 64KB
pub const CLINT_MTIMECMP: usize = CLINT_BASE + 0x4000; // hart 0, if single core
pub const CLINT_MTIME:    usize = CLINT_BASE + 0xBFF8;
pub const TEST_DEVICE_ADDR: usize = 0x100000;
pub const SCHED_PERIOD: usize = 1_000_000;
pub const CLINT: usize = 0x2000000;
pub const CPUS: usize = 4;

pub use crate::board::{CLOCK_FREQ, MEMORY_END, MMIO};

/// Return (bottom, top) of a kernel stack in kernel space.
pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}