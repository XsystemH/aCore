use core::arch::global_asm;
use crate::config::{CLINT_MTIME, CLINT_MTIMECMP, CLOCK_FREQ, SCHED_PERIOD};
use riscv::register::{mie, mscratch, mstatus, mtvec};

const TICKS_PER_SEC: usize = 500; // interrupt frequency
const MSEC_PER_SEC: usize = 1000;
const TIME_INTERVAL: usize = CLOCK_FREQ / TICKS_PER_SEC; // timer interval in seconds

global_asm!(include_str!("m_trap.S"));

/// read the `mtime` register
pub fn get_time() -> usize {
    // time::read()
    unsafe {
        let timer = CLINT_MTIME as *const usize;
        timer.read_volatile()
    }
}

/// get current time in milliseconds
pub fn get_time_ms() -> usize {
    // time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
    get_time() / (CLOCK_FREQ / MSEC_PER_SEC)
}

pub fn set_timer(time: usize) {
    unsafe {
        let timer = CLINT_MTIMECMP as *mut usize;
        *timer = time;
    }
}

/// set the next timer interrupt
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

#[unsafe(link_section = ".bss.stack")]
#[unsafe(no_mangle)]
pub static mut TIMER_SCRATCH: [usize; 5] = [0; 5];

#[unsafe(no_mangle)]
pub unsafe fn init_timer() {
    set_timer(get_time() + SCHED_PERIOD);

    let mscratch_ptr = unsafe { core::ptr::addr_of!(TIMER_SCRATCH) as usize };
    mscratch::write(mscratch_ptr);

    // set the machine-mode trap handler
    unsafe extern "C" {
        fn _timertrap();
    }

    TIMER_SCRATCH[3] = CLINT_MTIMECMP;
    TIMER_SCRATCH[4] = TIME_INTERVAL;

    mtvec::write(_timertrap as usize, mtvec::TrapMode::Direct);

    // enable machine-mode interrupts
    mstatus::set_mie();

    // enable machine-mode timer interrupts
    mie::set_mtimer();

    // println!("[kernel] timer initialized");
}