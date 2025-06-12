use core::ptr::{read_volatile, write_volatile};

// UART 寄存器的内存映射地址（在 QEMU virt 平台上）
use crate::config::UART_BASE;

// 寄存器偏移量
const RBR: usize = 0; // 接收缓冲寄存器 (读)
const THR: usize = 0; // 发送保持寄存器 (写)
const DLL: usize = 0;  // 除数锁存器 (低)
const DLM: usize = 1;  // 除数锁存器 (高)
const IER: usize = 1; // 中断使能寄存器
const FCR: usize = 2; // FIFO 控制寄存器
const LCR: usize = 3; // 线路控制寄存器
const MCR: usize = 4;  // Modem 控制寄存器
const LSR: usize = 5; // 线路状态寄存器

// LSR 状态位
const LSR_RX_READY: u8 = 1 << 0; // 数据准备就绪
const LSR_TX_IDLE: u8 = 1 << 5;  // 发送器空闲

pub fn init() {
    unsafe {
        // 禁用中断
        write_volatile((UART_BASE + IER) as *mut u8, 0x00);

        // 设置 DLAB 位以设置波特率
        write_volatile((UART_BASE + LCR) as *mut u8, 0x80);

        // 设置波特率为 38400
        // 38400 = 115200 / 3
        // 除数 = 22729000 / (16 * 38400) = 26
        write_volatile((UART_BASE + DLL) as *mut u8, 0x25); // 低8位
        write_volatile((UART_BASE + DLM) as *mut u8, 0x00); // 高8位

        // 8位数据位，1位停止位，无奇偶校验
        write_volatile((UART_BASE + LCR) as *mut u8, 0x03);

        // 启用 FIFO，清除接收和发送 FIFO，设置中断阈值为 14 字节
        write_volatile((UART_BASE + FCR) as *mut u8, 0xC7);

        // 启用接收数据可用中断
        write_volatile((UART_BASE + MCR) as *mut u8, 0x03);
    }
}

pub fn putchar(c: usize) {
    unsafe {
        // wait till the transmitter is idle
        while (read_volatile((UART_BASE + LSR) as *const u8) & LSR_TX_IDLE) == 0 {
            // wait
        }

        write_volatile((UART_BASE + THR) as *mut u8, c as u8);
    }
}

pub fn getchar() -> u8 {
    unsafe {
        // wait till data is available
        while (read_volatile((UART_BASE + LSR) as *const u8) & LSR_RX_READY) == 0 {
            // wait
        }
        read_volatile((UART_BASE + RBR) as *const u8)
    }
}