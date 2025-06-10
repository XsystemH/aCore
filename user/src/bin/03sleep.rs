#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time, yield_};

#[unsafe(no_mangle)]
fn main() -> i32 {
    let current_timer = get_time();
    println!("Current time: {}", current_timer as u64);
    let wait_for = current_timer + 3000;
    while get_time() < wait_for {
        println!("Waiting");
        yield_();
    }
    println!("What the fuck?");
    0
}
