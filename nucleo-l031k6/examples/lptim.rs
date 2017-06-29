#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l031k6 as board;

use board::hal::gpio::*;
use board::led::LED0;
use board::hal::lptim::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("LPTIM Test");


    // System Clock 32MHz
    // Prescaler 32_000
    // Timer Clock 1KHz
    // Reload = 1000
    // Overflow @ 1Hz

    let t = LPTIM;
    t.rcc_set_enabled(true);
    t.set_prescaler(36_000 - 1);
    t.set_auto_reload(2000 - 1);
    t.clr_update_interrupt_flag();
    t.set_enabled(true);
    loop {
        while !t.update_interrupt_flag() {}
        t.clr_update_interrupt_flag();
        LED0.toggle_output();
        println!("tick..");
    }
}

