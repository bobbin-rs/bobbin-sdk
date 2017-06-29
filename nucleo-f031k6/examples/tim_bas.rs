#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f031k6 as board;

use board::hal::gpio::*;
use board::led::LED0;
use board::tim_bas::TIM8;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("TIM_BAS Test");

    let t = TIM8;

    loop {
        LED0.toggle_output();
        board::delay(500);
    }
}

