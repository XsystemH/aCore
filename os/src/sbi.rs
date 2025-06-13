#![allow(unused)]

use crate::config::TEST_DEVICE_ADDR;

pub fn console_putchar(c: usize) {
    crate::uart::putchar(c);
}

pub fn console_getchar() -> usize {
    // #[allow(deprecated)]
    crate::uart::getchar() as usize
}

/// use sbi call to shutdown the kernel
pub fn shutdown(failure: bool) -> ! {
    const SHUTDOWN_CODE: u32 = 0x5555;  // 正常关机
    const FAILURE_CODE: u32 = 0x3333;   // 错误关机
    println!("[kernel] Shutdown");
    unsafe {
        if !failure {
            // 正常关机
            core::ptr::write_volatile(TEST_DEVICE_ADDR as *mut u32, SHUTDOWN_CODE);
        } else {
            // 错误关机
            core::ptr::write_volatile(TEST_DEVICE_ADDR as *mut u32, FAILURE_CODE);
        }
    }
    unreachable!()
}
