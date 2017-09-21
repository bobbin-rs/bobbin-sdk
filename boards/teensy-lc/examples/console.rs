#![no_std]
#![no_main]

#[macro_use]
extern crate teensy_lc as board;

use board::hal::port::GpioPin;
use board::hal::gpio::GpioExt;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0.gpio_pin();
    led0.toggle_output();
    println!("Running Console");
    led0.toggle_output();
    let mut i = 0u32;
    loop {
        led0.toggle_output();
        println!("Hello, World! {}", i);
        i = i.wrapping_add(1);
        board::delay(1000);
    }
}
