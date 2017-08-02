#![no_std]
#![no_main]
#![feature(asm)]

extern crate evb_s32k144 as board;

use board::hal::port::GpioPin;
use board::hal::gpio::{DigitalInput, DigitalOutput};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let btn0 = board::btn::BTN0.gpio_pin();
    let led0 = board::led::LED0.gpio_pin();
    let led1 = board::led::LED1.gpio_pin();
    let led2 = board::led::LED2.gpio_pin();
    // LEDs are active LOW

    led0.set_output(true);
    led1.set_output(true);
    led2.set_output(true);

    loop {
        //led0.toggle_output();
        //led1.toggle_output();
        led2.toggle_output();
        if btn0.input() {
            board::delay(100);
        } else {
            board::delay(500);
        }
    }
}
