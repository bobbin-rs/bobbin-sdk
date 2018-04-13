#![no_std]
#![no_main]

#[macro_use]
extern crate blue_pill as board;

use board::hal::gpio::*;
use board::led::LED0;
use board::hal::tim_adv::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("TIM_ADV Test");


    // System Clock 72MHz
    // Prescaler 36_000
    // Timer Clock 2KHz
    // Reload = 2000
    // Overflow @ 1Hz

    let t = TIM1;
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

