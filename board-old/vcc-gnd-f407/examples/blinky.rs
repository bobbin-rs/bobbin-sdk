#![no_std]
#![no_main]

extern crate vcc_gnd_f407 as board;

use board::hal::gpio::{DigitalOutput};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0;
    loop {
        led0.toggle_output();
        board::delay(500);
    }
}
