#![no_std]
#![no_main]

extern crate teensy_35 as board;

use board::hal::port::GpioPin;
use board::hal::gpio::GpioExt;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    let led0 = board::led::LED0.gpio_pin();
    loop {
        led0.toggle_output();
        board::delay(500);
    }
}
