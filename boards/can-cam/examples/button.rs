#![no_std]
#![no_main]

extern crate can_cam as board;

use board::hal::gpio::{DigitalOutput, DigitalInput};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0;
    let btn0 = board::btn::BTN0;
    loop {
        led0.toggle_output();
        if btn0.input() {
            board::delay(1000);
        } else {
            board::delay(100);
            
        }
    }
}
