#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate ek_tm4c1294xl as board;

use board::hal::watchdog::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running WDOG Test");
    board::delay(1000);
    println!("Configuring Watchdog");
    let w = WATCHDOG0;
    w.sysctl_set_enabled(true);    
    while !w.sysctl_ready() {}

    // w.set_load(|_| Load(0x0500_0000));
    w
        .set_timeout(0x500_0000)
        .enable();
    // w.set_ctl(|r| r.set_resen(1));
    
    for i in 0..5 {
        // Refresh Watchdog
        println!("{} {}", i, w.value().value());
        // w.set_load(|_| Load(0x0500_0000));
        w.refresh();
        board::delay(500);
    }
    println!("Waiting for watchdog timeout...");

    loop {}
}
