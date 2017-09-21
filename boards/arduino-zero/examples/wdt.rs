#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate arduino_zero as board;

use board::hal::wdt::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running WDT Test");
    board::delay(1000);
    println!("Configuring Watchdog");
    let w = WDT;
    w.pm_set_enabled(true);    
    w.with_config(|r| r.set_per(0x8));
    w.with_ctrl(|r| r.set_enable(1));
    
    for i in 0..5 {
        // Refresh Watchdog
        println!("{}", i);
        w.set_clear(|r| r.set_clear(0xA5));
        board::delay(500);
    }
    println!("Waiting for watchdog timeout...");

    loop {}
}
