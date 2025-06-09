//! Constants used in rCore

pub const USER_STACK_SIZE: usize = 4096;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const MAX_APP_NUM: usize = 4;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;

/*
#[cfg(feature = "board_k210")]
pub const CLOCK_FREQ: usize = 403000000 / 62;

#[cfg(feature = "board_qemu")]
pub const CLOCK_FREQ: usize = 12500000;
*/
pub const CLOCK_FREQ: usize = 12500000;

pub const CLINT_BASE:     usize = 0x2000000;
pub const CLINT_SIZE: usize = 0x10000;  // 64KB
pub const CLINT_MTIMECMP: usize = CLINT_BASE + 0x4000; // hart 0, if single core
pub const CLINT_MTIME:    usize = CLINT_BASE + 0xBFF8;

pub const SCHED_PERIOD: usize = 1_000_000;
pub const CLINT: usize = 0x2000000;
pub const CPUS: usize = 4;