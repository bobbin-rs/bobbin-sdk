#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate evb_s32k144 as board;

use board::chip::rcm::*;
use board::hal::wdog::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running WDOG Test");
    println!("SRS: {:?} SSRS: {:?}", RCM.srs(), RCM.ssrs());
    println!("CS: {:?}", WDOG.cs());
    board::delay(1000);

    // Unlock Watchdog
    WDOG.unlock();
    // WDOG.set_cnt(|_| Cnt(0xD928C520));
    // while WDOG.cs().ulk() == 0 {}
    WDOG.set_cs(|_| Cs(0x00002920).set_en(1).set_clk(0b01).set_pres(1));
    WDOG.set_timeout(1024);
    // WDOG.set_toval(|_| Toval(1024));
    // while WDOG.cs().rcs() == 0 {}
    

    println!("CS: {:?} TO: {} CNT: {:?}", WDOG.cs(), WDOG.toval(), WDOG.cnt());

    for i in 0..5 {
        // Refresh Watchdog
        println!("{} {} / {}", i, WDOG.counter(), WDOG.timeout());
        // WDOG.set_cnt(|_| Cnt(0xB480A602));
        WDOG.refresh();
        board::delay(500);
    }
    println!("Waiting for watchdog timeout...");

    loop {}
}
