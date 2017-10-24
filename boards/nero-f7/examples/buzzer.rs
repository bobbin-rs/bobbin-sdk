#![no_std]
#![no_main]

extern crate nero_f7 as board;

use board::hal::gpio::{DigitalInput, DigitalOutput};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let buz0 = board::buz::BUZ0;
    let led0 = board::led::LED0;
    loop {
        led0.toggle_output();
        buz0.toggle_output();
        board::delay(500);
    }
}
