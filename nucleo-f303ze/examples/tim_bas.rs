#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f303ze as board;

use board::hal::gpio::*;
use board::led::LED0;
use board::hal::tim_bas::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("TIM_BAS Test");


    // System Clock 72MHz
    // Prescaler 36_000
    // Timer Clock 2KHz
    // Reload = 2000
    // Overflow @ 1Hz

    let t = TIM6;
    t.rcc_set_enabled(true);
    t.set_prescaler(36_000 - 1);
    t.set_reload(2000 - 1);
    t.clr_update_interrupt_flag();
    t.set_enabled(true);
    loop {
        while !t.update_interrupt_flag() {}
        t.clr_update_interrupt_flag();
        LED0.toggle_output();
        println!("tick..");
    }
}

