#![no_std]
#![no_main]

extern crate can_cam as board;

use board::led::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = LED0;
    let led1 = LED1;
    let led2 = LED2;
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
