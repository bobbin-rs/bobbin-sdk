#![no_std]
#![no_main]

extern crate can_cam as board;

use board::hal::gpio::{DigitalOutput};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0;
    let led1 = board::led::LED1;
    let led2 = board::led::LED2;
    loop {
        led0.toggle_output();
        board::delay(500);
        led0.toggle_output();
        led1.toggle_output();
        board::delay(500);
        led1.toggle_output();
        led2.toggle_output();
        board::delay(500);
        led2.toggle_output();
    }
}
