#![no_std]
#![no_main]

extern crate ek_tm4c1294xl as board;

use board::hal::gpio::{DigitalInput, DigitalOutput};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0;
    let led1 = board::led::LED1;
    let btn0 = board::btn::BTN0;
    led0.toggle_output();
    loop {
        led0.toggle_output();
        led1.toggle_output();
        if btn0.input() {
            board::delay(500);
        } else {
            board::delay(100);
        }
    }
}
