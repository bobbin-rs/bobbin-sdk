#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate frdm_k64f as board;

use board::chip::rcm::*;
use board::hal::wdog::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running WDOG Test");
    println!("SRS0: {:?} SRS1: {:?}", RCM.srs0(), RCM.srs1());
    board::delay(1000);

    WDOG.configure(Config {
        source: ClockSource::Lpo,
        prescaler: Prescaler::Div1,
        timeout: 2048,
        window: 0,
    });

    println!("STCTRLH: {:?}", WDOG.stctrlh());

    for i in 0..5 {
        //WDOG.refresh();
        use core::ptr;
        unsafe {
            ptr::write_volatile(0x4005_200c as *mut u16, 0xA602);
            ptr::write_volatile(0x4005_200c as *mut u16, 0xB480);
        }
        println!("{} {}", i, WDOG.timer());
        board::delay(500);
    }
    println!("Waiting for watchdog timeout...");

    loop {}
}
